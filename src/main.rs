#![feature(rustc_private, iter_intersperse)]
#![cfg_attr(windows, feature(windows_unix_domain_sockets))]

mod clean;
mod compile;
mod erlang;
mod installation;
mod manifest;
mod new;
mod num_utils;
mod package;
mod print;

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use clap::builder::ArgPredicate;
use rustc_errors::emitter::HumanReadableErrorType;
use rustc_errors::{ColorConfig, FatalError, catch_fatal_errors};
use rustc_session::config::{ErrorOutputType, Externs};
use rustc_session::utils::CanonicalizedPath;

use crate::clean::CleanError;
use crate::compile::compile_server::{OtpVersionError, SpawnError};
use crate::installation::{Installation, LazyInstallation, LocateInstallationError};
use crate::manifest::{Manifest, ManifestError};
use crate::new::NewError;
use crate::package::{
    DuplicateExtern, ExternArg, ExternArgError, PackageInfo, PackageName, PackageNameError,
};
use crate::print::PrintError;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum DumpStage {
    Tokens,
    Ast,
    Expanded,
    Typed,
    Eaf,
    Abstr,
    Erl,
    Core0,
    Core,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum EmitStage {
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
    Eaf,
    Abstr,
    Erl,
    Core0,
    Core,
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
            DumpStage::Eaf => Stage::Eaf,
            DumpStage::Abstr => Stage::Abstr,
            DumpStage::Erl => Stage::Erl,
            DumpStage::Core0 => Stage::Core0,
            DumpStage::Core => Stage::Core,
        }
    }
}

impl From<EmitStage> for Stage {
    fn from(stage: EmitStage) -> Self {
        match stage {
            EmitStage::Beam => Stage::Beam,
            EmitStage::Meta => Stage::Meta,
            EmitStage::App => Stage::App,
        }
    }
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum ErrorFormat {
    /// Human-readable text.
    Pretty,
    /// One JSON object per line.
    Json,
}

impl From<ErrorFormat> for ErrorOutputType {
    fn from(format: ErrorFormat) -> Self {
        match format {
            ErrorFormat::Pretty => ErrorOutputType::default(),
            ErrorFormat::Json => ErrorOutputType::Json {
                pretty: false,
                json_rendered: HumanReadableErrorType { short: false, unicode: false },
                color_config: ColorConfig::Never,
            },
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "patina", about = "Compile Patina packages.")]
struct Args {
    /// Root of the Patina install. If not given, it's found relative to this executable.
    #[arg(long, env = "PATINA_ROOT", global = true, value_name = "DIR")]
    root: Option<PathBuf>,

    /// Change to DIR before doing anything, including resolving any paths.
    #[arg(short = 'C', global = true, value_name = "DIR")]
    directory: Option<PathBuf>,

    /// Format of the diagnostics written to stderr.
    #[arg(long, value_enum, global = true, value_name = "FORMAT", default_value = "pretty")]
    error_format: ErrorFormat,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    /// Compile the package in the current directory, as described by its `Patina.toml`.
    Build {
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

        /// Package name (defaults to the stem of the source filename).
        #[arg(long, value_name = "NAME")]
        pkg_name: Option<PackageName>,

        /// Package version. Required with `--emit app`.
        #[arg(long, value_name = "VERSION", required_if_eq("emit", "app"))]
        pkg_version: Option<String>,

        /// OTP application name.
        /// If not overridden, defaults to the package name.
        #[arg(long, value_name = "NAME")]
        otp_app_name: Option<String>,

        /// Emit compiled output.
        #[arg(
            long,
            value_name = "STAGE",
            default_values = ["meta", "beam"],
            default_value_if("dump", ArgPredicate::IsPresent, None)
        )]
        emit: Vec<EmitStage>,

        /// Package dependency as NAME=RMETA (may be repeated).
        /// If `=RMETA` is omitted, the package is assumed to be in the library
        /// search directories.
        #[arg(long = "extern", value_name = "NAME[=RMETA]")]
        extern_args: Vec<ExternArg>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Print information about the package or Patina itself.
    Print {
        #[arg(value_enum, value_name = "INFO", required = true)]
        prints: Vec<print::Print>,

        /// Output format.
        #[arg(long, value_enum)]
        format: print::Format,
    },

    /// Clean up the build artifacts & the incremental compilation files of `patina build`.
    Clean {
        #[command(flatten)]
        output: OutputArgs,
    },

    /// Create a new package.
    New {
        /// Directory to create the package in. It must not exist or be empty.
        dir: PathBuf,

        /// Package name. Can be omitted if `DIR` is already the package name.
        #[arg(long, value_name = "NAME")]
        pkg_name: Option<PackageName>,
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

    #[command(flatten)]
    output: OutputArgs,

    /// Add `DIR` to library search directories. Can be repeated.
    #[arg(short = 'L', long = "lib-dir", value_name = "DIR")]
    lib_dirs: Vec<PathBuf>,

    /// Path to a running compile server's socket. If not given, patina starts a temporary one.
    #[arg(long, value_name = "SOCKET_PATH")]
    compile_server: Option<PathBuf>,

    /// Allow the lint `LINT`. Can be repeated.
    #[arg(short = 'A', long = "allow-lint", value_name = "LINT")]
    allowed_lints: Vec<String>,

    /// Remap source paths starting with `FROM` to `TO` in diagnostics and outputs. Can be repeated.
    /// If not given, remaps `CWD()=""`.
    #[arg(long = "remap-path-prefix", value_name = "FROM=TO", value_parser = parse_remap_path_prefix)]
    remap_path_prefix: Vec<(PathBuf, PathBuf)>,
}

fn parse_remap_path_prefix(remap: &str) -> Result<(PathBuf, PathBuf), String> {
    let (from, to) = remap.rsplit_once('=').ok_or("must contain '=' between FROM and TO")?;
    Ok((from.into(), to.into()))
}

#[derive(clap::Args, Debug)]
struct OutputArgs {
    #[command(flatten)]
    out_dir: OutputDir,

    /// Directory for incremental compilation state (default: $out_dir/.patina_incremental).
    #[arg(long, value_name = "DIR")]
    incremental_dir: Option<PathBuf>,
}

impl OutputArgs {
    fn package_incremental_dir(&self, package: &PackageInfo) -> PathBuf {
        let incremental_dir = self.incremental_dir.clone().unwrap_or_else(|| {
            self.out_dir.resolve(&package.otp_app_name).join(".patina_incremental")
        });

        incremental_dir.join(package.name.to_string())
    }
}

#[derive(clap::Args, Debug)]
struct OutputDir {
    /// Output directory (default: target/debug/$otp_app_name/ebin).
    #[arg(long = "out-dir", value_name = "DIR")]
    dir: Option<PathBuf>,
}

impl OutputDir {
    fn resolve(&self, otp_app_name: &str) -> PathBuf {
        match &self.dir {
            Some(dir) => dir.clone(),
            None => Path::new("target").join("debug").join(otp_app_name).join("ebin"),
        }
    }
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
    DuplicateExtern(#[from] DuplicateExtern),
    #[error(transparent)]
    ExternArg(#[from] ExternArgError),
    #[error(transparent)]
    LocateInstallation(#[from] LocateInstallationError),
    #[error("failed to read {path}: {source}")]
    ReadSource {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to run compile server: {0}")]
    Serve(#[source] SpawnError),
    #[error("failed to get the current directory: {0}")]
    CurrentDir(#[source] io::Error),
    #[error("failed to change directories to {dir}: {source}")]
    ChangeDir {
        dir: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read OTP Version: {0}")]
    OtpVersionError(#[from] OtpVersionError),
    #[error(transparent)]
    Print(#[from] PrintError),
    #[error(transparent)]
    Clean(#[from] CleanError),
    #[error(transparent)]
    New(#[from] NewError),
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
    if let Some(dir) = args.directory {
        std::env::set_current_dir(&dir).map_err(|source| Error::ChangeDir { dir, source })?;
    }
    let base_dir = std::env::current_dir().map_err(Error::CurrentDir)?;

    let installation = LazyInstallation::new(args.root);
    let error_format = args.error_format;

    match args.command {
        Command::Build { emit, common } => {
            let manifest = Manifest::read(&base_dir)?;
            let file = manifest.source_path();
            let dependencies = manifest.dependency_names();
            let package = manifest.package();
            let externs = package.externs(dependencies.into_iter().map(ExternArg::from))?;

            run(RunArgs {
                common,
                error_format,
                emit,
                base_dir,
                file,
                package,
                externs,
                installation,
            })
        }
        Command::Compile {
            file,
            pkg_name,
            pkg_version,
            otp_app_name,
            emit,
            extern_args,
            common,
        } => {
            let name = package::resolve_package_name(pkg_name, &file)?;
            let package = PackageInfo::new(name, otp_app_name, pkg_version, None);
            for extern_arg in &extern_args {
                extern_arg.check_exists()?;
            }
            let externs = package.externs(extern_args)?;
            run(RunArgs {
                common,
                error_format,
                emit,
                base_dir,
                file,
                package,
                externs,
                installation,
            })
        }
        Command::Print { prints, format } => {
            print::print(format, &prints, &base_dir, installation)?;
            Ok(())
        }
        Command::Clean { output } => {
            let package = Manifest::read(&base_dir)?.package();
            let out_dir = output.out_dir.resolve(&package.otp_app_name);
            clean::clean(&out_dir, &output.package_incremental_dir(&package))?;
            Ok(())
        }
        Command::New { dir, pkg_name } => Ok(new::new(&dir, pkg_name)?),
        Command::Serve => compile::compile_server::serve(installation.get()?).map_err(Error::Serve),
    }
}

struct RunArgs {
    common: CommonArgs,
    error_format: ErrorFormat,
    emit: Vec<EmitStage>,
    base_dir: PathBuf,
    file: PathBuf,
    package: PackageInfo,
    externs: Externs,
    installation: LazyInstallation,
}

fn run(args: RunArgs) -> Result<(), Error> {
    let RunArgs { common, error_format, emit, base_dir, file, package, externs, installation } =
        args;

    if package.name.is_reserved() && !package::is_bootstrap() {
        return Err(PackageNameError::Reserved(package.name).into());
    }

    let installation: Installation = installation.try_into()?;
    let src = std::fs::read_to_string(&file)
        .map_err(|source| Error::ReadSource { path: file.clone(), source })?;

    let mut lib_dirs = common.lib_dirs;
    if !package::names_core_file(&externs) {
        lib_dirs.push(installation.core_dir.clone());
    }

    let server =
        compile::compile_server::SyncServer::start(common.compile_server.clone(), installation);

    let out_dir = common.output.out_dir.resolve(&package.otp_app_name);
    let incremental_dir = common.output.package_incremental_dir(&package);

    let remap_path_prefix = if common.remap_path_prefix.is_empty() {
        vec![(base_dir.clone(), PathBuf::new())]
    } else {
        common
            .remap_path_prefix
            .into_iter()
            .map(|(from, to)| {
                let from = CanonicalizedPath::new(base_dir.join(from)).canonicalized().clone();
                (from, to)
            })
            .collect()
    };

    let opts = compile::CompileOptions {
        base_dir,
        remap_path_prefix,
        otp_version: server.otp_version()?,
        out_dir,
        incremental_dir,
        lib_dirs,
        package,
        externs,
        allowed_lints: common.allowed_lints,
        error_format: error_format.into(),
    };

    let stages = common
        .dump
        .into_iter()
        .map(Stage::from)
        .chain(emit.into_iter().map(Stage::from))
        .collect::<Vec<_>>();

    catch_fatal_errors(|| compile::compile(&stages, &file, &src, &opts, &server))?;
    Ok(())
}
