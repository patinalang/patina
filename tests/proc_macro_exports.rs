#![feature(rustc_private)]

use std::path::Path;
use std::process::{Command, Output};

fn run(source: &Path, out_dir: &Path, package: &str, externs: &[(&str, Option<&Path>)]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_patina"));
    command
        .arg("--emit")
        .arg("meta")
        .arg("--emit")
        .arg("beam")
        .arg("--out-dir")
        .arg(out_dir)
        .arg("--pkg-name")
        .arg(package);
    for (name, path) in externs {
        command.arg("--extern").arg(match path {
            Some(path) => format!("{name}={}", path.display()),
            None => (*name).to_string(),
        });
    }
    command.arg(source);
    command.output().expect("run patina")
}

fn compile(source: &Path, out_dir: &Path, package: &str, externs: &[(&str, Option<&Path>)]) {
    let output = run(source, out_dir, package, externs);
    assert!(
        output.status.success(),
        "failed to compile {}:\nstdout:\n{}\nstderr:\n{}",
        source.display(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn facade_reports_reexported_proc_macro() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let out_dir = temp.path().join("out");
    std::fs::create_dir_all(&out_dir).expect("create output dir");

    let provider_source = temp.path().join("provider.pt");
    std::fs::write(
        &provider_source,
        r#"
use ProcMacro::*;

#[proc_macro]
pub fn identity(input: TokenStream) -> TokenStream {
    input
}
"#,
    )
    .expect("write provider source");
    compile(&provider_source, &out_dir, "Provider", &[("ProcMacro", None)]);

    let provider_rmeta = out_dir.join("libProvider.rmeta");
    let proc_macro_rmeta = out_dir.join("libProcMacro.rmeta");
    let facade_source = temp.path().join("facade.pt");
    std::fs::write(&facade_source, "pub use Provider::identity;\n").expect("write facade source");
    compile(
        &facade_source,
        &out_dir,
        "Facade",
        &[("Provider", Some(&provider_rmeta)), ("ProcMacro", Some(&proc_macro_rmeta))],
    );

    let facade_rmeta = out_dir.join("libFacade.rmeta");
    rustc_span::create_default_session_globals_then(|| {
        assert!(rustc_metadata::locator::exports_proc_macros(&provider_rmeta));
        assert!(rustc_metadata::locator::exports_proc_macros(&facade_rmeta));
    });
}

#[test]
fn wrapper_fields_are_private_across_packages() {
    let temp = tempfile::tempdir().expect("create temp dir");
    let out_dir = temp.path().join("out");
    std::fs::create_dir_all(&out_dir).expect("create output dir");

    let source = temp.path().join("bad_access.pt");
    std::fs::write(
        &source,
        r#"
use ProcMacro::*;

pub fn main() {
    let _literal = Literal::int(1).0;
    let _group = Group::new(Delimiter::Brace, TokenStream::new()).0;
    let _ident = Ident::new("x", Span::call_site()).0;
    let _punct = Punct::new('+', Spacing::Alone).0;
    let _kind = ProcMacro::Bridge::LitKind::Integer;
}
"#,
    )
    .expect("write source");

    let output = run(&source, &out_dir, "BadAccess", &[("ProcMacro", None)]);
    assert!(
        !output.status.success(),
        "accessing wrapper fields should not compile:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.matches("private").count() >= 4,
        "expected one privacy error per field access, got:\n{stderr}"
    );
    assert!(
        stderr.contains("module `Bridge` is private"),
        "expected the bridge structs to be private, got:\n{stderr}"
    );
}
