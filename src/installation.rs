use std::path::PathBuf;
use std::{fs, io};

use crate::package;

const BOOTSTRAP_EXPECTED_LIB_FILES: [&str; 1] = ["patina/ebin/patina.app"];
const EXPECTED_LIB_FILES: [&str; 2] =
    ["patina_core/ebin/libCore.rmeta", "patina_core/ebin/patina_core.app"];

#[derive(Debug, thiserror::Error)]
pub(crate) enum LocateInstallationError {
    #[error("error while searching for the Patina install: {0}")]
    Io(#[from] io::Error),
    #[error("failed to find the Patina install")]
    NotFound,
    #[error("Patina install corrupted: {0} is missing")]
    Corrupted(&'static str),
}

#[derive(Debug, Clone)]
pub(crate) struct Installation {
    pub(crate) lib_dirs: Vec<PathBuf>,
}

impl Installation {
    /// Reads the install at `root`.
    /// If `root` is `None`, we search in `./lib` or `../lib`, relative to the `patina` binary.
    /// Every subdirectory of the found `lib` dir ends up as a search directory.
    pub(crate) fn locate(root: Option<PathBuf>) -> Result<Installation, LocateInstallationError> {
        let root = match root {
            Some(root) => root,
            None => {
                let exe = std::env::current_exe()?.canonicalize()?;
                match exe.ancestors().skip(1).take(2).find(|dir| dir.join("lib").is_dir()) {
                    Some(root) => root.to_owned(),
                    None => return Err(LocateInstallationError::NotFound),
                }
            }
        };

        let lib = root.join("lib");

        for path in BOOTSTRAP_EXPECTED_LIB_FILES {
            if !lib.join(path).is_file() {
                return Err(LocateInstallationError::Corrupted(path));
            }
        }

        if !package::is_bootstrap() {
            for path in EXPECTED_LIB_FILES {
                if !lib.join(path).is_file() {
                    return Err(LocateInstallationError::Corrupted(path));
                }
            }
        }

        let mut lib_dirs = Vec::new();
        for entry in fs::read_dir(lib)? {
            let ebin = entry?.path().join("ebin");
            if ebin.is_dir() {
                lib_dirs.push(ebin);
            }
        }
        lib_dirs.sort();

        Ok(Installation { lib_dirs })
    }
}
