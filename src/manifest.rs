//! `Patina.toml`, the manifest of a package directory.

use std::io;
use std::path::{Path, PathBuf};

use rustc_data_structures::fx::FxIndexMap;
use serde::{Deserialize, Deserializer, de};
use serde_untagged::UntaggedEnumVisitor;

use crate::package::PackageName;

pub(crate) const MANIFEST_FILE: &str = "Patina.toml";

#[derive(Debug, thiserror::Error)]
pub(crate) enum ManifestError {
    #[error("failed to read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("invalid {path}: {source}")]
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
    /// In the manifest's order.
    #[serde(default)]
    dependencies: FxIndexMap<DependencyName, DependencyManifest>,
    /// The manifest file's directory.
    #[serde(skip)]
    dir: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PackageManifest {
    pub name: PackageName,
    pub version: String,
    pub otp_application_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DependencySource {
    Hex(String),
    Git { url: String, git_ref: Option<String> },
    Path(PathBuf),
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct NonEmptyString(String);

impl TryFrom<String> for NonEmptyString {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() { Err("can't be empty") } else { Ok(Self(value)) }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
struct DependencyName(PackageName);

impl TryFrom<String> for DependencyName {
    type Error = String;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        let name = PackageName::try_from(name).map_err(|err| err.to_string())?;
        if name.is_core() {
            return Err("core cannot be named as an explicit dependency".into());
        }
        Ok(Self(name))
    }
}

#[derive(Debug)]
struct DependencyManifest {
    #[cfg_attr(not(test), expect(dead_code))]
    version: Option<String>,
    source: Option<DependencySource>,
}

impl<'de> Deserialize<'de> for DependencyManifest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<DependencyManifest, D::Error> {
        UntaggedEnumVisitor::new()
            .expecting("a version requirement, or a table")
            .string(|version| {
                let version =
                    NonEmptyString::try_from(version.to_owned()).map_err(de::Error::custom)?;
                Ok(DependencyManifest { version: Some(version.0), source: None })
            })
            .map(|map| {
                let table: DependencyTable = map.deserialize()?;
                table.try_into().map_err(de::Error::custom)
            })
            .deserialize(deserializer)
    }
}

/// The table form of a `[dependencies]` entry.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DependencyTable {
    version: Option<NonEmptyString>,
    hex: Option<NonEmptyString>,
    git: Option<NonEmptyString>,
    #[serde(rename = "ref")]
    git_ref: Option<NonEmptyString>,
    path: Option<NonEmptyString>,
}

impl TryFrom<DependencyTable> for DependencyManifest {
    type Error = &'static str;

    fn try_from(table: DependencyTable) -> Result<Self, Self::Error> {
        let DependencyTable { version, hex, git, git_ref, path } = table;
        if git_ref.is_some() && git.is_none() {
            return Err("`ref` needs `git`");
        }
        let git_ref = git_ref.map(|git_ref| git_ref.0);
        let source = match (hex, git, path) {
            (None, None, None) => None,
            (Some(hex), None, None) => Some(DependencySource::Hex(hex.0)),
            (None, Some(url), None) => Some(DependencySource::Git { url: url.0, git_ref }),
            (None, None, Some(path)) => Some(DependencySource::Path(path.0.into())),
            _ => return Err("only one of `hex`, `git` and `path` can be given"),
        };
        Ok(DependencyManifest { version: version.map(|version| version.0), source })
    }
}

impl Manifest {
    pub(crate) fn read(dir: &Path) -> Result<Manifest, ManifestError> {
        let path = dir.join(MANIFEST_FILE);
        let read_err = |source| ManifestError::Read { path: path.clone(), source };
        let text = std::fs::read_to_string(&path).map_err(read_err)?;
        Self::parse(&text, dir).map_err(|source| ManifestError::Parse { path, source })
    }

    fn parse(text: &str, dir: &Path) -> Result<Manifest, toml::de::Error> {
        let mut manifest: Manifest = toml::from_str(text)?;
        manifest.dir = dir.to_owned();
        for dependency in manifest.dependencies.values_mut() {
            if let Some(DependencySource::Path(path)) = &mut dependency.source {
                *path = dir.join(&*path);
            }
        }
        Ok(manifest)
    }

    pub(crate) fn source_path(&self) -> PathBuf {
        self.dir.join("src").join(format!("{}.pt", self.package.name))
    }

    pub(crate) fn dependency_names(&self) -> Vec<PackageName> {
        self.dependencies.keys().map(|DependencyName(name)| name.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(dependencies: &str) -> Result<Manifest, String> {
        let text = format!(
            "[package]\nname = \"greeter\"\nversion = \"0.1.0\"\n\n[dependencies]\n{dependencies}"
        );
        Manifest::parse(&text, &package_dir()).map_err(|err| err.message().to_owned())
    }

    fn dependency(value: &str) -> Result<DependencyManifest, String> {
        let manifest = manifest(&format!("my_json_dep = {value}\n"))?;
        Ok(manifest.dependencies.into_values().next().expect("the dependency"))
    }

    #[track_caller]
    fn source(value: &str) -> Option<DependencySource> {
        dependency(value).unwrap().source
    }

    #[track_caller]
    fn version(value: &str) -> Option<String> {
        dependency(value).unwrap().version
    }

    #[track_caller]
    fn error(value: &str) -> String {
        dependency(value).unwrap_err()
    }

    fn package_dir() -> PathBuf {
        std::env::temp_dir().join("greeter")
    }

    #[test]
    fn dependency_sources() {
        assert_eq!(source(r#""~> 0.1""#), None);
        assert_eq!(source(r#"{ version = "~> 0.1" }"#), None);
        assert_eq!(source("{}"), None);
        assert_eq!(source(r#"{ hex = "dep_pkg" }"#), Some(DependencySource::Hex("dep_pkg".into())));
        assert_eq!(
            source(r#"{ git = "https://example.com/dep.git" }"#),
            Some(DependencySource::Git {
                url: "https://example.com/dep.git".into(),
                git_ref: None
            }),
        );
        assert_eq!(
            source(r#"{ git = "../dep", ref = "v0.1.0" }"#),
            Some(DependencySource::Git { url: "../dep".into(), git_ref: Some("v0.1.0".into()) }),
        );
    }

    #[test]
    fn version_goes_with_any_source() {
        assert_eq!(version(r#""~> 0.1""#).as_deref(), Some("~> 0.1"));
        assert_eq!(version(r#"{ version = "~> 0.1", hex = "dep" }"#).as_deref(), Some("~> 0.1"));
        assert_eq!(version(r#"{ version = "~> 0.1", git = "../dep" }"#).as_deref(), Some("~> 0.1"));
        assert_eq!(
            version(r#"{ version = "~> 0.1", path = "../dep" }"#).as_deref(),
            Some("~> 0.1")
        );
        assert_eq!(version(r#"{ path = "../dep" }"#), None);
        assert_eq!(version("{}"), None);
    }

    #[test]
    fn dependency_of_the_wrong_type_is_rejected() {
        assert!(error("1").contains("expected a version requirement, or a table"));
    }

    #[test]
    fn unknown_manifest_key_is_rejected() {
        let text = "[package]\nname = \"greeter\"\nversion = \"0.1.0\"\n\n[dependency]\n";
        let err = Manifest::parse(text, &package_dir()).unwrap_err();
        assert!(err.message().contains("unknown field `dependency`"), "got: {}", err.message());
    }

    #[test]
    fn invalid_dependency_sources() {
        let one_source = "only one of `hex`, `git` and `path` can be given";
        assert_eq!(error(r#"{ hex = "dep", git = "../dep" }"#), one_source);
        assert_eq!(error(r#"{ git = "../dep", path = "../dep" }"#), one_source);
        assert_eq!(error(r#"{ hex = "dep", ref = "main" }"#), "`ref` needs `git`");
        assert_eq!(error(r#"{ path = "../dep", ref = "abc" }"#), "`ref` needs `git`");
        // `ref` takes a branch or a tag too.
        for key in ["branch", "tag", "submodules"] {
            let value = format!(r#"{{ git = "../dep", {key} = "main" }}"#);
            assert!(error(&value).contains(&format!("unknown field `{key}`")), "for {value}");
        }
    }

    #[test]
    fn empty_values_are_rejected() {
        for value in [
            r#""""#,
            r#"{ version = "" }"#,
            r#"{ hex = "" }"#,
            r#"{ git = "" }"#,
            r#"{ git = "../dep", ref = "" }"#,
            r#"{ path = "" }"#,
        ] {
            let error = error(value);
            assert!(error.starts_with("can't be empty"), "for {value}, got: {error}");
        }
    }

    #[test]
    fn dependencies_keep_their_order() {
        let manifest =
            manifest("zeta = \"~> 0.1\"\nalpha = { path = \"../alpha\" }\nmid = \"~> 0.1\"\n");
        let names = manifest.unwrap().dependency_names();
        let names: Vec<_> = names.iter().map(PackageName::to_string).collect();
        assert_eq!(names, ["zeta", "alpha", "mid"]);
    }

    #[test]
    fn dependency_path_is_resolved_against_the_package_dir() {
        let absolute = std::env::temp_dir().join("elsewhere");
        let text = format!(
            "relative = {{ path = \"../lib\" }}\nabsolute = {{ path = '{}' }}\n",
            absolute.display()
        );
        let sources: Vec<_> =
            manifest(&text).unwrap().dependencies.into_values().map(|dep| dep.source).collect();
        assert_eq!(
            sources,
            [
                Some(DependencySource::Path(package_dir().join("../lib"))),
                Some(DependencySource::Path(absolute)),
            ]
        );
    }

    #[test]
    fn invalid_package_name_is_rejected() {
        let text = "[package]\nname = \"hello-lib\"\nversion = \"0.1.0\"\n";
        let err = Manifest::parse(text, &package_dir()).unwrap_err();
        assert_eq!(err.message(), "`hello-lib` isn't a valid package name");
    }

    #[test]
    fn invalid_dependency_names() {
        let name_error = |dependencies: &str| manifest(dependencies).unwrap_err();
        assert_eq!(
            name_error("\"hello-lib\" = \"~> 0.1\"\n"),
            "`hello-lib` isn't a valid package name"
        );
        assert_eq!(name_error("\"\" = \"~> 0.1\"\n"), "`` isn't a valid package name");
        assert_eq!(
            name_error("core = \"~> 0.1\"\n"),
            "core cannot be named as an explicit dependency"
        );
    }
}
