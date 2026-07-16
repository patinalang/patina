use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::time::{Duration, Instant};
use std::{fs, thread};

use rustc_errors::DiagCtxtHandle;

const SERVER_BEAM: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/patina_compile_server.beam"));

const OP_COMPILE: &[u8] = b"compile";
const OP_ABSTR: &[u8] = b"abstr";
const OP_ERL: &[u8] = b"erl";

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const CONNECT_RETRY: Duration = Duration::from_millis(5);

const ERL_INIT: &str = "\
[BeamPath, SockPath] = init:get_plain_arguments(), \
{module, patina_compile_server} = code:load_abs(BeamPath), \
patina_compile_server:start_link(SockPath)";

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
    pub(self) fn new() -> io::Result<TmpDir> {
        let path = std::env::temp_dir().join(format!("patina-compile-{}", std::process::id()));
        fs::create_dir_all(&path)?;
        Ok(TmpDir { path })
    }

    pub fn path(&self) -> &Path {
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

pub(super) struct CompileServer<'a> {
    dcx: DiagCtxtHandle<'a>,
    #[allow(unused)]
    tmpdir: TmpDir,
    child: Child,
    stream: UnixStream,
}

impl<'a> CompileServer<'a> {
    pub(super) fn start(dcx: DiagCtxtHandle<'a>) -> CompileServer<'a> {
        let tmpdir = TmpDir::new().unwrap_or_else(|e| {
            dcx.fatal(format!("failed to create compile server temp dir: {e}"));
        });

        let beam_path = tmpdir.path().join("patina_compile_server.beam");
        fs::write(&beam_path, SERVER_BEAM).unwrap_or_else(|e| {
            dcx.fatal(format!("failed to write compile server beam: {e}"));
        });

        let sock_path = tmpdir.path().join("srv.sock");

        let child = Command::new("erl")
            .args([
                "-noshell",
                "-eval",
                ERL_INIT,
                "-extra",
                &beam_path.with_extension("").to_string_lossy(),
                &sock_path.to_string_lossy(),
            ])
            .spawn()
            .unwrap_or_else(|e| {
                dcx.fatal(format!("failed to spawn erl compile server: {e}"));
            });

        let stream = connect_with_retry(dcx, &sock_path);

        CompileServer { dcx, tmpdir, child, stream }
    }

    pub(super) fn compile(&mut self, eaf: &[u8]) -> Result<CompileOk, CompileErr> {
        match self.call(OP_COMPILE, eaf).as_slice() {
            [status, beam, warnings] if status == b"ok" => {
                Ok(CompileOk { beam: beam.to_vec(), warnings: self.part_str(warnings) })
            }
            [status, errors, warnings] if status == b"error" => {
                Err(CompileErr { errors: self.part_str(errors), warnings: self.part_str(warnings) })
            }
            parts => self.unexpected(parts),
        }
    }

    pub(super) fn render_abstr(&mut self, eaf: &[u8]) -> String {
        self.render(OP_ABSTR, eaf)
    }

    pub(super) fn render_erl(&mut self, eaf: &[u8]) -> String {
        self.render(OP_ERL, eaf)
    }

    fn render(&mut self, op: &[u8], eaf: &[u8]) -> String {
        match self.call(op, eaf).as_slice() {
            [status, render] if status == b"ok" => self.part_str(render),
            parts => self.unexpected(parts),
        }
    }

    /// Send a request and read the response, handling server-side exceptions.
    fn call(&mut self, op: &[u8], eaf: &[u8]) -> Vec<Vec<u8>> {
        write_msg(&mut self.stream, &[op, eaf])
            .unwrap_or_else(|e| self.dcx.fatal(format!("failed to write to compile server: {e}")));
        let parts = read_msg(&mut self.stream)
            .unwrap_or_else(|e| self.dcx.fatal(format!("failed to read from compile server: {e}")));
        if parts.first().map(Vec::as_slice) == Some(b"raise") {
            let class = parts.get(1).map_or_else(|| "?".into(), |p| self.part_str(p));
            let reason = parts.get(2).map_or_else(|| "?".into(), |p| self.part_str(p));
            let stacktrace = parts.get(3).map_or_else(|| "?".into(), |p| self.part_str(p));
            self.dcx.fatal(format!("compile server raised {class}: {reason}\n{stacktrace}"));
        }
        parts
    }

    fn part_str(&self, part: &[u8]) -> String {
        String::from_utf8(part.to_vec()).unwrap_or_else(|e| {
            self.dcx.fatal(format!("compile server returned invalid UTF-8: {e}"));
        })
    }

    fn unexpected(&self, parts: &[Vec<u8>]) -> ! {
        let parts = parts.iter().map(|p| String::from_utf8_lossy(p)).collect::<Vec<_>>().join(", ");
        self.dcx.fatal(format!("unexpected compile server response: {parts}"));
    }
}

impl Drop for CompileServer<'_> {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
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

fn connect_with_retry(dcx: DiagCtxtHandle<'_>, sock_path: &Path) -> UnixStream {
    let deadline = Instant::now() + CONNECT_TIMEOUT;
    loop {
        match UnixStream::connect(sock_path) {
            Ok(stream) => return stream,
            Err(_) if Instant::now() < deadline => thread::sleep(CONNECT_RETRY),
            Err(e) => dcx.fatal(format!(
                "failed to connect to compile server at {}: {e}",
                sock_path.display()
            )),
        }
    }
}
