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
    Core0,
    Core,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum EmitStage {
    Eaf,
    Abstr,
    Beam,
    Meta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Stage {
    Tokens,
    Ast,
    Expanded,
    Typed,
    Erl,
    Core0,
    Core,
    Eaf,
    Abstr,
    Beam,
    Meta,
}

impl From<DumpStage> for Stage {
    fn from(stage: DumpStage) -> Self {
        match stage {
            DumpStage::Tokens => Stage::Tokens,
            DumpStage::Ast => Stage::Ast,
            DumpStage::Expanded => Stage::Expanded,
            DumpStage::Typed => Stage::Typed,
            DumpStage::Erl => Stage::Erl,
            DumpStage::Core0 => Stage::Core0,
            DumpStage::Core => Stage::Core,
        }
    }
}

impl From<EmitStage> for Stage {
    fn from(stage: EmitStage) -> Self {
        match stage {
            EmitStage::Eaf => Stage::Eaf,
            EmitStage::Abstr => Stage::Abstr,
            EmitStage::Beam => Stage::Beam,
            EmitStage::Meta => Stage::Meta,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "patina", about = "Compile Patina source files.")]
struct Args {
    /// Emit compiled output.
    #[arg(long, value_name = "STAGE")]
    emit: Vec<EmitStage>,

    /// Dump intermediate compiler output.
    /// `--dump` differs from `--emit` in that its outputs are diagnostics
    /// only, and there's no guarantees about their stability.
    #[arg(long, value_name = "STAGE")]
    dump: Vec<DumpStage>,

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
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => err.exit(),
    };

    let server = compile::compile_server::SyncServer::start();

    let stages = args
        .dump
        .into_iter()
        .map(Stage::from)
        .chain(args.emit.into_iter().map(Stage::from))
        .collect::<Vec<_>>();

    let stages = if stages.is_empty() { vec![Stage::Meta, Stage::Beam] } else { stages };

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
        compile::compile(
            &stages,
            &args.file,
            &src,
            &args.out_dir,
            &pkg_name,
            &args.externs,
            &server,
        )
    }) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
