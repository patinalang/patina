use std::path::Path;

use libtest_mimic::Failed;
use similar::TextDiff;

pub fn check_required(path: &Path, actual: &str, bless: bool) -> Result<(), Failed> {
    if bless {
        write_snapshot(path, actual);
        return Ok(());
    }
    check(path, actual)
}

pub fn check_optional(path: &Path, actual: &str, bless: bool) -> Result<(), Failed> {
    if bless {
        if actual.is_empty() {
            remove_snapshot(path);
        } else {
            write_snapshot(path, actual);
        }
        return Ok(());
    }

    if actual.is_empty() && path.exists() {
        Err(Failed::from(format!(
            "unexpected stderr snapshot: {}\n\
                 compiler produced no stderr; delete `stderr` or re-bless",
            path.display(),
        )))
    } else if actual.is_empty() {
        Ok(())
    } else {
        check(path, actual)
    }
}

fn remove_snapshot(path: &Path) {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
    .unwrap_or_else(|_| panic!("remove {}", path.display()));
}

fn write_snapshot(snapshot_path: &Path, actual: &str) {
    if let Some(parent) = snapshot_path.parent() {
        std::fs::create_dir_all(parent)
            .unwrap_or_else(|_| panic!("create dir {}", parent.display()));
    }

    std::fs::write(snapshot_path, actual)
        .unwrap_or_else(|_| panic!("write {}", snapshot_path.display()));
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
        other => other.unwrap_or_else(|_| panic!("read {}", snapshot_path.display())),
    };

    if expected == actual {
        return Ok(());
    }

    let diff = TextDiff::from_lines(expected.as_str(), actual);
    let rendered = diff.unified_diff().context_radius(3).header("expected", "actual").to_string();

    Err(Failed::from(format!("snapshot mismatch: {}\n{}", snapshot_path.display(), rendered)))
}
