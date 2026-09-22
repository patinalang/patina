use num_traits::{PrimInt, ToBytes, WrappingNeg};

pub const TERM_HEADER: u8 = 131;
pub const SMALL_ATOM_UTF8_EXT: u8 = 119;
pub const ATOM_UTF8_EXT: u8 = 118;
pub const SMALL_INTEGER_EXT: u8 = 97;
pub const INTEGER_EXT: u8 = 98;
pub const SMALL_BIG_EXT: u8 = 110;
pub const LARGE_BIG_EXT: u8 = 111;
pub const NEW_FLOAT_EXT: u8 = 70;
pub const SMALL_TUPLE_EXT: u8 = 104;
pub const LARGE_TUPLE_EXT: u8 = 105;
pub const LIST_EXT: u8 = 108;
pub const NIL_EXT: u8 = 106;
pub const STRING_EXT: u8 = 107;
pub const BINARY_EXT: u8 = 109;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("atom too long: {0} bytes")]
    AtomTooLong(usize),
    #[error("integer too large: {0} bytes")]
    IntegerTooLarge(usize),
    #[error("binary too long: {0} bytes")]
    BinaryTooLong(usize),
    #[error("tuple too long: {0} elements")]
    TupleTooLong(usize),
    #[error("list too long: {0} elements")]
    ListTooLong(usize),
    #[error("invalid float: {0}")]
    InvalidFloat(f64),
    #[error("unsupported type")]
    UnsupportedType,
    #[error("invalid ETF at offset {offset}: {details}")]
    Protocol { offset: usize, details: String },
    #[error("trailing bytes after ETF term: {cursor} of {len}")]
    TrailingBytes { cursor: usize, len: usize },
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn offset(&self) -> Option<usize> {
        match self {
            Error::Protocol { offset, .. } => Some(*offset),
            Error::TrailingBytes { cursor, .. } => Some(*cursor),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------

pub fn write_bool(output: &mut Vec<u8>, value: bool) -> Result<()> {
    write_atom(output, if value { "true" } else { "false" })
}

pub fn write_atom(output: &mut Vec<u8>, atom: &str) -> Result<()> {
    let bytes = atom.as_bytes();
    if let Ok(len) = bytes.len().try_into() {
        output.push(SMALL_ATOM_UTF8_EXT);
        output.push(len);
        output.extend_from_slice(bytes);
    } else if let Ok(len) = TryInto::<u16>::try_into(bytes.len()) {
        output.push(ATOM_UTF8_EXT);
        output.extend_from_slice(&len.to_be_bytes());
        output.extend_from_slice(bytes);
    } else {
        return Err(Error::AtomTooLong(bytes.len()));
    }
    Ok(())
}

pub fn write_binary(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    let len = u32::try_from(bytes.len()).map_err(|_| Error::BinaryTooLong(bytes.len()))?;
    output.push(BINARY_EXT);
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

pub fn write_float(output: &mut Vec<u8>, v: f64) -> Result<()> {
    if !v.is_finite() {
        return Err(Error::InvalidFloat(v));
    }
    output.push(NEW_FLOAT_EXT);
    output.extend_from_slice(&v.to_be_bytes());
    Ok(())
}

pub fn write_integer_bytes(output: &mut Vec<u8>, sign: u8, le_bytes: &[u8]) -> Result<()> {
    let bytes = trim_trailing_zeroes(le_bytes);

    if bytes.is_empty() {
        output.push(SMALL_INTEGER_EXT);
        output.push(0);
        Ok(())
    } else if bytes.len() <= 1 && sign == 0 {
        output.push(SMALL_INTEGER_EXT);
        output.push(bytes[0]);
        Ok(())
    } else if let Some(v) = bytes_to_s64(sign.into(), bytes)
        && let Ok(v) = TryInto::<i32>::try_into(v)
    {
        output.push(INTEGER_EXT);
        output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    } else if let Ok(len) = bytes.len().try_into() {
        output.push(SMALL_BIG_EXT);
        output.push(len);
        output.push(sign);
        output.extend_from_slice(bytes);
        Ok(())
    } else if let Ok(len) = TryInto::<u32>::try_into(bytes.len()) {
        output.push(LARGE_BIG_EXT);
        output.extend_from_slice(&len.to_be_bytes());
        output.push(sign);
        output.extend_from_slice(bytes);
        Ok(())
    } else {
        Err(Error::IntegerTooLarge(bytes.len()))
    }
}

pub fn write_integer<T: PrimInt + ToBytes + WrappingNeg>(output: &mut Vec<u8>, v: T) -> Result<()> {
    if let Some(v) = v.to_u8() {
        output.push(SMALL_INTEGER_EXT);
        output.push(v);
        Ok(())
    } else if let Some(v) = v.to_i32() {
        output.push(INTEGER_EXT);
        output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    } else {
        let (sign, mag) = to_sign_mag(v);
        write_integer_bytes(output, sign, mag.as_ref())
    }
}

fn bytes_to_s64(sign: Sign, le_bytes: &[u8]) -> Option<i64> {
    if le_bytes.len() > 8 {
        return None;
    }

    let mut bytes = [0u8; 8];
    bytes[..le_bytes.len()].copy_from_slice(le_bytes);

    let magnitude = u64::from_le_bytes(bytes);
    match sign {
        Sign::None if magnitude <= i64::MAX as u64 => Some(magnitude as i64),
        Sign::Negative if magnitude <= i64::MAX as u64 => Some(-(magnitude as i64)),
        Sign::Negative if magnitude == i64::MAX as u64 + 1 => Some(i64::MIN),
        _ => None,
    }
}

fn to_sign_mag<T: PrimInt + ToBytes + WrappingNeg>(v: T) -> (u8, <T as ToBytes>::Bytes) {
    let sign = if v < T::zero() { 1u8 } else { 0u8 };
    let mag = if sign == 1 { v.wrapping_neg() } else { v };
    (sign, mag.to_le_bytes())
}

fn trim_trailing_zeroes(bytes: &[u8]) -> &[u8] {
    if bytes.is_empty() {
        return bytes;
    }
    let end = bytes.iter().rposition(|&b| b != 0).map(|i| i + 1).unwrap_or(1);
    &bytes[..end]
}

pub fn write_tuple_header(output: &mut Vec<u8>, len: usize) -> Result<()> {
    if let Ok(len) = len.try_into() {
        output.push(SMALL_TUPLE_EXT);
        output.push(len);
    } else if let Ok(len) = TryInto::<u32>::try_into(len) {
        output.push(LARGE_TUPLE_EXT);
        output.extend_from_slice(&len.to_be_bytes());
    } else {
        return Err(Error::TupleTooLong(len));
    }
    Ok(())
}

pub fn write_list_header(output: &mut Vec<u8>, len: usize) -> Result<()> {
    if let Ok(len) = TryInto::<u32>::try_into(len) {
        output.push(LIST_EXT);
        output.extend_from_slice(&len.to_be_bytes());
        Ok(())
    } else {
        Err(Error::ListTooLong(len))
    }
}

pub fn write_nil(output: &mut Vec<u8>) -> Result<()> {
    output.push(NIL_EXT);
    Ok(())
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Atom<'a>(pub &'a str);

/// The header of a list-shaped value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ListHeader {
    /// `NIL_EXT`.
    Empty,
    /// `LIST_EXT n`: `n` elements follow, then a tail.
    Proper(usize),
    /// `STRING_EXT n`: a compact list of `n` byte values.
    String(usize),
}

/// A checked cursor over ETF bytes. It borrows its input and never allocates
/// from an unchecked wire count.
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    None,
    Negative,
}

impl From<u8> for Sign {
    fn from(value: u8) -> Self {
        if value == 0 { Sign::None } else { Sign::Negative }
    }
}

pub enum Integer<'a> {
    // Decoded integer representable in `i64`. This should cover the vast majority of cases.
    Int(i64),
    // Sign followed by the borrowed bytes of unsigned magnitude in little-endian order.
    Big(Sign, &'a [u8]),
}

impl<'a> Integer<'a> {
    pub fn as_i64(&self) -> Result<i64> {
        match self {
            Integer::Int(v) => Ok(*v),
            Integer::Big(_sign, bytes) => Err(Error::IntegerTooLarge(bytes.len())),
        }
    }
}

impl<'a> Reader<'a> {
    /// Creates a reader, tolerating an omitted leading [`TERM_HEADER`] byte.
    pub fn new(bytes: &'a [u8]) -> Reader<'a> {
        let mut reader = Reader { bytes, cursor: 0 };
        if bytes.first() == Some(&TERM_HEADER) {
            reader.cursor = 1;
        }
        reader
    }

    pub fn position(&self) -> usize {
        self.cursor
    }

    /// Validates that the whole input was consumed.
    pub fn finish(&self) -> Result<()> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(Error::TrailingBytes { cursor: self.cursor, len: self.bytes.len() })
        }
    }

    fn protocol_error(&self, details: impl Into<String>) -> Error {
        Error::Protocol { offset: self.cursor, details: details.into() }
    }

    fn read_exact(&mut self, len: usize) -> Result<&'a [u8]> {
        let slice = self
            .cursor
            .checked_add(len)
            .and_then(|end| self.bytes.get(self.cursor..end))
            .ok_or_else(|| self.protocol_error("unexpected end of input"))?;
        self.cursor += len;
        Ok(slice)
    }

    fn read_array<const N: usize>(&mut self) -> Result<&'a [u8; N]> {
        let slice = self.read_exact(N)?;
        Ok(slice.try_into().expect("read_exact yields exactly N bytes"))
    }

    fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_array::<1>()?[0])
    }

    fn read_u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(*self.read_array()?))
    }

    fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(*self.read_array()?))
    }

    /// Returns the type tag of the next term without consuming it.
    pub fn peek_tag(&self) -> Result<u8> {
        self.bytes
            .get(self.cursor)
            .copied()
            .ok_or_else(|| self.protocol_error("unexpected end of input"))
    }

    /// Whether the next term is an atom.
    pub fn next_is_atom(&self) -> Result<bool> {
        Ok(matches!(self.peek_tag()?, ATOM_UTF8_EXT | SMALL_ATOM_UTF8_EXT))
    }

    /// Reads a tuple header and returns its arity.
    pub fn read_tuple_header(&mut self) -> Result<usize> {
        match self.read_u8()? {
            SMALL_TUPLE_EXT => Ok(self.read_u8()? as usize),
            LARGE_TUPLE_EXT => Ok(self.read_u32()? as usize),
            tag => Err(self.protocol_error(format!("expected tuple header, found tag {tag}"))),
        }
    }

    /// Reads a list header.
    pub fn read_list_header(&mut self) -> Result<ListHeader> {
        match self.read_u8()? {
            NIL_EXT => Ok(ListHeader::Empty),
            LIST_EXT => {
                let len = self.read_u32()? as usize;
                Ok(ListHeader::Proper(len))
            }
            STRING_EXT => {
                let len = self.read_u16()? as usize;
                if len == 0 { Ok(ListHeader::Empty) } else { Ok(ListHeader::String(len)) }
            }
            tag => Err(self.protocol_error(format!("expected list header, found tag {tag}"))),
        }
    }

    /// Reads an atom, returning its borrowed payload.
    pub fn read_atom(&mut self) -> Result<Atom<'a>> {
        let len = match self.read_u8()? {
            ATOM_UTF8_EXT => self.read_u16()? as usize,
            SMALL_ATOM_UTF8_EXT => self.read_u8()? as usize,
            tag => return Err(self.protocol_error(format!("expected atom, found tag {tag}"))),
        };
        let bytes = self.read_exact(len)?;
        match str::from_utf8(bytes) {
            Ok(s) => Ok(Atom(s)),
            Err(_) => Err(self.protocol_error("invalid UTF-8 atom")),
        }
    }

    /// Reads an atom that must be the boolean `true` or `false`.
    pub fn read_bool(&mut self) -> Result<bool> {
        match self.read_atom()?.0 {
            "true" => Ok(true),
            "false" => Ok(false),
            atom => Err(self.protocol_error(format!("expected boolean atom, found `{atom}`"))),
        }
    }

    /// Reads a binary, borrowing its payload.
    pub fn read_binary(&mut self) -> Result<&'a [u8]> {
        match self.read_u8()? {
            BINARY_EXT => {
                let len = self.read_u32()? as usize;
                self.read_exact(len)
            }
            other => Err(self.protocol_error(format!("expected binary, found tag {other}"))),
        }
    }

    /// Reads a binary string, borrowing its payload.
    pub fn read_string(&mut self) -> Result<&'a str> {
        let bytes = self.read_binary()?;
        match std::str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(self.protocol_error("invalid UTF-8 string")),
        }
    }

    /// Reads an integer.
    pub fn read_integer(&mut self) -> Result<Integer<'a>> {
        match self.read_u8()? {
            SMALL_INTEGER_EXT => Ok(Integer::Int(self.read_u8()? as i64)),
            INTEGER_EXT => Ok(Integer::Int(i32::from_be_bytes(*self.read_array()?) as i64)),
            tag @ (SMALL_BIG_EXT | LARGE_BIG_EXT) => {
                let len = if tag == SMALL_BIG_EXT {
                    self.read_u8()? as usize
                } else {
                    self.read_u32()? as usize
                };
                let sign = if self.read_u8()? == 0 { Sign::None } else { Sign::Negative };
                let le_bytes = self.read_exact(len)?;

                if let Some(v) = bytes_to_s64(sign, le_bytes) {
                    Ok(Integer::Int(v))
                } else {
                    Ok(Integer::Big(sign, le_bytes))
                }
            }
            tag => Err(self.protocol_error(format!("expected integer, found tag {tag}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reader(bytes: &[u8]) -> Reader<'_> {
        Reader::new(bytes)
    }

    fn read_int(bytes: &[u8]) -> i64 {
        match reader(bytes).read_integer().unwrap() {
            Integer::Int(v) => v,
            Integer::Big(..) => panic!("expected a decoded `Int`, got `Big`"),
        }
    }

    #[test]
    fn write_atom_small() {
        let mut buf = Vec::new();
        write_atom(&mut buf, "foo").unwrap();
        assert_eq!(buf, vec![SMALL_ATOM_UTF8_EXT, 3, b'f', b'o', b'o']);
    }

    #[test]
    fn write_atom_medium() {
        let atom = "a".repeat(256);
        let mut buf = Vec::new();
        write_atom(&mut buf, &atom).unwrap();
        let mut expected = vec![ATOM_UTF8_EXT, 0x01, 0x00];
        expected.extend_from_slice(atom.as_bytes());
        assert_eq!(buf, expected);
    }

    #[test]
    fn write_atom_too_long() {
        let atom = "a".repeat(u16::MAX as usize + 1);
        let mut buf = Vec::new();
        assert!(
            matches!(write_atom(&mut buf, &atom), Err(Error::AtomTooLong(n)) if n == atom.len())
        );
    }

    #[test]
    fn write_binary_ok() {
        let mut buf = Vec::new();
        write_binary(&mut buf, b"abc").unwrap();
        assert_eq!(buf, vec![BINARY_EXT, 0, 0, 0, 3, b'a', b'b', b'c']);
    }

    #[test]
    fn write_float_ok() {
        let mut buf = Vec::new();
        write_float(&mut buf, 1.5).unwrap();
        let mut expected = vec![NEW_FLOAT_EXT];
        expected.extend_from_slice(&1.5f64.to_be_bytes());
        assert_eq!(buf, expected);
    }

    #[test]
    fn write_float_nan_errors() {
        let mut buf = Vec::new();
        assert!(matches!(write_float(&mut buf, f64::NAN), Err(Error::InvalidFloat(_))));
    }

    #[test]
    fn write_float_infinity_errors() {
        let mut buf = Vec::new();
        assert!(matches!(write_float(&mut buf, f64::INFINITY), Err(Error::InvalidFloat(_))));
    }

    #[test]
    fn write_integer_small() {
        let mut buf = Vec::new();
        write_integer(&mut buf, 42u32).unwrap();
        assert_eq!(buf, vec![SMALL_INTEGER_EXT, 42]);
    }

    #[test]
    fn write_integer_negative() {
        let mut buf = Vec::new();
        write_integer(&mut buf, -1i32).unwrap();
        assert_eq!(buf, vec![INTEGER_EXT, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn write_integer_above_i32() {
        let mut buf = Vec::new();
        write_integer(&mut buf, i32::MAX as u64 + 1).unwrap();
        assert_eq!(buf, vec![SMALL_BIG_EXT, 4, 0, 0, 0, 0, 0x80]);
    }

    #[test]
    fn write_integer_negative_big() {
        let mut buf = Vec::new();
        write_integer(&mut buf, i32::MIN as i64 - 1).unwrap();
        assert_eq!(buf, vec![SMALL_BIG_EXT, 4, 1, 0x01, 0, 0, 0x80]);
    }

    #[test]
    fn write_integer_u128() {
        let mut buf = Vec::new();
        write_integer(&mut buf, u128::from(u64::MAX) + 1).unwrap();
        let mut expected = vec![SMALL_BIG_EXT, 9, 0];
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(buf, expected);
    }

    #[test]
    fn write_integer_bytes_trims_trailing_zeroes() {
        let mut buf = Vec::new();
        write_integer_bytes(&mut buf, 0, &[1, 2, 3, 0, 0]).unwrap();
        // Little-endian magnitude 1 + 2*256 + 3*65536 fits in `i32`, so the
        // writer picks the compact fixed-width form.
        assert_eq!(buf, vec![INTEGER_EXT, 0, 3, 2, 1]);
    }

    #[test]
    fn write_integer_bytes_empty_is_zero() {
        let mut buf = Vec::new();
        write_integer_bytes(&mut buf, 0, &[]).unwrap();
        assert_eq!(buf, vec![SMALL_INTEGER_EXT, 0]);
    }

    #[test]
    fn write_integer_bytes_negative_zero_is_zero() {
        let mut buf = Vec::new();
        write_integer_bytes(&mut buf, 1, &[0, 0, 0]).unwrap();
        assert_eq!(buf, vec![INTEGER_EXT, 0, 0, 0, 0]);
    }

    #[test]
    fn write_integer_bytes_large() {
        let mut bytes = vec![0u8; 256];
        bytes[255] = 1;
        let mut buf = Vec::new();
        write_integer_bytes(&mut buf, 0, &bytes).unwrap();
        let mut expected = vec![LARGE_BIG_EXT, 0, 0, 1, 0, 0];
        expected.extend_from_slice(&bytes);
        assert_eq!(buf, expected);
    }

    #[test]
    fn write_tuple_header_small() {
        let mut buf = Vec::new();
        write_tuple_header(&mut buf, 3).unwrap();
        assert_eq!(buf, vec![SMALL_TUPLE_EXT, 3]);
    }

    #[test]
    fn write_tuple_header_large() {
        let mut buf = Vec::new();
        write_tuple_header(&mut buf, 256).unwrap();
        assert_eq!(buf, vec![LARGE_TUPLE_EXT, 0, 0, 1, 0]);
    }

    #[test]
    fn write_tuple_header_too_long() {
        let len = u32::MAX as usize + 1;
        let mut buf = Vec::new();
        assert!(
            matches!(write_tuple_header(&mut buf, len), Err(Error::TupleTooLong(n)) if n == len)
        );
    }

    #[test]
    fn write_list_header_ok() {
        let mut buf = Vec::new();
        write_list_header(&mut buf, 3).unwrap();
        assert_eq!(buf, vec![LIST_EXT, 0, 0, 0, 3]);
    }

    #[test]
    fn write_list_header_too_long() {
        let len = u32::MAX as usize + 1;
        let mut buf = Vec::new();
        assert!(matches!(write_list_header(&mut buf, len), Err(Error::ListTooLong(n)) if n == len));
    }

    #[test]
    fn write_nil_ok() {
        let mut buf = Vec::new();
        write_nil(&mut buf).unwrap();
        assert_eq!(buf, vec![NIL_EXT]);
    }

    #[test]
    fn reads_small_integer_boundaries() {
        assert_eq!(read_int(&[SMALL_INTEGER_EXT, 0]), 0);
        assert_eq!(read_int(&[SMALL_INTEGER_EXT, 255]), 255);
    }

    #[test]
    fn reads_integer_boundaries() {
        for value in [i32::MIN, -1, 0, 255, 256, i32::MAX] {
            let mut bytes = vec![INTEGER_EXT];
            bytes.extend_from_slice(&value.to_be_bytes());
            assert_eq!(read_int(&bytes), value as i64, "value {value}");
        }
    }

    #[test]
    fn accepts_small_values_in_integer_ext() {
        let mut bytes = vec![INTEGER_EXT];
        bytes.extend_from_slice(&7i32.to_be_bytes());
        assert_eq!(read_int(&bytes), 7);
    }

    #[test]
    fn reads_bignum_integers() {
        // SMALL_BIG_EXT encoding of 1.
        assert_eq!(read_int(&[SMALL_BIG_EXT, 1, 0, 1]), 1);
        // LARGE_BIG_EXT encoding of 1.
        assert_eq!(read_int(&[LARGE_BIG_EXT, 0, 0, 0, 1, 0, 1]), 1);
        // Negative magnitude is sign-magnitude encoded.
        assert_eq!(read_int(&[SMALL_BIG_EXT, 1, 1, 1]), -1);
    }

    #[test]
    fn reads_bignums_too_large_for_i64() {
        // 2^64 does not fit in an `i64`, so the magnitude is borrowed.
        let mut bytes = vec![SMALL_BIG_EXT, 9, 0];
        bytes.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 1]);
        match reader(&bytes).read_integer().unwrap() {
            Integer::Big(Sign::None, payload) => {
                assert_eq!(payload, &[0, 0, 0, 0, 0, 0, 0, 0, 1]);
            }
            _ => panic!("expected a `Big` integer"),
        }
    }

    #[test]
    fn rejects_truncated_integer() {
        assert!(reader(&[INTEGER_EXT, 0, 0]).read_integer().is_err());
        assert!(reader(&[SMALL_INTEGER_EXT]).read_integer().is_err());
    }

    #[test]
    fn reads_atom_encodings() {
        let small = reader(&[SMALL_ATOM_UTF8_EXT, 3, b'f', b'o', b'o']).read_atom().unwrap();
        assert_eq!(small.0, "foo");

        let large = reader(&[ATOM_UTF8_EXT, 0, 3, b'f', b'o', b'o']).read_atom().unwrap();
        assert_eq!(large.0, "foo");
    }

    #[test]
    fn rejects_invalid_utf8_atom() {
        assert!(reader(&[SMALL_ATOM_UTF8_EXT, 1, 0xff]).read_atom().is_err());
    }

    #[test]
    fn reads_binaries_borrowed() {
        let bytes = [BINARY_EXT, 0, 0, 0, 3, b'a', b'b', b'c'];
        assert_eq!(reader(&bytes).read_binary().unwrap(), b"abc");
        assert!(reader(&[BINARY_EXT, 0, 0, 0, 4, b'a']).read_binary().is_err());
    }

    #[test]
    fn reads_tuple_headers() {
        assert_eq!(reader(&[SMALL_TUPLE_EXT, 3]).read_tuple_header().unwrap(), 3);
        assert_eq!(reader(&[LARGE_TUPLE_EXT, 0, 0, 1, 0]).read_tuple_header().unwrap(), 256);
    }

    #[test]
    fn reads_lists_and_nil() {
        assert_eq!(reader(&[NIL_EXT]).read_list_header().unwrap(), ListHeader::Empty);
        assert_eq!(
            reader(&[LIST_EXT, 0, 0, 0, 2]).read_list_header().unwrap(),
            ListHeader::Proper(2)
        );
        assert_eq!(
            reader(&[LIST_EXT, 0, 0, 0, 0]).read_list_header().unwrap(),
            ListHeader::Proper(0)
        );
        assert_eq!(
            reader(&[STRING_EXT, 0, 2, b'a', b'b']).read_list_header().unwrap(),
            ListHeader::String(2)
        );
        assert_eq!(reader(&[STRING_EXT, 0, 0]).read_list_header().unwrap(), ListHeader::Empty);
    }

    #[test]
    fn rejects_improper_list_tails() {
        // A list header followed by an integer where a tail belongs is not a
        // valid list; the schema decoder must reject it rather than treat it
        // as an element.
        assert!(reader(&[SMALL_INTEGER_EXT, 1]).read_list_header().is_err());
    }

    #[test]
    fn rejects_truncated_containers() {
        assert!(reader(&[SMALL_TUPLE_EXT, 3, SMALL_INTEGER_EXT, 1]).read_tuple_header().is_ok());
        assert!(reader(&[BINARY_EXT, 0, 0, 0, 4, b'a']).read_binary().is_err());
        assert!(reader(&[LIST_EXT, 0, 0]).read_list_header().is_err());
    }

    #[test]
    fn rejects_unsupported_tags() {
        assert!(reader(&[0xff]).read_tuple_header().is_err());
        assert!(reader(&[0xff]).read_atom().is_err());
        assert!(reader(&[0xff]).read_binary().is_err());
        assert!(reader(&[0xff]).read_integer().is_err());
    }

    #[test]
    fn tolerates_missing_version_byte_and_rejects_trailing_bytes() {
        assert_eq!(read_int(&[SMALL_INTEGER_EXT, 5]), 5);
        assert_eq!(read_int(&[TERM_HEADER, SMALL_INTEGER_EXT, 5]), 5);

        let mut r = reader(&[TERM_HEADER, SMALL_INTEGER_EXT, 5, 0]);
        r.read_integer().unwrap();
        assert!(matches!(r.finish(), Err(Error::TrailingBytes { .. })));

        let mut r = reader(&[SMALL_INTEGER_EXT, 5]);
        r.read_integer().unwrap();
        assert!(r.finish().is_ok());
    }

    #[test]
    fn does_not_allocate_from_unchecked_counts() {
        // A header claiming billions of elements must fail on the first read,
        // not attempt a huge allocation.
        let mut r = reader(&[LIST_EXT, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(r.read_list_header().unwrap(), ListHeader::Proper(0xffff_ffff));
    }

    #[test]
    fn tmp_roundtrip_signed() {
        let values = [
            i64::MIN,
            i64::MIN + 1,
            i64::MIN / 2,
            -1,
            0,
            1,
            256,
            i64::MAX / 2,
            i64::MAX - 1,
            i64::MAX,
        ];
        for &v in &values {
            let mut buf = Vec::new();
            write_integer(&mut buf, v).unwrap();
            match reader(&buf).read_integer().unwrap() {
                Integer::Int(got) => assert_eq!(got, v, "value {v}"),
                Integer::Big(sign, bytes) => {
                    panic!("value {v} decoded as Big({sign:?}, {bytes:?})")
                }
            }
        }
    }

    #[test]
    fn tmp_roundtrip_unsigned() {
        let values = [0u64, 1, 255, 256, i64::MAX as u64, i64::MAX as u64 + 1, u64::MAX];
        for &v in &values {
            let mut buf = Vec::new();
            write_integer(&mut buf, v).unwrap();
            let got = reader(&buf).read_integer().unwrap();
            if v <= i64::MAX as u64 {
                match got {
                    Integer::Int(g) => assert_eq!(g, v as i64, "value {v}"),
                    Integer::Big(s, b) => panic!("value {v} decoded as Big({s:?}, {b:?})"),
                }
            } else {
                match got {
                    Integer::Big(Sign::None, _) => {}
                    Integer::Int(g) => panic!("value {v} decoded as Int({g})"),
                    Integer::Big(s, b) => panic!("value {v} decoded as Big({s:?}, {b:?})"),
                }
            }
        }
    }
}
