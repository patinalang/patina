use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};

use tempfile::TempDir;

const CONSUMER: &str = "pub fn f() { let _x = Provider::num!(); }\n";

fn provider(value: u32) -> String {
    format!(
        r#"
use Core::ProcMacro::*;

#[proc_macro]
pub fn num(_input: TokenStream) -> TokenStream {{
    TokenStream::from_trees([TokenTree::Literal(Literal::int({value}))])
}}
"#
    )
}

/// Builds a proc macro, uses it, rebuilds it in the same VM, and uses it again.
fn assert_rebuild_is_picked_up(first_out: &str, second_out: &str) {
    let session = Session::new();

    for (out, value) in [(first_out, 42), (second_out, 43)] {
        session.build("provider.pt", &provider(value), &["--out-dir", out]);

        let args = ["--dump", "expanded", "-L", out, "--extern", "Provider"];
        session.build("consumer.pt", CONSUMER, &args);

        let expanded = std::fs::read_to_string(session.dir.path().join("Consumer.expanded"))
            .expect("read expanded");
        assert!(expanded.contains(&format!("let _x = {value};")), "got:\n{expanded}");
    }
}

#[test]
fn rebuilt_proc_macro_is_reloaded_in_a_running_vm() {
    assert_rebuild_is_picked_up("out", "out");
}

#[test]
fn proc_macro_rebuilt_elsewhere_is_reloaded_in_a_running_vm() {
    assert_rebuild_is_picked_up("first_out", "second_out");
}

/// A `patina serve`, and a directory to compile in that isn't the server's working directory.
struct Session {
    dir: TempDir,
    socket: String,
    /// Stops once this is dropped, as that closes its stdin.
    _serve: Child,
}

impl Session {
    fn new() -> Session {
        let mut serve = Command::new(env!("CARGO_BIN_EXE_patina"))
            .arg("serve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("spawn compile server");
        let mut socket = String::new();
        BufReader::new(serve.stdout.take().expect("stdout is piped"))
            .read_line(&mut socket)
            .expect("read socket path");
        socket.truncate(socket.trim_end().len());
        let dir = tempfile::tempdir().expect("create temp dir");
        Session { dir, socket, _serve: serve }
    }

    fn build(&self, file: &str, source: &str, args: &[&str]) {
        std::fs::write(self.dir.path().join(file), source).expect("write source");
        let output = Command::new(env!("CARGO_BIN_EXE_patina"))
            .current_dir(self.dir.path())
            .args(["compile", "--compile-server", &self.socket])
            .args(args)
            .arg(file)
            .output()
            .expect("run patina");
        assert!(output.status.success(), "got:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// A running server keeps the code path from earlier compiles, which mustn't shadow this one's.
#[test]
fn later_compile_code_path_wins_in_a_running_vm() {
    let session = Session::new();
    session.build("provider.pt", &provider(42), &["--out-dir", "earlier"]);
    session.build("provider.pt", &provider(43), &["--out-dir", "later"]);

    // Puts `earlier` on the code path, without running the macro and so loading its module.
    session.build("unused.pt", "pub fn f() {}\n", &["-L", "earlier", "--extern", "Provider"]);

    let args = ["--dump", "expanded", "-L", "later", "--extern", "Provider"];
    session.build("consumer.pt", CONSUMER, &args);

    let expanded = std::fs::read_to_string(session.dir.path().join("Consumer.expanded"))
        .expect("read expanded");
    assert!(expanded.contains("let _x = 43;"), "got:\n{expanded}");
}

#[test]
fn relative_extern_path_reaches_the_compile_server() {
    let session = Session::new();
    session.build("provider.pt", &provider(42), &["--out-dir", "out"]);
    session.build("consumer.pt", CONSUMER, &["--extern", "Provider=out/libProvider.rmeta"]);
}
