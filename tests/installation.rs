mod common;

use std::path::Path;
use std::process::{Command, Output};

use common::Session;

/// Runs `patina <args> source.pt` in `dir`, with a trivial `source.pt`.
fn patina(dir: &Path, args: &[&str], envs: &[(&str, &Path)]) -> Output {
    std::fs::write(dir.join("source.pt"), "pub fn f() {}\n").expect("write source");
    Command::new(env!("CARGO_BIN_EXE_patina"))
        .current_dir(dir)
        .args(args)
        .arg("source.pt")
        .envs(envs.iter().copied())
        .output()
        .expect("run patina")
}

#[track_caller]
fn assert_success(output: &Output) {
    assert!(output.status.success(), "got:\n{}", String::from_utf8_lossy(&output.stderr));
}

#[track_caller]
fn assert_failure(output: &Output, message: &str) {
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(message), "got:\n{stderr}");
}

#[test]
fn reserved_package_name_needs_bootstrap() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let args = ["compile", "--pkg-name", "Std"];

    let output = patina(temp.path(), &args, &[]);
    assert_failure(&output, "package name `Std` is reserved");

    let output = patina(temp.path(), &args, &[("PATINA_BOOTSTRAP", "1".as_ref())]);
    assert_success(&output);
}

#[test]
fn install_missing_any_required_file_is_reported() {
    const INSTALL_FILES: [&str; 3] = [
        "patina/ebin/patina.app",
        "patina_core/ebin/libCore.rmeta",
        "patina_core/ebin/patina_core.app",
    ];

    for missing in INSTALL_FILES {
        let root = tempfile::tempdir().expect("create temp dir");
        for file in INSTALL_FILES.into_iter().filter(|file| *file != missing) {
            let path = root.path().join("lib").join(file);
            std::fs::create_dir_all(path.parent().expect("file's directory")).expect("create dir");
            std::fs::write(path, b"").expect("write file");
        }

        let output = patina(root.path(), &["compile"], &[("PATINA_ROOT", root.path())]);
        assert_failure(&output, &format!("Patina install corrupted: {missing} is missing"));
    }
}

/// The VM logs this failure as a crash report too, which mustn't garble the reply.
#[test]
fn server_start_failure_is_reported() {
    let temp = tempfile::tempdir().expect("create temp dir");
    // Too long for a socket path.
    let tmpdir = temp.path().join("x".repeat(150));
    std::fs::create_dir(&tmpdir).expect("create overlong temp dir");

    let output = patina(temp.path(), &["compile"], &[("TMPDIR", &tmpdir)]);
    assert_failure(&output, "failed to start compile server: error:");
}

#[test]
fn non_ascii_socket_path_reaches_the_client() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let tmpdir = tempfile::Builder::new().prefix("ż").tempdir().expect("create temp dir");
    for locale in ["C", "en_US.UTF-8"] {
        let envs = [("TMPDIR", tmpdir.path()), ("LC_ALL", locale.as_ref())];
        assert_success(&patina(temp.path(), &["compile"], &envs));
    }
}

/// Macros run against the Core named by `--extern`, not the installed one, so a broken module in
/// the named Core breaks them.
#[test]
fn core_override_replaces_installed_core_at_macro_time() {
    let real_core = Path::new(env!("CARGO_BIN_EXE_patina")).with_file_name("lib/patina_core/ebin");
    let session = Session::new();

    let core = session.path().join("core");
    std::fs::create_dir(&core).expect("create core dir");
    std::fs::copy(real_core.join("libCore.rmeta"), core.join("libCore.rmeta"))
        .expect("copy core metadata");
    std::fs::write(core.join("Elixir.Patina.Core.ProcMacro.Literal.beam"), b"")
        .expect("write broken module");

    let provider = r#"
use Core::ProcMacro::*;

#[proc_macro]
pub fn forty_two(_input: TokenStream) -> TokenStream {
    TokenStream::from_trees([TokenTree::Literal(Literal::int(42))])
}
"#;
    session.build("provider.pt", provider, &[]);

    let consumer = "pub fn f() { let _x = Provider::forty_two!(); }\n";
    let core_extern = format!("Core={}", core.join("libCore.rmeta").display());
    let args = ["--extern", &core_extern, "--extern", "Provider=libProvider.rmeta"];
    let output = session.try_build("consumer.pt", consumer, &args);
    assert_failure(&output, "message: error: undef");
}
