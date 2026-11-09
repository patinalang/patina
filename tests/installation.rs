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

    let output = run(compile_trivial(temp.path()).args(["--pkg-name", "Std"]));
    assert_failure(&output, "package name `Std` is reserved");

    let output =
        run(compile_trivial(temp.path()).args(["--pkg-name", "Std"]).env("PATINA_BOOTSTRAP", "1"));
    assert_success(&output);
}

#[test]
fn install_missing_compiler_app_is_reported() {
    let root = tempfile::tempdir().expect("create temp dir");
    let output = run(compile_trivial(root.path()).env("PATINA_ROOT", root.path()));
    assert_failure(&output, "Patina install corrupted: patina/ebin/patina.app is missing");
}

#[test]
fn install_without_core_needs_a_named_core() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let root = install_without_core(temp.path());

    let output = run(compile_trivial(temp.path()).env("PATINA_ROOT", &root));
    assert_failure(&output, "can't find crate for `Core`");

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

    let args = ["--extern", &other_core, "--extern", "Provider=libProvider.rmeta"];
    let output = session.try_build("consumer.pt", "pub fn g() { Provider::f() }\n", &args);
    assert_failure(&output, "can't find crate for `Core` which `Provider` depends on");
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
    format!("Core={}", common::install_root().join("patina_core/ebin/libCore.rmeta").display())
}

/// Copies the install into `dir`, without Core, and returns its root.
fn install_without_core(dir: &Path) -> PathBuf {
    let ebin = dir.join("root/patina/ebin");
    std::fs::create_dir_all(&ebin).expect("create dir");
    for entry in std::fs::read_dir(common::install_root().join("patina/ebin")).expect("read dir") {
        let file = entry.expect("read dir entry").path();
        std::fs::copy(&file, ebin.join(file.file_name().expect("file name"))).expect("copy file");
    }
    dir.join("root")
}

/// Builds a Core with nothing in it but an empty prelude, in `dir`, and returns its `--extern`.
fn build_stand_in_core(dir: &Path) -> String {
    let source = "#![feature(no_core)]\n#![no_std]\n#![no_core]\npub mod Prelude {}\n";
    std::fs::write(dir.join("core.pt"), source).expect("write source");
    let output = run(common::patina_command()
        .current_dir(dir)
        .args(["compile", "--pkg-name", "Core", "--emit", "meta", "core.pt"])
        .env("PATINA_BOOTSTRAP", "1"));
    assert_success(&output);
    format!("Core={}", dir.join("libCore.rmeta").display())
}
