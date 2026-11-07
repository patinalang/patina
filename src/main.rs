#![feature(rustc_private)]

mod compile;
mod erlang;
mod installation;
mod manifest;
mod num_utils;
mod package;

use std::io;
use std::path::{self, Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use clap::builder::{ArgPredicate, PathBufValueParser, TypedValueParser};
use rustc_errors::{FatalError, catch_fatal_errors};

use crate::compile::compile_server::SpawnError;
use crate::installation::{Installation, LocateInstallationError};
use crate::manifest::{Manifest, ManifestError};
use crate::package::{DependencyError, PackageDependency, PackageInfo, PackageNameError};

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
    App,
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
    App,
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
            EmitStage::App => Stage::App,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "patina", about = "Compile Patina packages.")]
struct Args {
    /// Root of the Patina install. If not given, it's found relative to this executable.
    #[arg(long, env = "PATINA_ROOT", global = true, value_name = "DIR", value_parser = absolute_path())]
    root: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Compile the package in DIR, as described by its `Patina.toml`.
    Build {
        /// Package directory.
        dir: PathBuf,

        /// Emit compiled output.
        #[arg(
            long,
            value_name = "STAGE",
            default_values = ["meta", "beam", "app"],
            default_value_if("dump", ArgPredicate::IsPresent, None)
        )]
        emit: Vec<EmitStage>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Compile a single source file as a package.
    Compile {
        /// Source file to compile.
        file: PathBuf,

        /// Package name (defaults to PascalCase of the source filename).
        #[arg(long, value_name = "NAME")]
        pkg_name: Option<String>,

        /// Package version. Required with `--emit app`.
        #[arg(long, value_name = "VERSION", required_if_eq("emit", "app"))]
        pkg_version: Option<String>,

        /// OTP application name.
        /// If not overridden, defaults to snake_case of the package name.
        #[arg(long, value_name = "NAME")]
        otp_application_name: Option<String>,

        /// Emit compiled output.
        #[arg(
            long,
            value_name = "STAGE",
            default_values = ["meta", "beam"],
            default_value_if("dump", ArgPredicate::IsPresent, None)
        )]
        emit: Vec<EmitStage>,

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
    /// Dump intermediate compiler output.
    /// `--dump` differs from `--emit` in that its outputs are diagnostics
    /// only, and there's no guarantees about their stability.
    /// Supplying `--dump` causes `--emit` to default to [] (nothing emitted by default).
    #[arg(long, value_name = "STAGE")]
    dump: Vec<DumpStage>,

    /// Output directory.
    #[arg(long, value_name = "DIR", default_value = ".", value_parser = absolute_path())]
    out_dir: PathBuf,

    /// Add `DIR` to library search directories. Can be repeated.
    #[arg(short = 'L', long = "lib-dir", value_name = "DIR", value_parser = absolute_path())]
    lib_dirs: Vec<PathBuf>,

    /// Path to a running compile server's socket. If not given, patina starts a temporary one.
    #[arg(long, value_name = "SOCKET_PATH")]
    compile_server: Option<PathBuf>,

    /// Package dependency as NAME=RMETA (may be repeated).
    /// If `=RMETA` is omitted, the package is assumed to be in the library
    /// search directories.
    #[arg(long = "extern", value_name = "NAME[=RMETA]")]
    externs: Vec<PackageDependency>,
}

fn absolute_path() -> impl TypedValueParser<Value = PathBuf> {
    PathBufValueParser::new().try_map(path::absolute::<PathBuf>)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    // Diagnostics are already reported.
    #[error("compilation failed")]
    Compile(#[from] FatalError),
    #[error(transparent)]
    Manifest(#[from] ManifestError),
    #[error(transparent)]
    PackageName(#[from] PackageNameError),
    #[error(transparent)]
    Dependency(#[from] DependencyError),
    #[error(transparent)]
    LocateInstallation(#[from] LocateInstallationError),
    #[error("failed to read {}: {source}", path.display())]
    ReadSource {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to run compile server: {0}")]
    Serve(#[source] SpawnError),
}

fn main() -> ExitCode {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => err.exit(),
    };

    match try_main(args) {
        Ok(()) => ExitCode::SUCCESS,
        // Error & diagnostics were already reported.
        Err(Error::Compile(_)) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn try_main(args: Args) -> Result<(), Error> {
    let installation = Installation::locate(args.root)?;
    match args.command {
        Command::Build { dir, emit, common } => {
            let manifest = Manifest::read(&dir)?;
            let file = manifest.source_path(&dir);
            let manifest = manifest.package;
            let name = package::resolve_package_name(Some(&manifest.name), &file)?;
            let package = PackageInfo {
                otp_application_name: manifest
                    .otp_application_name
                    .unwrap_or_else(|| package::snake_case(&name)),
                name,
                version: Some(manifest.version),
                description: manifest.description.unwrap_or_default(),
            };
            run(common, emit, &file, package, installation)
        }
        Command::Compile { file, pkg_name, pkg_version, otp_application_name, emit, common } => {
            let name = package::resolve_package_name(pkg_name.as_deref(), &file)?;
            let package = PackageInfo {
                otp_application_name: otp_application_name
                    .unwrap_or_else(|| package::snake_case(&name)),
                name,
                version: pkg_version,
                description: String::new(),
            };
            run(common, emit, &file, package, installation)
        }
        Command::Serve => compile::compile_server::serve(&installation).map_err(Error::Serve),
    }
}

fn run(
    args: CommonArgs,
    emit: Vec<EmitStage>,
    file: &Path,
    package: PackageInfo,
    installation: Installation,
) -> Result<(), Error> {
    let src = std::fs::read_to_string(file)
        .map_err(|source| Error::ReadSource { path: file.to_owned(), source })?;

    let mut lib_dirs = args.lib_dirs;
    lib_dirs.extend_from_slice(&installation.lib_dirs);

    let mut externs = args.externs;
    if !package.is_core() && !externs.iter().any(PackageDependency::is_core) {
        externs.push(PackageDependency::core_package_dependency());
    }
    package::validate_dependencies(&externs)?;

    let opts = compile::CompileOptions { out_dir: args.out_dir, lib_dirs, package, externs };

    let server =
        compile::compile_server::SyncServer::start(compile::compile_server::SyncServerOpts {
            sock_path: args.compile_server.clone(),
            installation,
        });

    let stages = args
        .dump
        .into_iter()
        .map(Stage::from)
        .chain(emit.into_iter().map(Stage::from))
        .collect::<Vec<_>>();

    catch_fatal_errors(|| compile::compile(&stages, file, &src, &opts, &server))?;
    Ok(())
}
