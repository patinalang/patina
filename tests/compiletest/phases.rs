//! Phase definitions and mode dispatcher.

use std::path::{Path, PathBuf};
use std::process::Command;

use libtest_mimic::Failed;

struct CompileDump {
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Tokens,
    Ast,
    Expanded,
    Typed,
}

impl Phase {
    pub const ALL: &'static [Phase] = &[Phase::Tokens, Phase::Ast, Phase::Expanded, Phase::Typed];

    pub fn dump_stage(self) -> &'static str {
        match self {
            Phase::Tokens => "tokens",
            Phase::Ast => "ast",
            Phase::Expanded => "expanded",
            Phase::Typed => "typed",
        }
    }
}

pub fn run(phase: Phase, pt_path: &Path, bless: bool) -> Result<(), Failed> {
    let dump = compile_dump(phase.dump_stage(), pt_path);
    check_dump_snapshots(pt_path, phase.dump_stage(), dump, bless)
}

fn check_dump_snapshots(
    pt_path: &Path,
    ext: &str,
    dump: CompileDump,
    bless: bool,
) -> Result<(), Failed> {
    let stdout_path = stdout_snapshot_path(pt_path, ext);
    let stderr_path = stderr_snapshot_path(pt_path, ext);
    crate::snapshot::check_pair(&stdout_path, &dump.stdout, &stderr_path, &dump.stderr, bless)
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
