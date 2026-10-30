mod runner;
mod snapshot;

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use libtest_mimic::Trial;
use walkdir::WalkDir;

pub(crate) const UI_ROOT: &str = "tests/ui/src";
pub(crate) const SNAPSHOTS_ROOT: &str = "tests/ui/snapshots";
pub(crate) const PKG_SRC_ROOT: &str = "tests/ui/pkg-src";

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

    let trials = match collect_snapshot_trials(args.bless) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("compiletest: failed to collect snapshot trials: {e:#}");
            return ExitCode::FAILURE;
        }
    };

    if libtest_mimic::run(&args.mimic, trials).exit_code() != ExitCode::SUCCESS {
        return ExitCode::FAILURE;
    }

    let trials = match collect_elixir_trials() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("compiletest: failed to collect elixir trials: {e:#}");
            return ExitCode::FAILURE;
        }
    };

    libtest_mimic::run(&args.mimic, trials).exit_code()
}

fn collect_snapshot_trials(bless: bool) -> anyhow::Result<Vec<Trial>> {
    let ui_root = Path::new(UI_ROOT);
    if !ui_root.is_dir() {
        anyhow::bail!("{UI_ROOT} does not exist");
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

        let test_name = rel.display().to_string();
        trials.push(Trial::test(test_name, move || runner::run(&path, bless)));
    }

    Ok(trials)
}

fn collect_elixir_trials() -> anyhow::Result<Vec<Trial>> {
    let ui_root = Path::new(UI_ROOT);
    if !ui_root.is_dir() {
        anyhow::bail!("{UI_ROOT} does not exist");
    }

    let mut trials = Vec::new();
    for entry in WalkDir::new(ui_root) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }

        let exs_path = entry.into_path();
        if !exs_path.extension().is_some_and(|ext| ext == "exs") {
            continue;
        }

        let pt_path = exs_path.with_extension("pt");
        if !pt_path.is_file() {
            anyhow::bail!("elixir test {} has no matching .pt file", exs_path.display());
        }

        let rel =
            exs_path.strip_prefix(ui_root).expect("walkdir paths are under ui_root").to_path_buf();

        let test_name = format!("{}::run", rel.display());
        trials.push(Trial::test(test_name, move || runner::run_exs(&pt_path, &exs_path)));
    }

    Ok(trials)
}
