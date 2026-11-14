//! Helpers shared by the integration tests.
//!
//! Every test binary compiles this module separately and uses only a part of it.
#![allow(dead_code)]

use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};

use tempfile::TempDir;

pub(crate) fn patina_command() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_patina"));
    cmd.env("PATINA_ROOT", install_root());
    cmd
}

#[cfg(unix)]
fn path_from_bytes(bytes: Vec<u8>) -> PathBuf {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;
    PathBuf::from(OsString::from_vec(bytes))
}

#[cfg(windows)]
fn path_from_bytes(bytes: Vec<u8>) -> PathBuf {
    PathBuf::from(String::from_utf8(bytes).expect("socket path is UTF-8"))
}

/// A running `patina serve`, which stops when this is dropped.
pub(crate) struct CompileServer {
    serve: Child,
    sock_path: PathBuf,
}

impl CompileServer {
    pub(crate) fn start() -> io::Result<CompileServer> {
        let mut serve =
            patina_command().arg("serve").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

        let mut line = Vec::new();
        let stdout = serve.stdout.take().expect("stdout is piped");
        if BufReader::new(stdout).read_until(b'\n', &mut line)? == 0 {
            let status = serve.wait()?;
            return Err(io::Error::other(format!(
                "`patina serve` exited before reporting its socket ({status})"
            )));
        }
        line.pop_if(|byte| *byte == b'\n');
        let sock_path = path_from_bytes(line);

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
        patina_command()
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
        assert_success(&self.try_build(file, source, args));
    }

    /// Writes the package `name`, with `dependencies` as its `[dependencies]` table, to the
    /// directory `name` in the session's directory.
    pub(crate) fn package(&self, name: &str, dependencies: &str, source: &str) {
        let dir = self.path().join(name);
        std::fs::create_dir_all(dir.join("src")).expect("create package dir");
        let manifest = format!(
            "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n\n[dependencies]\n{dependencies}\n"
        );
        std::fs::write(dir.join("Patina.toml"), manifest).expect("write manifest");
        let file = format!("{name}.pt");
        std::fs::write(dir.join("src").join(file), source).expect("write source");
    }

    /// `patina build` of the package in `dir`, in the session's directory.
    pub(crate) fn try_build_package(&self, dir: &str, args: &[&str]) -> Output {
        patina_command()
            .current_dir(self.path())
            .arg("build")
            .arg("--compile-server")
            .arg(self.server.sock_path())
            .args(args)
            .arg("-C")
            .arg(dir)
            .output()
            .expect("run patina")
    }

    /// Like [`Session::try_build_package`], but the build has to succeed.
    #[track_caller]
    pub(crate) fn build_package(&self, dir: &str, args: &[&str]) {
        assert_success(&self.try_build_package(dir, args));
    }

    /// Writes the package `name` (see [`Session::package`]) and builds it, which has to succeed.
    #[track_caller]
    pub(crate) fn build_new_package(
        &self,
        name: &str,
        dependencies: &str,
        source: &str,
        args: &[&str],
    ) {
        self.package(name, dependencies, source);
        self.build_package(name, args);
    }
}

#[track_caller]
pub(crate) fn assert_success(output: &Output) {
    assert!(output.status.success(), "got:\n{}", String::from_utf8_lossy(&output.stderr));
}

#[track_caller]
pub(crate) fn assert_failure(output: &Output, message: &str) {
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(message), "got:\n{stderr}");
}

pub(crate) fn install_root() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_patina")).with_file_name("root")
}

/// Source of a package with a proc macro, `num!()`, that expands to `value`.
pub(crate) fn provider(value: u32) -> String {
    format!(
        r#"
use core::proc_macro::*;

#[proc_macro]
pub fn num(_input: TokenStream) -> TokenStream {{
    TokenStream::from_trees([TokenTree::Literal(Literal::int({value}))])
}}
"#
    )
}

/// Source of a package that uses [`provider`]'s `num!()`.
pub(crate) const CONSUMER: &str = "pub fn f() { let _x = provider::num!(); }\n";
