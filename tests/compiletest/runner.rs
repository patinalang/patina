use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use libtest_mimic::Failed;

use crate::snapshot;

const TEXT_EXTENSIONS: &[&str] = &["tokens", "ast", "expanded", "typed", "erl", "abstr"];

struct TextArtifact {
    file_name: String,
    content: String,
}

struct CompileOutput {
    artifacts: Vec<TextArtifact>,
    stderr: String,
}

pub fn run(pt_path: &Path, bless: bool) -> Result<(), Failed> {
    let output = compile(pt_path);
    let dir = snapshot_base(pt_path);

    for artifact in &output.artifacts {
        snapshot::check_required(&dir.join(&artifact.file_name), &artifact.content, bless)?;
    }

    snapshot::check_optional(&dir.join("stderr"), &output.stderr, bless)
}

pub fn run_exs(pt_path: &Path, exs_path: &Path) -> Result<(), Failed> {
    let out_dir = tempfile::tempdir().expect("create temp out dir");
    let compiled = run_compile(&[], &["beam"], pt_path, out_dir.path());

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
                .expect(&format!("canonicalize elixir test {}", exs_path.display())),
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

fn compile(pt_path: &Path) -> CompileOutput {
    let out_dir = tempfile::tempdir().expect("create temp out dir");
    let output = run_compile(
        &["tokens", "ast", "expanded", "typed", "erl"],
        &["abstr", "beam"],
        pt_path,
        out_dir.path(),
    );
    let text_extensions: HashSet<&str> = TEXT_EXTENSIONS.iter().copied().collect();
    let artifacts = read_text_artifacts(out_dir.path(), &text_extensions);

    CompileOutput { artifacts, stderr: String::from_utf8_lossy(&output.stderr).into_owned() }
}

fn run_compile(dumps: &[&str], emits: &[&str], pt_path: &Path, out_dir: &Path) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_patina"));
    for dump in dumps {
        cmd.arg("--dump").arg(*dump);
    }
    for emit in emits {
        cmd.arg("--emit").arg(*emit);
    }
    cmd.arg("--out-dir").arg(out_dir).arg(pt_path);
    cmd.output().expect("run patina")
}

fn read_text_artifacts(out_dir: &Path, extensions: &HashSet<&str>) -> Vec<TextArtifact> {
    let mut artifacts = Vec::new();

    for entry in
        std::fs::read_dir(out_dir).expect(&format!("read emit out dir {}", out_dir.display()))
    {
        let path = entry.expect(&format!("read emit out dir {}", out_dir.display())).path();
        let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        if !extensions.contains(ext) {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect(&format!("emitted file name is not UTF-8: {}", path.display()))
            .to_string();
        let content =
            std::fs::read_to_string(&path).expect(&format!("read emitted file {}", path.display()));

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
