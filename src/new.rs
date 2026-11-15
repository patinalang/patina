use std::io;
use std::path::{Path, PathBuf};

use crate::manifest::MANIFEST_FILE;
use crate::package::{InvalidPackageName, PackageName};

const MANIFEST: &str = include_str!("../lib/templates/Patina.toml");
const ROOT_FILE: &str = include_str!("../lib/templates/root.pt");
const REBAR_CONFIG_SCRIPT: &str = include_str!("../lib/templates/rebar.config.script");
const APP_SRC_SCRIPT: &str = include_str!("../lib/templates/app.src.script");

#[derive(Debug, thiserror::Error)]
pub(crate) enum NewError {
    #[error("cannot infer package name from {0}, pass `--pkg-name`")]
    InvalidPackageName(#[from] InvalidPackageName),
    #[error("package name `{0}` is reserved")]
    Reserved(PackageName),
    #[error("{0} already exists and isn't an empty directory")]
    NotEmpty(PathBuf),
    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}

pub(crate) fn new(dir: &Path, name: Option<PackageName>) -> Result<(), NewError> {
    let name = match name {
        Some(name) => name,
        None => dir.to_string_lossy().parse()?,
    };
    if name.is_reserved() {
        return Err(NewError::Reserved(name));
    }

    let is_empty_dir = std::fs::read_dir(dir).is_ok_and(|mut entries| entries.next().is_none());
    if dir.exists() && !is_empty_dir {
        return Err(NewError::NotEmpty(dir.to_owned()));
    }

    let manifest = MANIFEST.replace("%NAME%", name.as_str());
    let src = dir.join("src");

    std::fs::create_dir_all(&src)
        .map_err(|source| NewError::Write { path: src.clone(), source })?;

    let files = [
        (MANIFEST_FILE.to_owned(), manifest.as_str()),
        ("rebar.config.script".to_owned(), REBAR_CONFIG_SCRIPT),
        (format!("src/{name}.pt"), ROOT_FILE),
        (format!("src/{name}.app.src.script"), APP_SRC_SCRIPT),
    ];

    for (file, contents) in files {
        let path = dir.join(file);
        std::fs::write(&path, contents).map_err(|source| NewError::Write { path, source })?;
    }
    Ok(())
}
