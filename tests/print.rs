mod common;

use common::{assert_success, install_root, patina_command};
use serde_json::{Value, json};

const GREETER: &str = "[package]\nname = \"greeter\"\nversion = \"0.1.0\"\n";

fn print(manifest: &str, infos: &[&str]) -> Value {
    let dir = tempfile::tempdir().expect("create temp dir");
    std::fs::write(dir.path().join("Patina.toml"), manifest).expect("write manifest");

    let output = patina_command()
        .arg("-C")
        .arg(dir.path())
        .args(["print", "--format", "json"])
        .args(infos)
        .output()
        .expect("run patina");

    assert_success(&output);
    serde_json::from_slice(&output.stdout).expect("print's output is JSON")
}

#[test]
fn package_defaults_are_filled_in() {
    let expected = json!({"package": {
        "name": "greeter", "version": "0.1.0", "otp_app_name": "greeter", "description": "",
    }});
    assert_eq!(print(GREETER, &["package"]), expected);
}

#[test]
fn dependency_defaults_are_filled_in() {
    let manifest = format!(
        "{GREETER}[dependencies]\n\
         hello_lib = \"~> 0.1\"\n\
         renamed = {{ otp_app_name = \"renamed_app\" }}\n\
         json = {{ git = \"https://example.com/json.git\" }}\n\
         local = {{ path = \"/local\" }}\n"
    );

    let expected = json!({"dependencies": [
        {"name": "hello_lib", "otp_app_name": "hello_lib", "version": "~> 0.1",
         "source": {"hex": "hello_lib"}},
        {"name": "renamed", "otp_app_name": "renamed_app", "version": null,
         "source": {"hex": "renamed_app"}},
        {"name": "json", "otp_app_name": "json", "version": null,
         "source": {"git": {"url": "https://example.com/json.git", "ref": null}}},
        {"name": "local", "otp_app_name": "local", "version": null,
         "source": {"path": "/local"}},
    ]});

    assert_eq!(print(&manifest, &["dependencies"]), expected);
}

#[test]
fn version_and_install_root_dont_read_the_package() {
    let expected = json!({
        "version": env!("CARGO_PKG_VERSION"),
        "install_root": install_root().to_str().unwrap(),
    });

    assert_eq!(print("not a manifest", &["version", "install_root"]), expected);
}

#[test]
fn install_root_is_absolute() {
    let root = install_root();
    let output = patina_command()
        .current_dir(root.parent().unwrap())
        .env("PATINA_ROOT", root.file_name().unwrap())
        .args(["print", "--format", "json", "install_root"])
        .output()
        .expect("run patina");

    assert_success(&output);
    let output: Value = serde_json::from_slice(&output.stdout).expect("print's output is JSON");
    assert_eq!(output, json!({"install_root": root.to_str().unwrap()}));
}
