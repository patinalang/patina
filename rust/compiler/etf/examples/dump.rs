use etf::{TERM_HEADER, write_integer};

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(path).unwrap();
    let mut mismatches = 0;
    for line in data.lines() {
        let mut parts = line.split_whitespace();
        let value: i128 = parts.next().unwrap().parse().unwrap();
        let expected = parts.next().unwrap();
        let mut buf = vec![TERM_HEADER];
        write_integer(&mut buf, value).unwrap();
        let hex: String = buf.iter().map(|b| format!("{b:02X}")).collect();
        if hex != expected {
            mismatches += 1;
            println!("MISMATCH {value}\n  rust   {hex}\n  erlang {expected}");
        }
    }
    println!("done, {mismatches} mismatches");
}
