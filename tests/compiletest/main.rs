mod phases;
mod snapshot;

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use libtest_mimic::Trial;
use walkdir::WalkDir;

use crate::phases::Phase;

pub(crate) const UI_ROOT: &str = "tests/ui/src";
pub(crate) const SNAPSHOTS_ROOT: &str = "tests/ui/snapshots";

#[derive(Parser, Debug)]
#[command(
    name = "compiletest",
    about = "Snapshot-based test runner for Patina UI tests.",
    disable_version_flag = true
)]
struct Args {
    /// Write actual output back to disk instead of diffing.
    #[arg(long)]
    bless: bool,

    #[command(flatten)]
    mimic: libtest_mimic::Arguments,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let trials = match collect_trials(args.bless) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("compiletest: failed to collect trials: {e:#}");
            return ExitCode::FAILURE;
        }
    };

    libtest_mimic::run(&args.mimic, trials).exit_code()
}

fn collect_trials(bless: bool) -> anyhow::Result<Vec<Trial>> {
    let ui_root = Path::new(UI_ROOT);
    if !ui_root.is_dir() {
        anyhow::bail!("{UI_ROOT} does not exist; run `cargo run -p extract` first");
    }

    let mut trials = Vec::new();
    for entry in WalkDir::new(ui_root) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.into_path();
        if !path.extension().is_some_and(|ext| ext == "pt") {
            continue;
        }

        let rel =
            path.strip_prefix(ui_root).expect("walkdir paths are under ui_root").to_path_buf();

        for phase in Phase::ALL.iter().copied() {
            let path = path.clone();
            let test_name = format!("{}::{}", rel.display(), phase.dump_stage());
            trials.push(Trial::test(test_name, move || phases::run(phase, &path, bless)));
        }
    }

    Ok(trials)
}
