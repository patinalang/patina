mod common;

use common::{Session, assert_failure};

const LIB: &str = "pub fn f() {}\n";

/// Writes `greeter`, which calls `hello_lib::f()`.
fn greeter(session: &Session, dependencies: &str) {
    session.package("greeter", dependencies, "pub fn g() { hello_lib::f() }\n");
}

/// Writes the package `name`, and builds it into `out_dir`.
fn build_lib(session: &Session, name: &str, out_dir: &str) {
    session.build_new_package(name, "", LIB, &["--out-dir", out_dir]);
}

#[test]
fn dependency_is_found_by_package_name() {
    let session = Session::new();
    build_lib(&session, "hello_lib", "../lib");
    greeter(&session, r#"hello_lib = "~> 0.1""#);
    session.build_package("greeter", &["-L", "../lib"]);
}

#[test]
fn missing_dependency_is_reported() {
    let session = Session::new();
    greeter(&session, r#"hello_lib = "~> 0.1""#);
    let output = session.try_build_package("greeter", &[]);
    assert_failure(&output, "can't find package for `hello_lib`");
}

#[test]
fn undeclared_package_is_not_a_dependency() {
    let session = Session::new();
    build_lib(&session, "hello_lib", "../lib");
    greeter(&session, "");
    let output = session.try_build_package("greeter", &["-L", "../lib"]);
    assert_failure(&output, "use of unresolved module or unlinked package `hello_lib`");
}

#[test]
fn core_dependency_is_rejected() {
    let session = Session::new();
    greeter(&session, r#"core = "~> 0.1""#);
    let output = session.try_build_package("greeter", &[]);
    assert_failure(&output, "core cannot be named as an explicit dependency");
}

#[test]
fn applications_follow_the_manifest() {
    let session = Session::new();
    for name in ["zeta", "alpha"] {
        build_lib(&session, name, "../lib");
    }
    let dependencies = "zeta = \"~> 0.1\"\nalpha = \"~> 0.1\"";
    session.build_new_package("greeter", dependencies, LIB, &["-L", "../lib"]);

    let app = session.path().join("greeter/target/debug/greeter/ebin/greeter.app");
    let app = std::fs::read_to_string(app).expect("read .app");
    // Ignore how the Erlang pretty printer breaks the list into lines.
    let app: String = app.split_whitespace().collect();
    let applications = "{applications,[kernel,stdlib,patina_core,zeta,alpha]}";
    assert!(app.contains(applications), "got:\n{app}");
}
