mod common;

use common::{CONSUMER, Session, provider};

/// Expands `CONSUMER` with `args`, and checks that `num!()` gave `value`.
#[track_caller]
fn assert_num_expands_to(session: &Session, args: &[&str], value: u32) {
    let args = [&["--dump", "expanded"], args].concat();
    session.build("consumer.pt", CONSUMER, &args);

    let expanded = session.path().join("target/debug/consumer/ebin/consumer.expanded");
    let expanded = std::fs::read_to_string(expanded).expect("read expanded");
    assert!(expanded.contains(&format!("let _x = {value};")), "got:\n{expanded}");
}

fn assert_rebuild_is_picked_up(first_out: &str, second_out: &str) {
    let session = Session::new();

    session.build("provider.pt", &provider(42), &["--out-dir", first_out]);
    assert_num_expands_to(&session, &["-L", first_out, "--extern", "provider"], 42);

    session.build("provider.pt", &provider(43), &["--out-dir", second_out]);
    assert_num_expands_to(&session, &["-L", second_out, "--extern", "provider"], 43);
}

#[test]
fn rebuilt_proc_macro_is_reloaded_in_a_running_vm() {
    assert_rebuild_is_picked_up("out", "out");
}

#[test]
fn proc_macro_rebuilt_elsewhere_is_reloaded_in_a_running_vm() {
    assert_rebuild_is_picked_up("first_out", "second_out");
}

/// A running server keeps the code path from earlier compiles, which mustn't shadow this one's.
#[test]
fn later_compile_code_path_wins_in_a_running_vm() {
    let session = Session::new();
    session.build("provider.pt", &provider(42), &["--out-dir", "earlier"]);
    session.build("provider.pt", &provider(43), &["--out-dir", "later"]);

    // Puts `earlier` on the code path, without running the macro and so loading its module.
    session.build("unused.pt", "pub fn f() {}\n", &["-L", "earlier", "--extern", "provider"]);

    assert_num_expands_to(&session, &["-L", "later", "--extern", "provider"], 43);
}

#[test]
fn relative_extern_path_reaches_the_compile_server() {
    let session = Session::new();
    session.build("provider.pt", &provider(42), &["--out-dir", "out"]);
    session.build("consumer.pt", CONSUMER, &["--extern", "provider=out/libprovider.rmeta"]);
}
