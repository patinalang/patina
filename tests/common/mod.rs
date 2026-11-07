//! Helpers shared by the integration tests.
//!
//! Every test binary compiles this module separately and uses only a part of it.
#![allow(dead_code)]

use std::ffi::OsString;
use std::io::{self, BufRead, BufReader};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};

use tempfile::TempDir;

/// A running `patina serve`, which stops when this is dropped.
pub(crate) struct CompileServer {
    serve: Child,
    sock_path: PathBuf,
}

impl CompileServer {
    pub(crate) fn start() -> io::Result<CompileServer> {
        let mut serve = Command::new(env!("CARGO_BIN_EXE_patina"))
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut line = Vec::new();
        let stdout = serve.stdout.take().expect("stdout is piped");
        if BufReader::new(stdout).read_until(b'\n', &mut line)? == 0 {
            let status = serve.wait()?;
            return Err(io::Error::other(format!(
                "`patina serve` exited before reporting its socket ({status})"
            )));
        }
        line.pop_if(|byte| *byte == b'\n');
        let sock_path = PathBuf::from(OsString::from_vec(line));

        Ok(CompileServer { serve, sock_path })
    }

    pub(crate) fn sock_path(&self) -> &Path {
        &self.sock_path
    }
}

impl Drop for CompileServer {
    fn drop(&mut self) {
        drop(self.serve.stdin.take());
        let _ = self.serve.wait();
    }
}

/// A compile server, and a directory to compile in that isn't the server's working directory.
pub(crate) struct Session {
    dir: TempDir,
    server: CompileServer,
}

impl Session {
    pub(crate) fn new() -> Session {
        let server = CompileServer::start().expect("start compile server");
        let dir = tempfile::tempdir().expect("create temp dir");
        Session { dir, server }
    }

    pub(crate) fn path(&self) -> &Path {
        self.dir.path()
    }

    /// Writes `source` to `file` in the session's directory, and compiles it.
    pub(crate) fn try_build(&self, file: &str, source: &str, args: &[&str]) -> Output {
        std::fs::write(self.path().join(file), source).expect("write source");
        Command::new(env!("CARGO_BIN_EXE_patina"))
            .current_dir(self.path())
            .arg("compile")
            .arg("--compile-server")
            .arg(self.server.sock_path())
            .args(args)
            .arg(file)
            .output()
            .expect("run patina")
    }

    /// Like [`Session::try_build`], but the compilation has to succeed.
    #[track_caller]
    pub(crate) fn build(&self, file: &str, source: &str, args: &[&str]) {
        let output = self.try_build(file, source, args);
        assert!(output.status.success(), "got:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

pub(crate) fn install_root() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_patina")).with_file_name("lib")
}
