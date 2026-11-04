use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use convert_case::{Case, Casing};
use libtest_mimic::Failed;

use crate::snapshot;

const TEXT_EXTENSIONS: &[&str] =
    &["tokens", "ast", "expanded", "typed", "erl", "core0", "core", "abstr"];

/// The library packages under `pkg-src/`, in build order: each may use only the previous.
const PACKAGES: &[&str] = &[
    "consts",
    "temperatures",
    "traits",
    "wrappers",
    "macros_and_items",
    "more_temperatures",
    "label_shout",
    "label_whisper",
    "macros",
    "reexports",
    "inlines",
    "shared_modules",
];

struct TextArtifact {
    file_name: String,
    content: String,
}

struct CompileOutput {
    artifacts: Vec<TextArtifact>,
    stderr: String,
}

pub struct Runner {
    pub compile_server: PathBuf,
    pub bless: bool,
}

impl Runner {
    pub fn run(&self, pt_path: &Path) -> Result<(), Failed> {
        let output = self.compile(pt_path);
        let dir = snapshot_base(pt_path);

        for artifact in &output.artifacts {
            snapshot::check_required(
                &dir.join(&artifact.file_name),
                &artifact.content,
                self.bless,
            )?;
        }

        snapshot::check_optional(&dir.join("stderr"), &output.stderr, self.bless)
    }

    pub fn run_exs(&self, pt_path: &Path, exs_path: &Path) -> Result<(), Failed> {
        let out_dir = tempfile::tempdir().expect("create temp out dir");
        let externs = self.package_deps(pt_path, out_dir.path());
        let compiled = self.run_compile(&[], &["beam"], pt_path, out_dir.path(), &externs, false);

        if !compiled.status.success() {
            return Err(Failed::from(format!(
                "failed to compile {}\nstatus: {}\nstdout:\n{}stderr:\n{}",
                pt_path.display(),
                compiled.status,
                String::from_utf8_lossy(&compiled.stdout),
                String::from_utf8_lossy(&compiled.stderr),
            )));
        }

        let output = Command::new("elixir")
            .arg(
                exs_path
                    .canonicalize()
                    .unwrap_or_else(|_| panic!("canonicalize elixir test {}", exs_path.display())),
            )
            .current_dir(out_dir.path())
            .output()
            .expect("run elixir");

        if output.status.success() {
            Ok(())
        } else {
            Err(Failed::from(format!(
                "elixir test failed: {}\nstatus: {}\nstdout:\n{}stderr:\n{}",
                exs_path.display(),
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            )))
        }
    }

    fn compile(&self, pt_path: &Path) -> CompileOutput {
        let out_dir = tempfile::tempdir().expect("create temp out dir");
        let externs = self.package_deps(pt_path, out_dir.path());
        let source = std::fs::read_to_string(pt_path).expect("read test source");
        let mut dumps = vec!["tokens", "ast", "expanded", "typed", "erl"];

        let mut header = source.lines().take_while(|line| line.starts_with("//"));
        if let Some(extra) = header.find_map(|line| line.strip_prefix("// snapshot:")) {
            dumps.extend(extra.split(',').map(str::trim));
        }

        let output =
            self.run_compile(&dumps, &["abstr", "beam"], pt_path, out_dir.path(), &externs, false);
        let text_extensions: HashSet<&str> = TEXT_EXTENSIONS.iter().copied().collect();
        let artifacts = read_text_artifacts(out_dir.path(), &text_extensions);

        CompileOutput { artifacts, stderr: String::from_utf8_lossy(&output.stderr).into_owned() }
    }

    fn package_deps(&self, pt_path: &Path, out_dir: &Path) -> Vec<String> {
        let mut deps = Vec::new();
        let is_pkg_test =
            pt_path.strip_prefix(crate::UI_ROOT).is_ok_and(|rel| rel.starts_with("pkg"));
        if !is_pkg_test {
            return deps;
        }

        for package in PACKAGES {
            let src = Path::new(crate::PKG_SRC_ROOT).join(format!("{package}.pt"));
            let source = std::fs::read_to_string(&src)
                .unwrap_or_else(|_| panic!("read dependency package {}", src.display()));
            let proc_macro = source.contains("ProcMacro");
            let output = self.run_compile(&[], &["meta", "beam"], &src, out_dir, &deps, proc_macro);
            assert!(
                output.status.success(),
                "failed to build dependency package {}:\nstdout:\n{}stderr:\n{}",
                src.display(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
            deps.push(package.to_case(Case::Pascal));
        }
        deps
    }

    fn run_compile(
        &self,
        dumps: &[&str],
        emits: &[&str],
        pt_path: &Path,
        out_dir: &Path,
        externs: &[String],
        proc_macro: bool,
    ) -> Output {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_patina"));
        for dump in dumps {
            cmd.arg("--dump").arg(*dump);
        }
        for emit in emits {
            cmd.arg("--emit").arg(*emit);
        }
        for name in externs {
            cmd.arg("--extern").arg(name);
        }
        if proc_macro {
            cmd.arg("--extern").arg("ProcMacro");
        }
        cmd.arg("--compile-server").arg(&self.compile_server);
        cmd.arg("--out-dir").arg(out_dir).arg(pt_path);
        cmd.output().expect("run patina")
    }
}

fn read_text_artifacts(out_dir: &Path, extensions: &HashSet<&str>) -> Vec<TextArtifact> {
    let mut artifacts = Vec::new();

    for entry in std::fs::read_dir(out_dir)
        .unwrap_or_else(|_| panic!("read emit out dir {}", out_dir.display()))
    {
        let path =
            entry.unwrap_or_else(|_| panic!("read emit out dir {}", out_dir.display())).path();
        let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        if !extensions.contains(ext) {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_else(|| panic!("emitted file name is not UTF-8: {}", path.display()))
            .to_string();
        let content = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("read emitted file {}", path.display()));

        artifacts.push(TextArtifact { file_name, content });
    }

    artifacts.sort_by(|a, b| a.file_name.cmp(&b.file_name));
    artifacts
}

fn snapshot_base(pt_path: &Path) -> PathBuf {
    let rel = pt_path.strip_prefix(crate::UI_ROOT).expect("pt sources live under UI_ROOT");
    let rel = rel.strip_prefix("src").unwrap_or(rel);
    Path::new(crate::SNAPSHOTS_ROOT).join(rel).with_extension("")
}
