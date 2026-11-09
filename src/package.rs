use std::path::{self, Path, PathBuf};
use std::str::FromStr;
use std::{fmt, io};

use rustc_data_structures::fx::FxIndexMap;
use rustc_session::config::{ExternEntry, ExternLocation, Externs};
use rustc_session::utils::CanonicalizedPath;
use serde::Deserialize;

const CORE_PACKAGE_NAME: &str = "core";
const STD_PACKAGE_NAME: &str = "std";
const PATINA_BOOTSTRAP_ENV: &str = "PATINA_BOOTSTRAP";

#[derive(Debug, thiserror::Error)]
pub(crate) enum ExternArgError {
    #[error("missing package name")]
    MissingName,
    #[error(transparent)]
    InvalidName(#[from] InvalidPackageName),
    #[error("missing metadata path")]
    MissingPath,
    #[error("failed to resolve metadata path: {0}")]
    ResolvePath(#[source] io::Error),
    #[error("metadata file {0} does not exist")]
    MissingMetadata(PathBuf),
}

#[derive(Debug, thiserror::Error)]
#[error("`{0}` isn't a valid package name")]
pub(crate) struct InvalidPackageName(String);

#[derive(Debug, thiserror::Error)]
pub(crate) enum PackageNameError {
    #[error("cannot derive a package name from {0}")]
    NoFileStem(PathBuf),
    #[error(transparent)]
    Invalid(#[from] InvalidPackageName),
    #[error("package name `{0}` is reserved")]
    Reserved(PackageName),
}

#[derive(Debug, thiserror::Error)]
#[error("`--extern {0}` is given more than once")]
pub(crate) struct DuplicateExtern(PackageName);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub(crate) struct PackageName(String);

impl PackageName {
    fn core() -> Self {
        Self(CORE_PACKAGE_NAME.to_owned())
    }

    pub(crate) fn is_core(&self) -> bool {
        self.0 == CORE_PACKAGE_NAME
    }

    fn is_reserved(&self) -> bool {
        matches!(self.0.as_str(), CORE_PACKAGE_NAME | STD_PACKAGE_NAME)
    }
}

impl TryFrom<String> for PackageName {
    type Error = InvalidPackageName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if rustc_lexer::is_ident(&name) { Ok(Self(name)) } else { Err(InvalidPackageName(name)) }
    }
}

impl FromStr for PackageName {
    type Err = InvalidPackageName;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        name.to_owned().try_into()
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

pub(crate) struct PackageInfo {
    pub name: PackageName,
    pub otp_app_name: String,
    pub version: Option<String>,
    pub description: String,
}

impl PackageInfo {
    pub(crate) fn new(
        name: PackageName,
        otp_app_name: Option<String>,
        version: Option<String>,
        description: Option<String>,
    ) -> PackageInfo {
        let otp_app_name = otp_app_name.unwrap_or_else(|| name.to_string());
        let description = description.unwrap_or_else(|| "".to_owned());
        PackageInfo { name, otp_app_name, version, description }
    }

    pub(crate) fn is_core(&self) -> bool {
        self.name.is_core()
    }

    pub(crate) fn externs(
        &self,
        deps: impl IntoIterator<Item = ExternArg>,
    ) -> Result<Externs, DuplicateExtern> {
        let core = PackageName::core();
        let core_dep = (!self.is_core()).then(|| (core.0.clone(), extern_entry(&core, None)));

        let mut entries = FxIndexMap::from_iter(core_dep);

        for ExternArg { name, path } in deps {
            let entry = extern_entry(&name, path);
            if let Some(old) = entries.insert(name.0.clone(), entry)
                && !(name.is_core() && old.files().is_none())
            {
                return Err(DuplicateExtern(name));
            }
        }

        Ok(Externs::new(entries))
    }
}

/// One `--extern NAME[=RMETA]`. Without `RMETA`, the package is found on the search path.
#[derive(Debug, Clone)]
pub(crate) struct ExternArg {
    pub name: PackageName,
    pub path: Option<PathBuf>,
}

impl FromStr for ExternArg {
    type Err = ExternArgError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.is_empty() {
            return Err(ExternArgError::MissingName);
        }
        match raw.split_once('=') {
            Some(("", _)) => Err(ExternArgError::MissingName),
            Some((_, "")) => Err(ExternArgError::MissingPath),
            Some((name, path)) => {
                let name = name.parse()?;
                let path = path::absolute(path).map_err(ExternArgError::ResolvePath)?;
                if !path.is_file() {
                    return Err(ExternArgError::MissingMetadata(path));
                }
                Ok(Self { name, path: Some(path) })
            }
            None => Ok(Self { name: raw.parse()?, path: None }),
        }
    }
}

/// A dependency that's found on the search path.
impl From<PackageName> for ExternArg {
    fn from(name: PackageName) -> Self {
        Self { name, path: None }
    }
}

/// Whether `externs` names a specific metadata file for Core,
/// rather than leaving Core to the search path.
pub(crate) fn names_core_file(externs: &Externs) -> bool {
    externs.get(CORE_PACKAGE_NAME).is_some_and(|entry| entry.files().is_some())
}

fn extern_entry(name: &PackageName, path: Option<PathBuf>) -> ExternEntry {
    let location = match path {
        Some(path) => ExternLocation::ExactPaths([CanonicalizedPath::new(path)].into()),
        None => ExternLocation::FoundInLibrarySearchDirectories,
    };

    ExternEntry {
        location,
        is_private_dep: false,
        add_prelude: !name.is_core(),
        nounused_dep: name.is_core(),
        // Even if no code uses a dependency, specifying it still causes
        // its Erlang app to be loaded. By marking `force: true` we're
        // making sure unused dependencies are still processed.
        force: true,
    }
}

/// Are we building patina itself (`true`), or building with patina (`false`).
pub(crate) fn is_bootstrap() -> bool {
    std::env::var_os(PATINA_BOOTSTRAP_ENV).is_some_and(|value| value == "1")
}

pub(crate) fn resolve_package_name(
    explicit: Option<PackageName>,
    source: &Path,
) -> Result<PackageName, PackageNameError> {
    let name = match explicit {
        Some(name) => name,
        None => match source.file_stem().and_then(|stem| stem.to_str()) {
            Some(stem) => PackageName::try_from(stem.to_owned())?,
            None => return Err(PackageNameError::NoFileStem(source.to_owned())),
        },
    };

    if name.is_reserved() && !is_bootstrap() {
        Err(PackageNameError::Reserved(name))
    } else {
        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(name: &str) -> PackageName {
        name.parse().unwrap()
    }

    fn package(package: &str) -> PackageInfo {
        PackageInfo::new(name(package), None, None, None)
    }

    fn explicit(dependency: &str, path: Option<&Path>) -> ExternArg {
        ExternArg { name: name(dependency), path: path.map(Path::to_owned) }
    }

    fn entries(externs: &Externs) -> Vec<(&str, Option<&Path>)> {
        externs
            .iter()
            .map(|(name, entry)| {
                let file = entry.files().and_then(|mut files| files.next());
                (name.as_str(), file.map(|file| file.original().as_path()))
            })
            .collect()
    }

    #[test]
    fn package_name_has_to_be_an_identifier() {
        assert!("my_json_lib".parse::<PackageName>().is_ok());
        for invalid in ["", "hello-lib", "1st", "My Lib"] {
            assert!(
                matches!(invalid.parse::<PackageName>(), Err(InvalidPackageName(name)) if name == invalid),
                "for {invalid:?}"
            );
        }
    }

    #[test]
    fn package_name_comes_from_the_file_stem() {
        let resolved = resolve_package_name(None, Path::new("src/my_lib.pt")).unwrap();
        assert_eq!(resolved, name("my_lib"));
    }

    #[test]
    fn file_stem_has_to_make_a_package_name() {
        assert!(matches!(
            resolve_package_name(None, Path::new("1st.pt")),
            Err(PackageNameError::Invalid(_))
        ));
    }

    #[test]
    fn dependencies_follow_core() {
        let deps = [name("zeta"), name("alpha")].map(ExternArg::from);
        let externs = package("greeter").externs(deps).unwrap();
        assert_eq!(entries(&externs), [("core", None), ("zeta", None), ("alpha", None)]);
        assert!(!names_core_file(&externs));
    }

    #[test]
    fn explicit_core_stays_first() {
        let rmeta = Path::new("libcore.rmeta");
        let deps = vec![explicit("zeta", None), explicit("core", Some(rmeta))];

        let externs = package("greeter").externs(deps).unwrap();
        assert_eq!(entries(&externs), [("core", Some(rmeta)), ("zeta", None)]);
        assert!(names_core_file(&externs));
    }

    #[test]
    fn core_does_not_depend_on_itself() {
        assert!(entries(&package("core").externs([]).unwrap()).is_empty());
    }

    #[test]
    fn duplicate_extern_is_rejected() {
        let deps = vec![explicit("alpha", None), explicit("alpha", None)];
        assert!(matches!(
            package("greeter").externs(deps),
            Err(DuplicateExtern(dup)) if dup == name("alpha")
        ));
    }

    #[test]
    fn duplicate_explicit_core_is_rejected() {
        let rmeta = Path::new("libcore.rmeta");
        let deps = vec![explicit("core", Some(rmeta)), explicit("core", Some(rmeta))];
        assert!(matches!(
            package("greeter").externs(deps),
            Err(DuplicateExtern(dup)) if dup == name("core")
        ));
    }

    #[test]
    fn extern_metadata_file_has_to_exist() {
        let rmeta = std::env::current_exe().unwrap();
        assert!(format!("core={}", rmeta.display()).parse::<ExternArg>().is_ok());
        let missing = rmeta.with_file_name("libmissing.rmeta");
        assert!(matches!(
            format!("missing={}", missing.display()).parse::<ExternArg>(),
            Err(ExternArgError::MissingMetadata(path)) if path == missing
        ));
    }

    #[test]
    fn extern_needs_a_name_and_a_path_after_equals() {
        assert!(matches!("".parse::<ExternArg>(), Err(ExternArgError::MissingName)));
        assert!(matches!("=lib.rmeta".parse::<ExternArg>(), Err(ExternArgError::MissingName)));
        assert!(matches!("lib=".parse::<ExternArg>(), Err(ExternArgError::MissingPath)));
    }

    #[test]
    fn extern_name_has_to_be_an_identifier() {
        assert!("hello_lib".parse::<ExternArg>().is_ok());
        for raw in ["hello-lib", "hello-lib=lib/libhello.rmeta"] {
            assert!(
                matches!(
                    raw.parse::<ExternArg>(),
                    Err(ExternArgError::InvalidName(InvalidPackageName(name))) if name == "hello-lib"
                ),
                "for {raw}"
            );
        }
    }
}
