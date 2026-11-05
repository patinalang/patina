use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

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

        let mut line = String::new();
        let stdout = serve.stdout.take().expect("stdout is piped");
        if BufReader::new(stdout).read_line(&mut line)? == 0 {
            let status = serve.wait()?;
            return Err(io::Error::other(format!(
                "`patina serve` exited before reporting its socket ({status})"
            )));
        }
        let sock_path = PathBuf::from(line.trim_end_matches('\n'));

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
