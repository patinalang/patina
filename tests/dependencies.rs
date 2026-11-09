mod common;

use common::{Session, assert_failure};

const LIB: &str = "pub fn f() {}\n";

/// Writes `Greeter`, which calls `HelloLib::f()`.
fn greeter(session: &Session, dependencies: &str) {
    session.package("Greeter", dependencies, "pub fn g() { HelloLib::f() }\n");
}

/// Writes the package `name`, and builds it into `out_dir`.
fn build_lib(session: &Session, name: &str, out_dir: &str) {
    session.package(name, "", LIB);
    session.build_package(name, &["--out-dir", out_dir]);
}

#[test]
fn dependency_is_found_by_package_name() {
    let session = Session::new();
    build_lib(&session, "HelloLib", "lib");
    greeter(&session, r#"HelloLib = "~> 0.1""#);
    session.build_package("Greeter", &["-L", "lib"]);
}

#[test]
fn missing_dependency_is_reported() {
    let session = Session::new();
    greeter(&session, r#"HelloLib = "~> 0.1""#);
    let output = session.try_build_package("Greeter", &[]);
    assert_failure(&output, "can't find crate for `HelloLib`");
}

#[test]
fn undeclared_package_is_not_a_dependency() {
    let session = Session::new();
    build_lib(&session, "HelloLib", "lib");
    greeter(&session, "");
    let output = session.try_build_package("Greeter", &["-L", "lib"]);
    assert_failure(&output, "use of undeclared type `HelloLib`");
}

#[test]
fn core_dependency_is_rejected() {
    let session = Session::new();
    greeter(&session, r#"Core = "~> 0.1""#);
    let output = session.try_build_package("Greeter", &[]);
    assert_failure(&output, "Core cannot be named as an explicit dependency");
}

#[test]
fn applications_follow_the_manifest() {
    let session = Session::new();
    for name in ["Zeta", "Alpha"] {
        build_lib(&session, name, "lib");
    }
    session.package("Greeter", "Zeta = \"~> 0.1\"\nAlpha = \"~> 0.1\"", LIB);
    session.build_package("Greeter", &["-L", "lib"]);

    let app = std::fs::read_to_string(session.path().join("greeter.app")).expect("read .app");
    // Ignore how the Erlang pretty printer breaks the list into lines.
    let app: String = app.split_whitespace().collect();
    let applications = "{applications,[kernel,stdlib,patina_core,zeta,alpha]}";
    assert!(app.contains(applications), "got:\n{app}");
}
