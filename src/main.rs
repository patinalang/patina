#![feature(rustc_private)]

mod compile;
mod erlang;
mod num_utils;
mod package;

use std::process::ExitCode;

use clap::Parser;
use rustc_errors::catch_fatal_errors;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum DumpStage {
    Tokens,
    Ast,
    Expanded,
    Typed,
    Erl,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum EmitStage {
    Eaf,
    Abstr,
    Beam,
}

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

    /// Package name (defaults to PascalCase of the source filename).
    #[arg(long, value_name = "NAME")]
    pkg_name: Option<String>,

    /// Package dependency as NAME=RMETA (may be repeated).
    /// If `=RMETA` is omitted, the package is assumed to be in the library
    /// search directories.
    #[arg(long = "extern", value_name = "NAME[=RMETA]")]
    externs: Vec<package::PackageDependency>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let stage = args
        .dump
        .map(Stage::Dump)
        .or_else(|| args.emit.map(Stage::Emit))
        .unwrap_or(Stage::Emit(EmitStage::Beam));

    let src = match std::fs::read_to_string(&args.file) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("error reading {}: {e}", args.file.display());
            return ExitCode::FAILURE;
        }
    };

    let pkg_name = match package::resolve_package_name(args.pkg_name.as_deref(), &args.file) {
        Ok(name) => name,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err) = package::validate_dependencies(&args.externs) {
        eprintln!("error: {err}");
        return ExitCode::FAILURE;
    }

    match catch_fatal_errors(|| {
        compile::compile(stage, &args.file, &src, &args.out_dir, &pkg_name, &args.externs)
    }) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
