use std::cell::OnceCell;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::installation::{LazyInstallation, LocateInstallationError};
use crate::manifest::{DependencyInfo, Manifest, ManifestError};
use crate::package::PackageInfo;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum Format {
    /// Prints the information as JSON. Every requested item gets its own top-level key.
    Json,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum Print {
    /// Prints package's information, as in `Patina.toml` `[package]` section.
    Package,
    /// Prints package's dependencies, as in `Patina.toml` `[dependencies]` section.
    Dependencies,
    /// Prints the compiler's version.
    Version,
    /// Prints the absolute path of the Patina install.
    #[value(name = "install_root")]
    InstallRoot,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PrintError {
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error(transparent)]
    LocateInstallation(#[from] LocateInstallationError),
    #[error("failed to resolve the install root {path}: {source}")]
    ResolveInstallRoot {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to write the output: {0}")]
    Write(#[from] serde_json::Error),
}

#[derive(Default, Serialize)]
struct Output {
    #[serde(skip_serializing_if = "Option::is_none")]
    package: Option<PackageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<DependencyInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_root: Option<PathBuf>,
}

struct LazyManifest<'a> {
    package_dir: &'a Path,
    manifest: OnceCell<Manifest>,
}

impl<'a> LazyManifest<'a> {
    fn new(package_dir: &'a Path) -> Self {
        Self { package_dir, manifest: OnceCell::new() }
    }

    fn get(&self) -> Result<&Manifest, ManifestError> {
        match self.manifest.get() {
            Some(manifest) => Ok(manifest),
            None => {
                let manifest = Manifest::read(self.package_dir)?;
                Ok(self.manifest.get_or_init(|| manifest))
            }
        }
    }
}

pub(crate) fn print(
    format: Format,
    prints: &[Print],
    package_dir: &Path,
    installation: LazyInstallation,
) -> Result<(), PrintError> {
    let Format::Json = format;

    let manifest = LazyManifest::new(package_dir);
    let mut output = Output::default();

    for print in prints {
        match print {
            Print::Package => output.package = Some(manifest.get()?.package()),
            Print::Dependencies => output.dependencies = Some(manifest.get()?.dependencies()),
            Print::Version => output.version = Some(env!("CARGO_PKG_VERSION")),
            Print::InstallRoot => {
                let path = &installation.get()?.root;
                let root = std::path::absolute(path).map_err(|source| {
                    PrintError::ResolveInstallRoot { path: path.clone(), source }
                })?;
                output.install_root = Some(root);
            }
        };
    }

    let mut stdout = io::stdout().lock();
    serde_json::to_writer(&mut stdout, &output)?;
    writeln!(stdout).map_err(serde_json::Error::io)?;
    Ok(())
}
