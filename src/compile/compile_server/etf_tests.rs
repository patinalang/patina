use std::sync::{Arc, OnceLock};

use super::{SyncServer, SyncServerOpts};

fn server() -> &'static Arc<SyncServer> {
    static SERVER: OnceLock<Arc<SyncServer>> = OnceLock::new();
    SERVER.get_or_init(|| SyncServer::start(SyncServerOpts::default()))
}

/// Encodes `term`, echoes it through the VM, and returns what comes back.
fn echo(term: &[u8]) -> Vec<u8> {
    server()
        .try_with(|server| server.echo(term).expect("echo through the VM"))
        .expect("start the compile server")
}

/// A term the VM must hand back byte for byte.
#[track_caller]
fn assert_stable(what: &str, term: Vec<u8>) {
    assert_eq!(echo(&term), term, "{what} did not survive a round trip through OTP");
}

fn term(build: impl FnOnce(&mut Vec<u8>)) -> Vec<u8> {
    let mut bytes = vec![etf::TERM_HEADER];
    build(&mut bytes);
    bytes
}

#[test]
fn atoms_binaries_and_booleans_round_trip() {
    assert_stable("an atom", term(|b| etf::write_atom(b, "group").unwrap()));
    assert_stable("an empty atom", term(|b| etf::write_atom(b, "").unwrap()));
    assert_stable("a non-ascii atom", term(|b| etf::write_atom(b, "identyfikator").unwrap()));
    assert_stable("a binary", term(|b| etf::write_binary(b, b"hello").unwrap()));
    assert_stable("an empty binary", term(|b| etf::write_binary(b, b"").unwrap()));
    assert_stable(
        "a binary of non-utf8 bytes",
        term(|b| etf::write_binary(b, &[0xff, 0x00, 0xfe]).unwrap()),
    );
    assert_stable("true", term(|b| etf::write_bool(b, true).unwrap()));
    assert_stable("false", term(|b| etf::write_bool(b, false).unwrap()));
}

/// Integers are the one place our encoding has to pick a width, and the VM
/// re-encodes each value in its own smallest form. Values that round-trip
/// unchanged are the ones where the two agree.
#[test]
fn integers_round_trip_in_the_vm_s_own_encoding() {
    for value in [0i64, 1, 255, 256, -1, -256, i64::from(i32::MAX), i64::from(i32::MIN)] {
        let encoded = term(|b| etf::write_integer(b, value).unwrap());
        let echoed = echo(&encoded);
        // Read our own encoding and the VM's back; the value is what has to
        // match, not the byte-level choice of representation.
        let mut ours = etf::Reader::new(&encoded);
        let mut theirs = etf::Reader::new(&echoed);
        assert_eq!(
            ours.read_integer().unwrap().as_i64().unwrap(),
            theirs.read_integer().unwrap().as_i64().unwrap(),
            "value {value} changed on its way through OTP"
        );
    }
}

/// Where fixed-width integers stop: at `i64::MAX` both sides still read the
/// same value, and one past it OTP produces a bignum we deliberately reject.
#[test]
fn integer_boundary_agrees_with_the_vm() {
    let at_max = term(|b| etf::write_integer(b, i128::from(i64::MAX)).unwrap());
    let echoed = echo(&at_max);
    assert_eq!(
        etf::Reader::new(&echoed).read_integer().unwrap().as_i64().unwrap(),
        i64::MAX,
        "i64::MAX must survive a round trip"
    );

    let past_max = term(|b| etf::write_integer(b, i128::from(i64::MAX) + 1).unwrap());
    let echoed = echo(&past_max);
    assert!(
        etf::Reader::new(&echoed).read_integer().unwrap().as_i64().is_err(),
        "a value past i64::MAX must come back as a bignum we reject"
    );
}

/// Lists are how a token stream is carried, including the empty one, which has
/// its own representation.
#[test]
fn lists_round_trip() {
    let empty = term(|b| etf::write_nil(b).unwrap());
    assert_stable("the empty list", empty);

    let list = term(|b| {
        etf::write_list_header(b, 2).unwrap();
        etf::write_atom(b, "a").unwrap();
        etf::write_atom(b, "b").unwrap();
        etf::write_nil(b).unwrap();
    });
    assert_stable("a list of atoms", list);

    // A zero-length `LIST_EXT` is a list the VM normalises to `NIL_EXT`, so
    // this one is compared by value rather than by bytes.
    let zero_length = [etf::TERM_HEADER, etf::LIST_EXT, 0, 0, 0, 0, etf::NIL_EXT];
    let echoed = echo(&zero_length);
    assert!(
        matches!(
            etf::Reader::new(&echoed).read_list_header().unwrap(),
            etf::ListHeader::Empty | etf::ListHeader::Proper(0)
        ),
        "a zero-length list must come back empty, got {echoed:?}"
    );
}

/// A token tree is a tagged tuple, and a token stream is a list of them nested
/// to arbitrary depth. This is that shape, end to end.
#[test]
fn nested_tagged_tuples_round_trip() {
    let tree = term(|b| {
        // `[{group, parenthesis, [{ident, <<"x">>, false, 0}], 0, 0, 0}]`
        etf::write_list_header(b, 1).unwrap();
        etf::write_tuple_header(b, 6).unwrap();
        etf::write_atom(b, "group").unwrap();
        etf::write_atom(b, "parenthesis").unwrap();
        etf::write_list_header(b, 1).unwrap();
        etf::write_tuple_header(b, 4).unwrap();
        etf::write_atom(b, "ident").unwrap();
        etf::write_binary(b, b"x").unwrap();
        etf::write_bool(b, false).unwrap();
        etf::write_integer(b, 0i64).unwrap();
        etf::write_nil(b).unwrap();
        etf::write_integer(b, 0i64).unwrap();
        etf::write_integer(b, 1i64).unwrap();
        etf::write_integer(b, 2i64).unwrap();
        etf::write_nil(b).unwrap();
    });
    assert_stable("a nested token tree", tree);
}

/// A tuple large enough to need `LARGE_TUPLE_EXT` rather than the small form.
#[test]
fn large_tuples_round_trip() {
    let large = term(|b| {
        etf::write_tuple_header(b, 300).unwrap();
        for i in 0..300 {
            etf::write_integer(b, i as i64).unwrap();
        }
    });
    assert_stable("a 300-element tuple", large);
}
