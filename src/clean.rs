use std::io;
use std::path::{Path, PathBuf};

use crate::compile;

#[derive(Debug, thiserror::Error)]
#[error("failed to remove {path}: {source}")]
pub(crate) struct CleanError {
    path: PathBuf,
    #[source]
    source: io::Error,
}

/// Removes the `out_dir` files (as recorded in the `<incremental_dir>/<pkg>/files`)
/// and the package's `incremental_dir`.
pub(crate) fn clean(out_dir: &Path, incremental_dir: &Path) -> Result<(), CleanError> {
    let files_list = compile::files_list_path(incremental_dir);

    let files = match std::fs::read_to_string(&files_list) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => String::new(),
        other => other.map_err(|source| CleanError { path: files_list.clone(), source })?,
    };

    for file in files.lines() {
        let path = out_dir.join(file);
        match std::fs::remove_file(&path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            other => other.map_err(|source| CleanError { path, source })?,
        }
    }

    match std::fs::remove_dir_all(incremental_dir) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        other => other.map_err(|source| CleanError { path: incremental_dir.to_owned(), source })?,
    }

    Ok(())
}
