use std::env::consts::EXE_SUFFIX;
use std::io;
use std::path::{Path, PathBuf};

const EXPECTED_FILES: [&str; 1] = ["patina/ebin/patina.app"];

#[derive(Debug, thiserror::Error)]
pub(crate) enum LocateInstallationError {
    #[error("error while searching for the Patina install: {0}")]
    Io(#[from] io::Error),
    #[error(
        "failed to find the Patina install: {exe_path} isn't at <root>/patina/bin/patina; \
         pass --root or set PATINA_ROOT"
    )]
    NotFound { exe_path: PathBuf },
    #[error("Patina install corrupted: {0} is missing")]
    Corrupted(&'static str),
}

#[derive(Debug, Clone)]
pub(crate) struct Installation {
    /// `<root>/patina/ebin`
    pub(crate) patina_dir: PathBuf,
    /// `<root>/patina_core/ebin`
    pub(crate) core_dir: PathBuf,
}

impl Installation {
    /// Reads the install at `root`.
    /// If `root` is `None`, we assume this executable is in the install.
    pub(crate) fn locate(root: Option<PathBuf>) -> Result<Installation, LocateInstallationError> {
        let root = match root {
            Some(root) => root,
            None => {
                let exe_path = std::env::current_exe()?;
                #[cfg(unix)]
                let exe_path = exe_path.canonicalize()?;
                match root_of_exe(&exe_path) {
                    Some(root) => root.to_owned(),
                    None => return Err(LocateInstallationError::NotFound { exe_path }),
                }
            }
        };

        for path in EXPECTED_FILES {
            if !root.join(path).is_file() {
                return Err(LocateInstallationError::Corrupted(path));
            }
        }

        Ok(Installation {
            patina_dir: root.join("patina/ebin"),
            core_dir: root.join("patina_core/ebin"),
        })
    }
}

fn root_of_exe(exe: &Path) -> Option<&Path> {
    if exe.file_name()? != format!("patina{EXE_SUFFIX}").as_str() {
        return None;
    }

    let bin = exe.parent()?;
    if bin.file_name()? != "bin" {
        return None;
    }

    let app = bin.parent()?;
    if app.file_name()? != "patina" {
        return None;
    }

    app.parent()
}
