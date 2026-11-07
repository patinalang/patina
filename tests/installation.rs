use std::path::Path;
use std::process::{Command, Output};

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
