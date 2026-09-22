use std::path::{Path, PathBuf};
use std::str::FromStr;

use convert_case::{Case, Casing};
use rustc_data_structures::fx::FxHashSet;
use rustc_session::config::{ExternEntry, ExternLocation};
use rustc_session::utils::CanonicalizedPath;

pub(crate) const CORE_PACKAGE_NAME: &str = "Core";
pub(crate) const STD_PACKAGE_NAME: &str = "Std";
pub(crate) const PROC_MACRO_PACKAGE_NAME: &str = "ProcMacro";

#[derive(Debug, Clone)]
pub(crate) struct PackageDependency {
    pub name: String,
    pub path: Option<PathBuf>,
}

impl FromStr for PackageDependency {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            return Err("missing dependency name");
        }
        match raw.split_once('=') {
            Some(("", _)) => Err("missing dependency name"),
            Some((_, "")) => Err("missing metadata path"),
            Some((name, path)) => {
                Ok(Self { name: name.to_owned(), path: Some(PathBuf::from(path)) })
            }
            None => Ok(Self { name: raw.to_owned(), path: None }),
        }
    }
}

impl From<&PackageDependency> for ExternEntry {
    fn from(dep: &PackageDependency) -> ExternEntry {
        let location = match &dep.path {
            Some(path) => {
                ExternLocation::ExactPaths([CanonicalizedPath::new(path.to_owned())].into())
            }
            None => ExternLocation::FoundInLibrarySearchDirectories,
        };

        ExternEntry {
            location,
            is_private_dep: false,
            add_prelude: dep.name != CORE_PACKAGE_NAME,
            nounused_dep: dep.name == CORE_PACKAGE_NAME,
            force: dep.name == CORE_PACKAGE_NAME,
        }
    }
}

pub(crate) fn resolve_package_name(
    explicit: Option<&str>,
    source: &Path,
) -> Result<String, &'static str> {
    let name = explicit.map(str::to_owned).unwrap_or_else(|| {
        let stem = source.file_stem().and_then(|s| s.to_str()).expect("source file to have stem");
        stem.to_case(Case::Pascal)
    });

    match name.as_str() {
        CORE_PACKAGE_NAME | STD_PACKAGE_NAME | PROC_MACRO_PACKAGE_NAME => {
            Err("package name is reserved")
        }
        _ => Ok(name),
    }
}

pub(crate) fn validate_dependencies(deps: &[PackageDependency]) -> Result<(), String> {
    let mut seen = FxHashSet::default();
    for dep in deps {
        if !seen.insert(&dep.name) {
            return Err(format!("duplicate package dependency `{}`", dep.name));
        }
        if let Some(path) = &dep.path
            && !path.is_file()
        {
            return Err(format!(
                "package dependency `{}` metadata file does not exist at {}",
                dep.name,
                path.display()
            ));
        }
    }
    Ok(())
}
