use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=src/patina_compile_server.erl");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let erl_src = "src/patina_compile_server.erl";

    match Command::new("erlc").args(["-o", &out_dir, erl_src]).output() {
        Ok(output) => {
            if !output.status.success() {
                println!(
                    "cargo::error=erlc failed to compile {erl_src}: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => {
            println!("cargo::error=failed to run erlc (is Erlang/OTP installed?): {e}");
        }
    }
}
