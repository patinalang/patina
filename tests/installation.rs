mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use common::{Session, assert_failure, assert_success};

/// `patina compile` of a trivial `source.pt`, in `dir`.
fn compile_trivial(dir: &Path) -> Command {
    std::fs::write(dir.join("source.pt"), "pub fn f() {}\n").expect("write source");
    let mut cmd = common::patina_command();
    cmd.current_dir(dir).args(["compile", "source.pt"]);
    cmd
}

fn run(cmd: &mut Command) -> Output {
    cmd.output().expect("run patina")
}

#[test]
fn reserved_package_name_needs_bootstrap() {
    let temp = tempfile::tempdir().expect("create temp dir");

    let output = run(compile_trivial(temp.path()).args(["--pkg-name", "std"]));
    assert_failure(&output, "package name `std` is reserved");

    let output =
        run(compile_trivial(temp.path()).args(["--pkg-name", "std"]).env("PATINA_BOOTSTRAP", "1"));
    assert_success(&output);
}

#[test]
fn install_missing_compile_server_is_reported() {
    let root = tempfile::tempdir().expect("create temp dir");
    let output = run(compile_trivial(root.path()).env("PATINA_ROOT", root.path()));
    let missing = "patina_core/ebin/patina_compile_server.beam is missing";
    assert_failure(&output, &format!("Patina install corrupted: {missing}"));
}

#[test]
fn install_without_core_needs_a_named_core() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let root = install_without_core(temp.path());

    let output = run(compile_trivial(temp.path()).env("PATINA_ROOT", &root));
    assert_failure(&output, "can't find crate for `core`");

    let output = run(compile_trivial(temp.path())
        .env("PATINA_ROOT", &root)
        .args(["--extern", &installed_core()]));
    assert_success(&output);
}

/// Otherwise the build would end up with two Cores.
#[test]
fn named_core_hides_installed_core_from_dependencies() {
    let session = Session::new();
    session.build("provider.pt", "pub fn f() {}\n", &[]);
    let other_core = build_stand_in_core(session.path());

    let provider = "provider=target/debug/provider/ebin/libprovider.rmeta";
    let args = ["--extern", &other_core, "--extern", provider];
    let output = session.try_build("consumer.pt", "pub fn g() { provider::f() }\n", &args);
    assert_failure(&output, "can't find crate for `core` which `provider` depends on");
}

/// The VM logs this failure as a crash report too, which mustn't garble the reply.
#[test]
fn server_start_failure_is_reported() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let too_long_for_a_socket_path = temp.path().join("x".repeat(150));
    std::fs::create_dir(&too_long_for_a_socket_path).expect("create temp dir");

    let output = run(compile_trivial(temp.path()).env("TMPDIR", &too_long_for_a_socket_path));
    assert_failure(&output, "failed to start compile server: error:");
}

#[test]
fn non_ascii_socket_path_reaches_the_client() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let tmpdir = tempfile::Builder::new().prefix("ż").tempdir().expect("create temp dir");
    for locale in ["C", "en_US.UTF-8"] {
        let output =
            run(compile_trivial(temp.path()).env("TMPDIR", tmpdir.path()).env("LC_ALL", locale));
        assert_success(&output);
    }
}

/// `--extern` naming the install's Core.
fn installed_core() -> String {
    format!("core={}", common::install_root().join("patina_core/ebin/libcore.rmeta").display())
}

/// Copies the install into `dir`, without Core, and returns its root.
fn install_without_core(dir: &Path) -> PathBuf {
    let ebin = dir.join("root/patina_core/ebin");
    std::fs::create_dir_all(&ebin).expect("create dir");
    for module in ["patina", "patina_compile_server", "patina_error"] {
        let file = format!("{module}.beam");
        std::fs::copy(common::install_root().join("patina_core/ebin").join(&file), ebin.join(file))
            .expect("copy file");
    }
    dir.join("root")
}

/// Builds a Core with nothing in it but an empty prelude, in `dir`, and returns its `--extern`.
fn build_stand_in_core(dir: &Path) -> String {
    let source = "#![feature(no_core)]\n#![no_std]\n#![no_core]\npub mod prelude {}\n";
    std::fs::write(dir.join("core.pt"), source).expect("write source");
    let output = run(common::patina_command()
        .current_dir(dir)
        .args(["compile", "--pkg-name", "core", "--emit", "meta", "core.pt"])
        .env("PATINA_BOOTSTRAP", "1"));
    assert_success(&output);
    format!("core={}", dir.join("target/debug/core/ebin/libcore.rmeta").display())
}
