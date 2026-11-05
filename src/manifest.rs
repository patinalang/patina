//! `Patina.toml`, the manifest of a package directory.

use std::io;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::package::snake_case;

pub(crate) const MANIFEST_FILE: &str = "Patina.toml";

#[derive(Debug, thiserror::Error)]
pub(crate) enum ManifestError {
    #[error("failed to read {}: {source}", path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("invalid {}: {source}", path.display())]
    Parse {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Manifest {
    pub package: PackageManifest,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PackageManifest {
    pub name: String,
}

impl Manifest {
    pub(crate) fn read(dir: &Path) -> Result<Manifest, ManifestError> {
        let path = dir.join(MANIFEST_FILE);
        let text = match std::fs::read_to_string(&path) {
            Ok(text) => text,
            Err(source) => return Err(ManifestError::Read { path, source }),
        };
        toml::from_str(&text).map_err(|source| ManifestError::Parse { path, source })
    }

    pub(crate) fn source_path(&self, dir: &Path) -> PathBuf {
        dir.join("src").join(format!("{}.pt", snake_case(&self.package.name)))
    }
}
