use std::path::{Path, PathBuf};
use std::str::FromStr;

use convert_case::{Case, Casing};
use rustc_data_structures::fx::FxHashSet;
use rustc_session::config::{ExternEntry, ExternLocation};
use rustc_session::utils::CanonicalizedPath;

const CORE_PACKAGE_NAME: &str = "Core";
const CORE_PACKAGE_OTP_APP: &str = "patina_core";
const STD_PACKAGE_NAME: &str = "Std";
const PROC_MACRO_PACKAGE_NAME: &str = "ProcMacro";
const PROC_MACRO_PACKAGE_OTP_APP: &str = "patina_proc_macro";

#[derive(Debug, thiserror::Error)]
pub(crate) enum DependencySpecError {
    #[error("missing dependency name")]
    MissingName,
    #[error("missing metadata path")]
    MissingPath,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PackageNameError {
    #[error("cannot derive a package name from {}", .0.display())]
    NoFileStem(PathBuf),
    #[error("package name `{0}` is reserved")]
    Reserved(String),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum DependencyError {
    #[error("duplicate package dependency `{0}`")]
    Duplicate(String),
    #[error("package dependency `{name}` metadata file does not exist at {}", path.display())]
    MissingMetadata { name: String, path: PathBuf },
}

pub(crate) struct PackageInfo {
    pub name: String,
    pub otp_application_name: String,
    pub version: Option<String>,
    pub description: String,
}

impl PackageInfo {
    pub(crate) fn is_core(&self) -> bool {
        self.name == CORE_PACKAGE_NAME
    }

    pub(crate) fn is_proc_macro(&self) -> bool {
        self.name == PROC_MACRO_PACKAGE_NAME
    }

    pub(crate) fn core_package_info() -> PackageInfo {
        Self {
            name: CORE_PACKAGE_NAME.to_owned(),
            otp_application_name: CORE_PACKAGE_OTP_APP.to_owned(),
            version: None,
            description: String::new(),
        }
    }

    pub(crate) fn proc_macro_package_info() -> PackageInfo {
        Self {
            name: PROC_MACRO_PACKAGE_NAME.to_owned(),
            otp_application_name: PROC_MACRO_PACKAGE_OTP_APP.to_owned(),
            version: None,
            description: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PackageDependency {
    pub name: String,
    pub path: Option<PathBuf>,
}

impl PackageDependency {
    pub(crate) fn is_core(&self) -> bool {
        self.name == CORE_PACKAGE_NAME
    }

    pub(crate) fn is_proc_macro(&self) -> bool {
        self.name == PROC_MACRO_PACKAGE_NAME
    }

    pub(crate) fn core_package_dependency(path: Option<PathBuf>) -> Self {
        Self { name: CORE_PACKAGE_NAME.to_owned(), path }
    }

    pub(crate) fn proc_macro_package_dependency(path: Option<PathBuf>) -> Self {
        Self { name: PROC_MACRO_PACKAGE_NAME.to_owned(), path }
    }
}

impl FromStr for PackageDependency {
    type Err = DependencySpecError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            return Err(DependencySpecError::MissingName);
        }
        match raw.split_once('=') {
            Some(("", _)) => Err(DependencySpecError::MissingName),
            Some((_, "")) => Err(DependencySpecError::MissingPath),
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
            // Even if no code uses a dependency, specifying it still causes
            // its Erlang app to be loaded. By marking `force: true` we're
            // making sure unused dependencies are still processed.
            force: true,
        }
    }
}

pub(crate) fn snake_case(name: &str) -> String {
    name.to_case(Case::Snake)
}

pub(crate) fn resolve_package_name(
    explicit: Option<&str>,
    source: &Path,
) -> Result<String, PackageNameError> {
    let name = match explicit {
        Some(name) => name.to_owned(),
        None => match source.file_stem().and_then(|stem| stem.to_str()) {
            Some(stem) => stem.to_case(Case::Pascal),
            None => return Err(PackageNameError::NoFileStem(source.to_owned())),
        },
    };

    match name.as_str() {
        CORE_PACKAGE_NAME | STD_PACKAGE_NAME | PROC_MACRO_PACKAGE_NAME => {
            Err(PackageNameError::Reserved(name))
        }
        _ => Ok(name),
    }
}

pub(crate) fn validate_dependencies(deps: &[PackageDependency]) -> Result<(), DependencyError> {
    let mut seen = FxHashSet::default();
    for dep in deps {
        if !seen.insert(&dep.name) {
            return Err(DependencyError::Duplicate(dep.name.clone()));
        }
        if let Some(path) = &dep.path
            && !path.is_file()
        {
            return Err(DependencyError::MissingMetadata {
                name: dep.name.clone(),
                path: path.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_keeps_acronyms_together() {
        assert_eq!(snake_case("MyJsonLib"), "my_json_lib");
        assert_eq!(snake_case("MyJSONLib"), "my_json_lib");
        assert_eq!(snake_case("Core"), "core");
    }
}
