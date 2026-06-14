//! Phase definitions and mode dispatcher.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use libtest_mimic::Failed;

use crate::snapshot;

struct CompileDump {
    pub stdout: String,
    pub stderr: String,
}

struct TextArtifact {
    pub file_name: String,
    pub content: String,
}

struct EmitOutput {
    pub stderr: String,
}

struct TextEmitOutput {
    pub artifacts: Vec<TextArtifact>,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Tokens,
    Ast,
    Expanded,
    Typed,
    Abstr,
    Erl,
    Beam,
}

impl Phase {
    pub const ALL: &'static [Phase] = &[
        Phase::Tokens,
        Phase::Ast,
        Phase::Expanded,
        Phase::Typed,
        Phase::Abstr,
        Phase::Erl,
        Phase::Beam,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Phase::Tokens => "tokens",
            Phase::Ast => "ast",
            Phase::Expanded => "expanded",
            Phase::Typed => "typed",
            Phase::Abstr => "abstr",
            Phase::Erl => "erl",
            Phase::Beam => "beam",
        }
    }
}

pub fn run(phase: Phase, pt_path: &Path, bless: bool) -> Result<(), Failed> {
    match phase {
        Phase::Abstr => {
            let emit = compile_emit_text(phase.name(), phase.name(), pt_path);
            check_text_emit_snapshots(pt_path, phase.name(), emit, bless)
        }
        Phase::Beam => {
            let emit = compile_emit_to_dir(phase.name(), pt_path, &emit_artifacts_dir(pt_path));
            check_emit_stderr_snapshot(pt_path, phase.name(), emit, bless)
        }
        phase => {
            let dump = compile_dump(phase.name(), pt_path);
            check_dump_snapshots(pt_path, phase.name(), dump, bless)
        }
    }
}

pub fn run_exs(pt_path: &Path, exs_path: &Path) -> Result<(), Failed> {
    let out_dir = emit_artifacts_dir(pt_path);
    compile_emit_to_dir("beam", pt_path, &out_dir);

    let output = Command::new("elixir")
        .arg(
            exs_path
                .canonicalize()
                .expect(&format!("canonicalize elixir test {}", exs_path.display())),
        )
        .current_dir(&out_dir)
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

fn check_dump_snapshots(
    pt_path: &Path,
    ext: &str,
    dump: CompileDump,
    bless: bool,
) -> Result<(), Failed> {
    let stdout_path = stdout_snapshot_path(pt_path, ext);
    let stderr_path = stderr_snapshot_path(pt_path, ext);
    snapshot::check_pair(&stdout_path, &dump.stdout, &stderr_path, &dump.stderr, bless)
}

fn check_text_emit_snapshots(
    pt_path: &Path,
    emit: &str,
    output: TextEmitOutput,
    bless: bool,
) -> Result<(), Failed> {
    let dir = emit_artifacts_dir(pt_path);
    let stderr_path = stderr_snapshot_path(pt_path, emit);

    let mut checked_paths = Vec::new();
    for artifact in &output.artifacts {
        let path = dir.join(&artifact.file_name);
        snapshot::check_required(&path, &artifact.content, bless)?;
        checked_paths.push(path);
    }
    if bless {
        remove_stale_emit_snapshots(pt_path, emit, &checked_paths);
    }

    snapshot::check_optional(&stderr_path, &output.stderr, bless)
}

fn check_emit_stderr_snapshot(
    pt_path: &Path,
    emit: &str,
    output: EmitOutput,
    bless: bool,
) -> Result<(), Failed> {
    let stderr_path = stderr_snapshot_path(pt_path, emit);
    snapshot::check_optional(&stderr_path, &output.stderr, bless)
}

fn remove_stale_emit_snapshots(pt_path: &Path, extension: &str, keep: &[PathBuf]) {
    let dir = emit_artifacts_dir(pt_path);
    if dir.exists() {
        for entry in
            std::fs::read_dir(&dir).expect(&format!("read emit snapshot dir {}", dir.display()))
        {
            let path = entry.expect(&format!("read emit snapshot dir {}", dir.display())).path();
            if path.extension().is_some_and(|ext| ext == extension)
                && !keep.iter().any(|kept| kept == &path)
            {
                std::fs::remove_file(&path)
                    .expect(&format!("remove stale emit snapshot {}", path.display()));
            }
        }
    }

    let base = snapshot_base(pt_path);
    let Some(parent) = base.parent() else { return };
    let case_name =
        base.file_stem().and_then(|stem| stem.to_str()).expect("snapshot stem is UTF-8");
    let old_single_snapshot = format!("{case_name}.{extension}");
    let old_snapshot_prefix = format!("{case_name}.");

    for entry in
        std::fs::read_dir(parent).expect(&format!("read snapshot dir {}", parent.display()))
    {
        let path = entry.expect(&format!("read snapshot dir {}", parent.display())).path();
        if !path.extension().is_some_and(|ext| ext == extension) {
            continue;
        }

        let name = path.file_name().and_then(|name| name.to_str()).expect("snapshot name is UTF-8");
        if name == old_single_snapshot || name.starts_with(&old_snapshot_prefix) {
            std::fs::remove_file(&path)
                .expect(&format!("remove stale emit snapshot {}", path.display()));
        }
    }
}

fn stdout_snapshot_path(pt_path: &Path, ext: &str) -> PathBuf {
    snapshot_base(pt_path).with_extension(ext)
}

fn stderr_snapshot_path(pt_path: &Path, ext: &str) -> PathBuf {
    snapshot_base(pt_path).with_extension(ext).with_added_extension("stderr")
}

fn snapshot_base(pt_path: &Path) -> PathBuf {
    let rel = pt_path.strip_prefix(crate::UI_ROOT).expect("pt sources live under UI_ROOT");
    let rel = rel.strip_prefix("src").unwrap_or(rel);
    Path::new(crate::SNAPSHOTS_ROOT).join(rel)
}

fn emit_artifacts_dir(pt_path: &Path) -> PathBuf {
    snapshot_base(pt_path).with_extension("")
}

fn compile_dump(dump: &str, pt_path: &Path) -> CompileDump {
    let output = Command::new(env!("CARGO_BIN_EXE_patina"))
        .arg("--dump")
        .arg(dump)
        .arg(pt_path)
        .output()
        .expect("run patina");

    CompileDump {
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn compile_emit_text(emit: &str, extension: &str, pt_path: &Path) -> TextEmitOutput {
    let out_dir = tempfile::tempdir().expect("create emit out dir");

    let output = run_emit(emit, pt_path, out_dir.path());
    let artifacts = read_text_artifacts(out_dir.path(), extension);

    TextEmitOutput { artifacts, stderr: String::from_utf8_lossy(&output.stderr).into_owned() }
}

fn compile_emit_to_dir(emit: &str, pt_path: &Path, out_dir: &Path) -> EmitOutput {
    std::fs::create_dir_all(out_dir).expect(&format!("create emit out dir {}", out_dir.display()));
    let output = run_emit(emit, pt_path, out_dir);

    EmitOutput { stderr: String::from_utf8_lossy(&output.stderr).into_owned() }
}

fn run_emit(emit: &str, pt_path: &Path, out_dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_patina"))
        .arg("--emit")
        .arg(emit)
        .arg("--out-dir")
        .arg(out_dir)
        .arg(pt_path)
        .output()
        .expect("run patina")
}

fn read_text_artifacts(out_dir: &Path, extension: &str) -> Vec<TextArtifact> {
    let mut artifacts = Vec::new();

    for entry in
        std::fs::read_dir(out_dir).expect(&format!("read emit out dir {}", out_dir.display()))
    {
        let path = entry.expect(&format!("read emit out dir {}", out_dir.display())).path();
        if !path.extension().is_some_and(|ext| ext == extension) {
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
