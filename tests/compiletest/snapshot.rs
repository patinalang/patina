use std::path::Path;

use libtest_mimic::Failed;
use similar::TextDiff;

pub fn check_pair(
    stdout_path: &Path,
    stdout: &str,
    stderr_path: &Path,
    stderr: &str,
    bless: bool,
) -> Result<(), Failed> {
    if bless {
        write_snapshot(stdout_path, stdout);
        if stderr.is_empty() {
            remove_snapshot(stderr_path);
        } else {
            write_snapshot(stderr_path, stderr);
        }
        return Ok(());
    }

    check(stdout_path, stdout)?;

    if stderr.is_empty() && stderr_path.exists() {
        Err(Failed::from(format!(
            "unexpected stderr snapshot: {}\n\
                 compiler produced no stderr; delete the file or re-bless",
            stderr_path.display(),
        )))
    } else if stderr.is_empty() {
        Ok(())
    } else {
        check(stderr_path, stderr)
    }
}

fn remove_snapshot(path: &Path) {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
    .expect(&format!("remove {}", path.display()));
}

fn write_snapshot(snapshot_path: &Path, actual: &str) {
    if let Some(parent) = snapshot_path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|e| panic!("create dir {}: {e}", parent.display()));
    }

    std::fs::write(snapshot_path, actual)
        .unwrap_or_else(|e| panic!("write {}: {e}", snapshot_path.display()));
}

fn check(snapshot_path: &Path, actual: &str) -> Result<(), Failed> {
    let expected = match std::fs::read_to_string(snapshot_path) {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(Failed::from(format!(
                "snapshot missing: {}\n\
                 run with `cargo test --test compiletest -- --bless` to create it",
                snapshot_path.display(),
            )));
        }
        Err(e) => panic!("read {}: {e}", snapshot_path.display()),
    };

    if expected == actual {
        return Ok(());
    }

    let diff = TextDiff::from_lines(expected.as_str(), actual);
    let rendered = diff.unified_diff().context_radius(3).header("expected", "actual").to_string();

    Err(Failed::from(format!("snapshot mismatch: {}\n{}", snapshot_path.display(), rendered)))
}
