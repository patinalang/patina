mod common;

use common::{Session, assert_failure, assert_success};

#[test]
fn dependency_change_is_seen() {
    let session = Session::new();
    let lib = ["--out-dir", "../lib"];
    session.build_new_package("hello_lib", "", "pub fn f() -> int { 1 }\n", &lib);
    let greeter = "pub fn g() -> int { hello_lib::f() }\n";
    session.build_new_package("greeter", r#"hello_lib = "~> 0.1""#, greeter, &["-L", "../lib"]);

    session.build_new_package("hello_lib", "", "pub fn f() -> str { \"one\" }\n", &lib);
    let output = session.try_build_package("greeter", &["-L", "../lib"]);
    assert_failure(&output, "mismatched types");
}

#[test]
fn warnings_are_replayed_from_the_cache() {
    let session = Session::new();
    session.package("greeter", "", "fn unused() {}\n");
    for _ in 0..2 {
        let output = session.try_build_package("greeter", &[]);
        assert_success(&output);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("function `unused` is never used"), "got:\n{stderr}");
    }
}

fn unique_id_derive(const_name: &str) -> String {
    format!(
        r#"use core::proc_macro::*;

extern mod erlang;

#[proc_macro_derive(UniqueId)]
pub fn unique_id(_input: TokenStream) -> TokenStream {{
    let id = unsafe {{ erlang::unique_integer([a"positive"]).cast_unchecked() }};
    TokenStream::from_trees([
        ident("pub"), ident("const"), ident("{const_name}"), punct(':'), ident("int"), punct('='),
        TokenTree::Literal(Literal::int(id)), punct(';'),
    ])
}}

fn ident(name: str) -> TokenTree {{
    TokenTree::Ident(Ident::new(name, Span::call_site()))
}}

fn punct(ch: char) -> TokenTree {{
    TokenTree::Punct(Punct::new(ch, Spacing::Alone))
}}
"#
    )
}

#[test]
fn derive_reruns_only_when_its_package_changes() {
    let session = Session::new();
    let expand_user = || {
        let user = "use provider::UniqueId;\n\n#[derive(UniqueId)]\npub struct S;\n";
        let args = ["-L", "lib", "--extern", "provider", "--dump", "expanded", "--emit", "beam"];
        session.build("user.pt", user, &args);
        let expanded = session.path().join("target/debug/user/ebin/user.expanded");
        std::fs::read_to_string(expanded).expect("read expansion")
    };

    session.build("provider.pt", &unique_id_derive("ID"), &["--out-dir", "lib"]);
    let first = expand_user();
    assert_eq!(expand_user(), first, "the derive ran again with nothing changed");

    session.build("provider.pt", &unique_id_derive("NEW_ID"), &["--out-dir", "lib"]);
    let changed = expand_user();
    assert!(
        changed.contains("NEW_ID"),
        "the derive didn't run after its package changed:\n{changed}"
    );
}

#[test]
fn incremental_dir_can_be_elsewhere() {
    let session = Session::new();
    session.build_new_package("greeter", "", "pub fn f() {}\n", &["--incremental-dir", "cache"]);
    assert!(session.path().join("greeter/cache/greeter").is_dir());
    assert!(!session.path().join(GREETER_EBIN).join(".patina_incremental").exists());
}

/// The default output directory of the `greeter` package.
const GREETER_EBIN: &str = "greeter/target/debug/greeter/ebin";

const GREETER: &str = "\
pub mod a {
    pub fn f() -> int { 1 }
}

pub mod b {
    pub fn f() -> int { 2 }
}
";

#[cfg(unix)]
fn beam_inode(session: &Session, module: &str) -> u64 {
    use std::os::unix::fs::MetadataExt;
    let beam =
        session.path().join(GREETER_EBIN).join(format!("Elixir.Patina.Greeter.{module}.beam"));
    std::fs::metadata(beam).expect("the beam exists").ino()
}

#[cfg(unix)]
#[test]
fn only_changed_modules_are_recompiled() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let (a, b) = (beam_inode(&session, "A"), beam_inode(&session, "B"));

    session.build_new_package("greeter", "", &GREETER.replace("2", "3"), &[]);
    assert_eq!(beam_inode(&session, "A"), a);
    assert_ne!(beam_inode(&session, "B"), b);
}

#[cfg(unix)]
#[test]
fn module_whose_lines_moved_is_recompiled() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let (a, b) = (beam_inode(&session, "A"), beam_inode(&session, "B"));

    session.build_new_package("greeter", "", &GREETER.replace("pub mod b", "\npub mod b"), &[]);
    assert_eq!(beam_inode(&session, "A"), a);
    assert_ne!(beam_inode(&session, "B"), b);
}

#[cfg(unix)]
#[test]
fn build_without_beams_is_not_remembered() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let b = beam_inode(&session, "B");

    session.build_new_package("greeter", "", &GREETER.replace("2", "3"), &["--dump", "erl"]);
    session.build_package("greeter", &[]);
    assert_ne!(beam_inode(&session, "B"), b);
}

#[test]
fn deleted_beam_is_restored() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let beam = session.path().join(GREETER_EBIN).join("Elixir.Patina.Greeter.A.beam");
    let built = std::fs::read(&beam).expect("read the beam");

    std::fs::remove_file(&beam).expect("delete the beam");
    session.build_package("greeter", &[]);
    assert!(std::fs::read(&beam).expect("read the beam") == built);
}

#[test]
fn no_hidden_temporary_files_are_left() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    std::fs::remove_file(session.path().join(GREETER_EBIN).join("Elixir.Patina.Greeter.A.beam"))
        .expect("delete");
    session.build_new_package("greeter", "", &GREETER.replace("2", "3"), &[]);

    let entries =
        std::fs::read_dir(session.path().join(GREETER_EBIN)).expect("read the output directory");
    let hidden: Vec<_> = entries
        .map(|entry| entry.expect("read an entry").file_name())
        .filter(|name| name.to_string_lossy().starts_with('.') && name != ".patina_incremental")
        .collect();
    assert!(hidden.is_empty(), "left behind: {hidden:?}");
}

#[cfg(unix)]
fn metadata_inode(session: &Session) -> u64 {
    use std::os::unix::fs::MetadataExt;
    let rmeta = session.path().join(GREETER_EBIN).join("libgreeter.rmeta");
    std::fs::metadata(rmeta).expect("the .rmeta exists").ino()
}

#[cfg(unix)]
#[test]
fn unchanged_metadata_is_linked_from_the_cache() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let rmeta = metadata_inode(&session);

    session.build_package("greeter", &[]);
    assert_eq!(metadata_inode(&session), rmeta);
}

#[cfg(unix)]
#[test]
fn changed_metadata_is_encoded_again() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);
    let rmeta = metadata_inode(&session);

    session.build_new_package("greeter", "", &GREETER.replace("2", "3"), &[]);
    assert_ne!(metadata_inode(&session), rmeta);
}

#[test]
fn removed_module_beam_is_deleted_even_if_the_cache_is_discarded() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);

    // Changed options make incremental mechanism ignore the previous session.
    session.build_new_package("greeter", "", "pub mod a {}\n", &["-A", "dead_code"]);
    assert!(!session.path().join(GREETER_EBIN).join("Elixir.Patina.Greeter.B.beam").exists());
}

#[test]
fn clean_removes_everything_build_wrote() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &["--out-dir", "../out"]);

    session.clean_package("greeter", &["--out-dir", "../out"]);
    let out = session.path().join("out");
    let left: Vec<_> = std::fs::read_dir(&out).expect("read out").collect();
    assert_eq!(left.len(), 1, "left behind: {left:?}");
    // The incremental dir is shared by the packages built into `out`, so it's kept.
    let incremental: Vec<_> =
        std::fs::read_dir(out.join(".patina_incremental")).expect("read incremental").collect();
    assert!(incremental.is_empty(), "left behind: {incremental:?}");
}

#[test]
fn clean_keeps_other_files_in_the_out_dir() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &["--out-dir", "../out"]);
    session.build_new_package("hello_lib", "", GREETER, &["--out-dir", "../out"]);
    std::fs::write(session.path().join("out/foreign.beam"), "").expect("write a foreign beam");

    session.clean_package("greeter", &["--out-dir", "../out"]);
    let out = session.path().join("out");
    assert!(out.join("foreign.beam").exists());
    assert!(out.join("Elixir.Patina.HelloLib.A.beam").exists());
    assert!(out.join("libhello_lib.rmeta").exists());
    assert!(out.join(".patina_incremental/hello_lib").is_dir());
}

#[test]
fn app_file_is_kept_by_a_build_that_does_not_emit_it() {
    let session = Session::new();
    session.build_new_package("greeter", "", GREETER, &[]);

    session.build_package("greeter", &["--emit", "meta", "--emit", "beam"]);
    let app = session.path().join(GREETER_EBIN).join("greeter.app");
    assert!(app.exists());

    // It's still the build's, so `clean` removes it.
    session.clean_package("greeter", &[]);
    assert!(!app.exists());
}

#[test]
fn default_out_dir_is_named_after_the_otp_app() {
    let session = Session::new();
    session.package("greeter", "", GREETER);
    let manifest = "[package]\nname = \"greeter\"\notp_app_name = \"hello\"\nversion = \"0.1.0\"\n";
    std::fs::write(session.path().join("greeter/Patina.toml"), manifest).expect("write manifest");

    session.build_package("greeter", &[]);
    assert!(session.path().join("greeter/target/debug/hello/ebin/hello.app").exists());
}
