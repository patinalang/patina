use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::Shutdown;
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{fs, mem, thread};

use rustc_data_structures::marker::IntoDynSyncSend;
use rustc_errors::DiagCtxtHandle;
use rustc_expand::proc_macro::{BeamMacroRunner, MacroRunError};

const SERVER_BEAM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/patina_compile_server.beam"));

const OP_COMPILE: &[u8] = b"compile";
const OP_RENDER_TERM: &[u8] = b"render_term";
const OP_ERL: &[u8] = b"erl";
const OP_CORE0: &[u8] = b"core0";
const OP_CORE: &[u8] = b"core";
const OP_CODE_PATH: &[u8] = b"code_path";
const OP_MACRO: &[u8] = b"macro";
#[cfg(test)]
const OP_ECHO: &[u8] = b"echo";

const CONNECT_TIMEOUT: Duration = Duration::from_secs(60);

/// How long to wait for a reply to a macro invocation.
/// There's a shorter timeout on the `patina_compile_server` side.
const MACRO_REPLY_TIMEOUT: Duration = Duration::from_secs(120);

const ERL_INIT: &str = "\
spawn(fun() -> io:get_line([]), erlang:halt(0) end), \
[BeamPath] = init:get_plain_arguments(), \
{module, patina_compile_server} = code:load_abs(BeamPath), \
{ok, _} = patina_compile_server:ensure_started(), \
io:format(\"~ts~n\", [patina_compile_server:socket_path()])";

pub(super) struct CompileOk {
    pub beam: Vec<u8>,
    pub warnings: String,
}

pub(super) struct CompileErr {
    pub errors: String,
    pub warnings: String,
}

struct TmpDir {
    path: PathBuf,
}

impl TmpDir {
    fn new() -> io::Result<TmpDir> {
        let path = std::env::temp_dir().join(format!("patina-{}", std::process::id()));

        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
        fs::create_dir(&path)?;

        Ok(TmpDir { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        if let Err(e) = fs::remove_dir_all(&self.path) {
            eprintln!("failed to remove compile server temp dir `{}`: {e}", self.path.display());
        }
    }
}

/// Why a compile server could not be started.
#[derive(Debug, thiserror::Error)]
pub(crate) enum SpawnError {
    #[error("failed to create temp dir: {0}")]
    TmpDir(#[source] io::Error),
    #[error("failed to write compile server beam: {0}")]
    WriteBeam(#[source] io::Error),
    #[error("failed to spawn erl compile server: {0}")]
    Erl(#[source] io::Error),
    #[error("failed to connect to compile server: {0}")]
    Connect(#[source] io::Error),
}

/// Why there is no VM to talk to, kept for every caller that asks later.
#[derive(Debug, thiserror::Error)]
pub(crate) enum StartupError {
    #[error("failed to start compile server: {0}")]
    Spawn(#[from] SpawnError),
    #[error("the compile server's startup thread panicked")]
    Panicked,
    #[error("the compile server's startup was lost")]
    Lost,
}

pub(crate) struct CompileServer {
    _vm: BeamVm,
    stream: UnixStream,
}

enum BeamVm {
    /// A VM we started, which goes away with us.
    Own {
        /// Removed on drop, after the VM is reaped.
        _tmpdir: TmpDir,
        child: Child,
    },
    /// A VM that was already running, which we only connected to.
    Shared,
}

pub(crate) struct SyncServer {
    state: IntoDynSyncSend<Mutex<State>>,
}

enum State {
    Starting(JoinHandle<Result<CompileServer, SpawnError>>),
    Ready(CompileServer),
    Failed(StartupError),
}

impl CompileServer {
    fn connect(sock_path: &Path) -> Result<CompileServer, SpawnError> {
        let stream = UnixStream::connect(sock_path).map_err(SpawnError::Connect)?;
        Ok(CompileServer { _vm: BeamVm::Shared, stream })
    }

    fn spawn() -> Result<CompileServer, SpawnError> {
        let (vm, sock_path) = BeamVm::start()?;
        let stream = UnixStream::connect(sock_path).map_err(SpawnError::Connect)?;
        Ok(CompileServer { _vm: vm, stream })
    }

    pub(super) fn compile(
        &mut self,
        dcx: DiagCtxtHandle<'_>,
        eaf: &[u8],
    ) -> Result<CompileOk, CompileErr> {
        let parts = self.call(dcx, &[OP_COMPILE, eaf]);
        match parts.as_slice() {
            [status, beam, warnings] if status == b"ok" => {
                Ok(CompileOk { beam: beam.to_vec(), warnings: self.part_str(dcx, warnings) })
            }
            [status, errors, warnings] if status == b"error" => Err(CompileErr {
                errors: self.part_str(dcx, errors),
                warnings: self.part_str(dcx, warnings),
            }),
            parts => self.unexpected(dcx, parts),
        }
    }

    pub(super) fn render_term(&mut self, dcx: DiagCtxtHandle<'_>, term: &[u8]) -> String {
        self.render(dcx, OP_RENDER_TERM, term)
    }

    pub(super) fn render_erl(&mut self, dcx: DiagCtxtHandle<'_>, eaf: &[u8]) -> String {
        self.render(dcx, OP_ERL, eaf)
    }

    pub(super) fn render_core0(
        &mut self,
        dcx: DiagCtxtHandle<'_>,
        eaf: &[u8],
    ) -> Result<String, CompileErr> {
        self.compile_and_render(dcx, OP_CORE0, eaf)
    }

    pub(super) fn render_core(
        &mut self,
        dcx: DiagCtxtHandle<'_>,
        eaf: &[u8],
    ) -> Result<String, CompileErr> {
        self.compile_and_render(dcx, OP_CORE, eaf)
    }

    fn compile_and_render(
        &mut self,
        dcx: DiagCtxtHandle<'_>,
        op: &[u8],
        eaf: &[u8],
    ) -> Result<String, CompileErr> {
        let parts = self.call(dcx, &[op, eaf]);
        match parts.as_slice() {
            [status, render] if status == b"ok" => Ok(self.part_str(dcx, render)),
            [status, errors, warnings] if status == b"error" => Err(CompileErr {
                errors: self.part_str(dcx, errors),
                warnings: self.part_str(dcx, warnings),
            }),
            parts => self.unexpected(dcx, parts),
        }
    }

    fn render(&mut self, dcx: DiagCtxtHandle<'_>, op: &[u8], eaf: &[u8]) -> String {
        let parts = self.call(dcx, &[op, eaf]);
        match parts.as_slice() {
            [status, render] if status == b"ok" => self.part_str(dcx, render),
            parts => self.unexpected(dcx, parts),
        }
    }

    pub(super) fn extend_code_path(
        &mut self,
        dcx: DiagCtxtHandle<'_>,
        dirs: impl IntoIterator<Item = PathBuf>,
    ) {
        let dirs = dirs
            .into_iter()
            .map(|dir| dir.to_string_lossy().into_owned().into_bytes())
            .collect::<Vec<_>>();

        let request: Box<[&[u8]]> =
            std::iter::once(OP_CODE_PATH).chain(dirs.iter().map(Vec::as_slice)).collect();

        let parts = self.call(dcx, &request);
        match parts.as_slice() {
            [status] if status == b"ok" => {}
            parts => self.unexpected(dcx, parts),
        }
    }

    pub(super) fn call_macro(
        &mut self,
        module: &str,
        function: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>, MacroRunError> {
        let request: Box<[&[u8]]> = [OP_MACRO, module.as_bytes(), function.as_bytes()]
            .into_iter()
            .chain(args.iter().map(Vec::as_slice))
            .collect();

        let parts = self
            .transact_within(&request, Some(MACRO_REPLY_TIMEOUT))
            .map_err(|e| MacroRunError::Transport(format!("compile server I/O failed: {e}")))?;

        match parts.as_slice() {
            [status, result] if status == b"ok" => Ok(result.clone()),
            [status, class, reason, stack] if status == b"error" => {
                let class = String::from_utf8_lossy(class);
                let reason = String::from_utf8_lossy(reason);
                let stack = String::from_utf8_lossy(stack);
                Err(MacroRunError::Raised(format!("{class}: {reason}\n{stack}")))
            }
            [status, reason] if status == b"down" => Err(MacroRunError::Raised(format!(
                "the proc macro's process died: {}",
                String::from_utf8_lossy(reason)
            ))),
            [status] if status == b"timeout" => Err(MacroRunError::Timeout),
            parts => Err(MacroRunError::Transport(format!(
                "unexpected compile server response: {}",
                parts.iter().map(|p| String::from_utf8_lossy(p)).collect::<Vec<_>>().join(", ")
            ))),
        }
    }

    #[cfg(test)]
    pub(crate) fn echo(&mut self, dcx: DiagCtxtHandle<'_>, bytes: &[u8]) -> Vec<u8> {
        let parts = self.call(dcx, &[OP_ECHO, bytes]);
        match parts.as_slice() {
            [status, result] if status == b"ok" => result.clone(),
            parts => self.unexpected(dcx, parts),
        }
    }

    /// Send a request and read the response, handling server-side exceptions.
    fn call(&mut self, dcx: DiagCtxtHandle<'_>, request: &[&[u8]]) -> Vec<Vec<u8>> {
        let parts = match self.transact_within(request, None) {
            Ok(parts) => parts,
            Err(e) => dcx.fatal(format!("compile server communication failed: {e}")),
        };
        if parts.first().map(Vec::as_slice) == Some(b"raise") {
            let class = parts.get(1).map_or_else(|| "?".into(), |p| self.part_str(dcx, p));
            let reason = parts.get(2).map_or_else(|| "?".into(), |p| self.part_str(dcx, p));
            let stacktrace = parts.get(3).map_or_else(|| "?".into(), |p| self.part_str(dcx, p));
            dcx.fatal(format!("compile server raised {class}: {reason}\n{stacktrace}"));
        }
        parts
    }

    /// Send a request and read the reply, giving up after `timeout` if given.
    fn transact_within(
        &mut self,
        parts: &[&[u8]],
        timeout: Option<Duration>,
    ) -> io::Result<Vec<Vec<u8>>> {
        let result = self.exchange(parts, timeout);
        if result.is_err() {
            // Any failure leaves the stream positioned mid-frame
            let _ = self.stream.shutdown(Shutdown::Both);
        }
        result
    }

    fn exchange(&mut self, parts: &[&[u8]], timeout: Option<Duration>) -> io::Result<Vec<Vec<u8>>> {
        write_msg(&mut self.stream, parts)?;

        if timeout.is_none() {
            return read_msg(&mut self.stream);
        }

        self.stream.set_read_timeout(timeout)?;
        let result = read_msg(&mut self.stream);
        let restored = self.stream.set_read_timeout(None);
        match result {
            Ok(parts) => restored.map(|()| parts),
            Err(e) if matches!(e.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut) => {
                Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!(
                        "compile server stopped responding after {}s",
                        timeout.expect("timeout is set in this branch").as_secs()
                    ),
                ))
            }
            Err(e) => Err(e),
        }
    }

    fn part_str(&self, dcx: DiagCtxtHandle<'_>, part: &[u8]) -> String {
        String::from_utf8(part.to_vec()).unwrap_or_else(|e| {
            dcx.fatal(format!("compile server returned invalid UTF-8: {e}"));
        })
    }

    fn unexpected(&self, dcx: DiagCtxtHandle<'_>, parts: &[Vec<u8>]) -> ! {
        let parts = parts.iter().map(|p| String::from_utf8_lossy(p)).collect::<Vec<_>>().join(", ");
        dcx.fatal(format!("unexpected compile server response: {parts}"));
    }
}

impl BeamVm {
    fn start() -> Result<(BeamVm, PathBuf), SpawnError> {
        let tmpdir = TmpDir::new().map_err(SpawnError::TmpDir)?;

        let beam_path = tmpdir.path().join("patina_compile_server.beam");
        fs::write(&beam_path, SERVER_BEAM).map_err(SpawnError::WriteBeam)?;

        let mut child = Command::new("erl")
            .args([
                "+pc",
                "unicode",
                "-noshell",
                "-eval",
                ERL_INIT,
                "-extra",
                &beam_path.with_extension("").to_string_lossy(),
            ])
            .env("TMPDIR", tmpdir.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(SpawnError::Erl)?;

        let sock_path = start_stdout_handler(child.stdout.take().expect("stdout is piped"));
        forward_output_to_stderr(child.stderr.take());
        let vm = BeamVm::Own { _tmpdir: tmpdir, child };

        let sock_path = match sock_path.recv_timeout(CONNECT_TIMEOUT) {
            Ok(sock_path) => sock_path,
            Err(RecvTimeoutError::Timeout) => Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "compile server didn't report its socket in time",
            )),
            Err(RecvTimeoutError::Disconnected) => {
                Err(io::Error::other("compile server's stdout handler panicked"))
            }
        };
        let sock_path = sock_path.map_err(SpawnError::Connect)?;
        Ok((vm, sock_path))
    }
}

/// Runs a compile server for other patinas to connect to, until our stdin closes. Prints the
/// server's socket path as the first line on stdout.
pub(crate) fn serve() -> Result<(), SpawnError> {
    let (vm, sock_path) = BeamVm::start()?;
    println!("{}", sock_path.display());
    // Any error means there's no stdin to wait on anymore, so stop all the same.
    let _ = io::copy(&mut io::stdin().lock(), &mut io::sink());
    drop(vm);
    Ok(())
}

impl Drop for BeamVm {
    fn drop(&mut self) {
        match self {
            BeamVm::Own { child, .. } => {
                drop(child.stdin.take());
                let _ = child.kill();
                let _ = child.wait();
            }
            BeamVm::Shared => {}
        }
    }
}

/// Drain one of the VM's output pipes onto our own stderr.
fn forward_output_to_stderr<R: Read + Send + 'static>(pipe: Option<R>) {
    let Some(mut pipe) = pipe else { return };
    thread::spawn(move || {
        let _ = io::copy(&mut pipe, &mut io::stderr());
    });
}

impl BeamMacroRunner for SyncServer {
    fn call(
        &self,
        module: &str,
        function: &str,
        args: &[Vec<u8>],
    ) -> Result<Vec<u8>, MacroRunError> {
        self.try_with(|server| match server {
            Ok(server) => server.call_macro(module, function, args),
            Err(error) => Err(MacroRunError::Transport(error.to_string())),
        })
    }
}

#[derive(Debug, Default)]
pub(crate) struct SyncServerOpts {
    pub(crate) sock_path: Option<PathBuf>,
}

impl SyncServer {
    pub(crate) fn start(opts: SyncServerOpts) -> Arc<SyncServer> {
        let startup = State::Starting(thread::spawn(move || match opts.sock_path {
            Some(sock_path) => CompileServer::connect(&sock_path),
            None => CompileServer::spawn(),
        }));
        Arc::new(SyncServer { state: IntoDynSyncSend(Mutex::new(startup)) })
    }

    pub(super) fn with<R>(
        &self,
        dcx: DiagCtxtHandle<'_>,
        f: impl FnOnce(&mut CompileServer) -> R,
    ) -> R {
        self.try_with(|server| match server {
            Ok(server) => f(server),
            Err(error) => dcx.fatal(error.to_string()),
        })
    }

    /// Like [`SyncServer::with`], but hands `f` the startup failure instead of
    /// raising it. Used by the proc-macro runner, which reports errors through
    /// rustc's macro machinery.
    pub(crate) fn try_with<R>(
        &self,
        f: impl FnOnce(Result<&mut CompileServer, &StartupError>) -> R,
    ) -> R {
        let mut state = self.state.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        f(state.ready())
    }
}

impl State {
    fn ready(&mut self) -> Result<&mut CompileServer, &StartupError> {
        match self {
            State::Starting(_) => {
                let lost = State::Failed(StartupError::Lost);
                let State::Starting(startup) = mem::replace(self, lost) else {
                    unreachable!("just matched")
                };
                *self = match startup.join() {
                    Ok(Ok(server)) => State::Ready(server),
                    Ok(Err(err)) => State::Failed(err.into()),
                    Err(_) => State::Failed(StartupError::Panicked),
                };
                self.ready()
            }
            State::Ready(server) => Ok(server),
            State::Failed(err) => Err(err),
        }
    }
}

impl Drop for SyncServer {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(|poisoned| poisoned.into_inner());
        let _ = state.ready();
    }
}

/// Write a length-prefixed message: a one-byte part count, then each part as a
/// big-endian `u32` length followed by its bytes.
fn write_msg(stream: &mut UnixStream, parts: &[&[u8]]) -> io::Result<()> {
    let count = u8::try_from(parts.len())
        .map_err(|_| invalid(format!("too many message parts: {}", parts.len())))?;
    stream.write_all(&[count])?;
    for part in parts {
        let len = u32::try_from(part.len())
            .map_err(|_| invalid(format!("message part too large: {} bytes", part.len())))?;
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(part)?;
    }
    Ok(())
}

/// Read a message framed as written by [`write_msg`].
fn read_msg(stream: &mut UnixStream) -> io::Result<Vec<Vec<u8>>> {
    let mut count = [0u8; 1];
    stream.read_exact(&mut count)?;

    let mut parts = Vec::with_capacity(count[0] as usize);
    for _ in 0..count[0] {
        let mut len = [0u8; 4];
        stream.read_exact(&mut len)?;
        let mut data = vec![0u8; u32::from_be_bytes(len) as usize];
        stream.read_exact(&mut data)?;
        parts.push(data);
    }
    Ok(parts)
}

fn invalid(msg: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, msg)
}

/// Starts a thread that reads our VM's stdout: first the socket path the VM prints as its first
/// line, which it sends on the returned channel, then the rest, drained onto our stderr.
/// The server listens on the socket before the VM prints it.
fn start_stdout_handler(stdout: ChildStdout) -> mpsc::Receiver<io::Result<PathBuf>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();
        let sock_path = match reader.read_line(&mut line) {
            Ok(0) => Err(io::Error::other("compile server exited before reporting its socket")),
            Ok(_) => Ok(PathBuf::from(line.trim_end_matches('\n'))),
            Err(e) => Err(e),
        };
        let _ = tx.send(sock_path);
        let _ = io::copy(&mut reader, &mut io::stderr());
    });
    rx
}

#[cfg(test)]
mod etf_tests;
