#![feature(rustc_private)]

mod compile;
mod erlang;
mod num_utils;
mod package;

use std::path::{Path, PathBuf};
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
#[command(name = "patina", about = "Compile Patina packages.")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Compile a single source file as a package.
    Compile {
        /// Source file to compile.
        file: PathBuf,

        /// Package name (defaults to PascalCase of the source filename).
        #[arg(long, value_name = "NAME")]
        pkg_name: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Run a compile server to be used with Patina's `--compile-server` option.
    ///
    /// The server's socket path is printed as the first line on stdout.
    Serve,
}

/// Options for compiling a package.
#[derive(clap::Args, Debug)]
struct CommonArgs {
    /// Emit compiled output.
    #[arg(long, value_name = "STAGE")]
    emit: Vec<EmitStage>,

    /// Dump intermediate compiler output.
    /// `--dump` differs from `--emit` in that its outputs are diagnostics
    /// only, and there's no guarantees about their stability.
    #[arg(long, value_name = "STAGE")]
    dump: Vec<DumpStage>,

    /// Output directory.
    #[arg(long, value_name = "DIR", default_value = ".")]
    out_dir: PathBuf,

    /// Path to a running compile server's socket. If not given, patina starts a temporary one.
    #[arg(long, value_name = "SOCKET_PATH")]
    compile_server: Option<PathBuf>,

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

    match args.command {
        Command::Compile { file, pkg_name, common } => run(common, &file, pkg_name.as_deref()),
        Command::Serve => match compile::compile_server::serve() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: failed to run compile server: {e}");
                ExitCode::FAILURE
            }
        },
    }
}

fn run(args: CommonArgs, file: &Path, pkg_name: Option<&str>) -> ExitCode {
    let server =
        compile::compile_server::SyncServer::start(compile::compile_server::SyncServerOpts {
            sock_path: args.compile_server.clone(),
        });

    let stages = args
        .dump
        .into_iter()
        .map(Stage::from)
        .chain(args.emit.into_iter().map(Stage::from))
        .collect::<Vec<_>>();

    let stages = if stages.is_empty() { vec![Stage::Meta, Stage::Beam] } else { stages };

    let src = match std::fs::read_to_string(file) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("error reading {}: {e}", file.display());
            return ExitCode::FAILURE;
        }
    };

    let pkg_name = match package::resolve_package_name(pkg_name, file) {
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
        compile::compile(&stages, file, &src, &args.out_dir, &pkg_name, &args.externs, &server)
    }) {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
