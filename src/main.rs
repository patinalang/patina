#![feature(rustc_private)]

mod compile;
mod erlang;

use std::process::ExitCode;

use clap::Parser;
use rustc_errors::catch_fatal_errors;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum DumpStage {
    Tokens,
    Ast,
    Expanded,
    Typed,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum EmitStage {}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Stage {
    Dump(DumpStage),
    Emit(EmitStage),
}

#[derive(Parser, Debug)]
#[command(name = "patina", about = "Compile Patina source files.")]
struct Args {
    /// Dump intermediate compiler output to stdout.
    #[arg(long, value_name = "STAGE", conflicts_with = "emit")]
    dump: Option<DumpStage>,

    /// Emit compiled output.
    #[arg(long, value_name = "STAGE", conflicts_with = "dump")]
    emit: Option<EmitStage>,

    /// Source file to compile.
    file: std::path::PathBuf,

    /// Output directory.
    #[arg(long, value_name = "DIR", default_value = ".")]
    out_dir: std::path::PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let stage = args
        .dump
        .map(Stage::Dump)
        .or_else(|| args.emit.map(Stage::Emit))
        .unwrap_or(Stage::Dump(DumpStage::Typed));

    let src = match std::fs::read_to_string(&args.file) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("error reading {}: {e}", args.file.display());
            return ExitCode::FAILURE;
        }
    };

    match catch_fatal_errors(|| compile::compile(stage, &args.file, &src, &args.out_dir)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
