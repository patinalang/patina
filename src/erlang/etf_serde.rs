use std::fmt::Display;

use num_traits::{PrimInt, ToBytes, WrappingNeg};
use serde::{Serialize, ser};

use crate::num_utils::{to_sign_mag, trim_trailing_zeroes};

const TERM_HEADER: u8 = 131;
const SMALL_ATOM_UTF8_EXT: u8 = 119;
const ATOM_UTF8_EXT: u8 = 118;
const SMALL_INTEGER_EXT: u8 = 97;
const INTEGER_EXT: u8 = 98;
const SMALL_BIG_EXT: u8 = 110;
const LARGE_BIG_EXT: u8 = 111;
const NEW_FLOAT_EXT: u8 = 70;
const SMALL_TUPLE_EXT: u8 = 104;
const LARGE_TUPLE_EXT: u8 = 105;
const LIST_EXT: u8 = 108;
const NIL_EXT: u8 = 106;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("{0}")]
    Message(String),
    #[error("atom too long: {0} bytes")]
    AtomTooLong(usize),
    #[error("integer too large: {0} bytes")]
    IntegerTooLarge(usize),
    #[error("tuple too long: {0} elements")]
    TupleTooLong(usize),
    #[error("list too long: {0} elements")]
    ListTooLong(usize),
    #[error("invalid float: {0}")]
    InvalidFloat(f64),
    #[error("unsupported type")]
    UnsupportedType,
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub(crate) struct Serializer {
    output: Vec<u8>,
}

pub(crate) fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer { output: vec![TERM_HEADER] };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

// Unsigned integer as little endian bytes.
#[derive(Clone, Copy, Debug)]
pub(crate) struct BigUInt<'a>(pub &'a [u8]);

impl<'a> Serialize for BigUInt<'a> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0)
    }
}

impl Serializer {
    fn write_atom(&mut self, atom: &str) -> Result<()> {
        let bytes = atom.as_bytes();
        if let Ok(len) = bytes.len().try_into() {
            self.output.push(SMALL_ATOM_UTF8_EXT);
            self.output.push(len);
            self.output.extend_from_slice(bytes);
        } else if let Ok(len) = TryInto::<u16>::try_into(bytes.len()) {
            self.output.push(ATOM_UTF8_EXT);
            self.output.extend_from_slice(&len.to_be_bytes());
            self.output.extend_from_slice(bytes);
        } else {
            return Err(Error::AtomTooLong(bytes.len()));
        }
        Ok(())
    }

    fn write_float(&mut self, v: f64) -> Result<()> {
        if !v.is_finite() {
            return Err(Error::InvalidFloat(v));
        }
        self.output.push(NEW_FLOAT_EXT);
        self.output.extend_from_slice(&v.to_be_bytes());
        Ok(())
    }

    pub(crate) fn write_integer_bytes(&mut self, sign: u8, bytes: &[u8]) -> Result<()> {
        let bytes = trim_trailing_zeroes(bytes);

        if let Ok(len) = bytes.len().try_into() {
            self.output.push(SMALL_BIG_EXT);
            self.output.push(len);
            self.output.push(sign);
            self.output.extend_from_slice(bytes);
            Ok(())
        } else if let Ok(len) = TryInto::<u32>::try_into(bytes.len()) {
            self.output.push(LARGE_BIG_EXT);
            self.output.extend_from_slice(&len.to_be_bytes());
            self.output.push(sign);
            self.output.extend_from_slice(bytes);
            Ok(())
        } else {
            Err(Error::IntegerTooLarge(bytes.len()))
        }
    }

    fn write_integer<T: PrimInt + ToBytes + WrappingNeg>(&mut self, v: T) -> Result<()> {
        if let Some(v) = v.to_u8() {
            self.output.push(SMALL_INTEGER_EXT);
            self.output.push(v);
            Ok(())
        } else if let Some(v) = v.to_i32() {
            self.output.push(INTEGER_EXT);
            self.output.extend_from_slice(&v.to_be_bytes());
            Ok(())
        } else {
            let (sign, mag) = to_sign_mag(v);
            self.write_integer_bytes(sign, mag.as_ref())
        }
    }

    fn write_tuple_header(&mut self, len: usize) -> Result<()> {
        if let Ok(len) = len.try_into() {
            self.output.push(SMALL_TUPLE_EXT);
            self.output.push(len);
        } else if let Ok(len) = TryInto::<u32>::try_into(len) {
            self.output.push(LARGE_TUPLE_EXT);
            self.output.extend_from_slice(&len.to_be_bytes());
        } else {
            return Err(Error::TupleTooLong(len));
        }
        Ok(())
    }

    fn write_list_header(&mut self, len: usize) -> Result<()> {
        if let Ok(len) = TryInto::<u32>::try_into(len) {
            self.output.push(LIST_EXT);
            self.output.extend_from_slice(&len.to_be_bytes());
            Ok(())
        } else {
            Err(Error::ListTooLong(len))
        }
    }

    fn write_nil(&mut self) -> Result<()> {
        self.output.push(NIL_EXT);
        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = SerializeTupleStruct<'a>;
    type SerializeTupleVariant = SerializeTupleStruct<'a>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = SerializeTupleStruct<'a>;
    type SerializeStructVariant = SerializeTupleStruct<'a>;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.write_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_float(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_float(v)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.write_integer(Into::<u32>::into(v))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.write_atom(v)
    }

    /// Serializes an *unsigned* integer given little endian bytes.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let v = trim_trailing_zeroes(v);

        match v {
            [] => self.write_integer(0u8),
            [a] => self.write_integer(*a),
            [a, b] => self.write_integer(u16::from_le_bytes([*a, *b])),
            [a, b, c] => self.write_integer(u32::from_le_bytes([*a, *b, *c, 0])),
            [a, b, c, d] => self.write_integer(u32::from_le_bytes([*a, *b, *c, *d])),
            _ => self.write_integer_bytes(0, v),
        }
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.write_nil()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        self.write_atom(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.write_atom(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            // An empty list is just NIL_EXT; will get written by the end method
            Some(0) => Ok(()),
            Some(len) => self.write_list_header(len),
            None => Err(Error::UnsupportedType),
        }?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let insert_name = if name.starts_with("__type__") {
            self.write_tuple_header(len + 2)?;
            self.write_atom("type")?;
            Some((1, name.strip_prefix("__type__").unwrap()))
        } else if name.starts_with("__attribute__") {
            self.write_tuple_header(len + 2)?;
            self.write_atom("attribute")?;
            Some((1, name.strip_prefix("__attribute__").unwrap()))
        } else if name != "__elide__" {
            self.write_tuple_header(len + 1)?;
            self.write_atom(name)?;
            None
        } else {
            self.write_tuple_header(len)?;
            None
        };

        Ok(SerializeTupleStruct { serializer: self, count: 0, insert_name })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_tuple_struct(variant, len)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_tuple_struct(name, len)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_tuple_struct(variant, len)
    }
}

impl ser::SerializeSeq for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_nil()
    }
}

impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.write_nil()
    }
}

pub(crate) struct SerializeTupleStruct<'a> {
    serializer: &'a mut Serializer,
    count: usize,
    insert_name: Option<(usize, &'static str)>,
}

impl SerializeTupleStruct<'_> {
    fn write_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)?;
        self.count += 1;
        if let Some((pos, name)) = self.insert_name
            && self.count == pos
        {
            self.serializer.write_atom(name)?;
            self.count += 1;
        }
        Ok(())
    }
}

impl ser::SerializeTupleStruct for SerializeTupleStruct<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_field(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for SerializeTupleStruct<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_field(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for SerializeTupleStruct<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_field(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for SerializeTupleStruct<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_field(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atom() {
        assert_eq!(
            to_bytes("foo").unwrap(),
            vec![TERM_HEADER, SMALL_ATOM_UTF8_EXT, 3, b'f', b'o', b'o']
        );
    }

    #[test]
    fn small_integer() {
        assert_eq!(to_bytes(&42u32).unwrap(), vec![TERM_HEADER, SMALL_INTEGER_EXT, 42]);
    }

    #[test]
    fn negative_integer() {
        assert_eq!(
            to_bytes(&-1i32).unwrap(),
            vec![TERM_HEADER, INTEGER_EXT, 0xff, 0xff, 0xff, 0xff]
        );
    }

    #[test]
    fn integer_above_i32() {
        let v: u64 = i32::MAX as u64 + 1;
        assert_eq!(to_bytes(&v).unwrap(), vec![TERM_HEADER, SMALL_BIG_EXT, 4, 0, 0, 0, 0, 0x80]);
    }

    #[test]
    fn negative_big_integer() {
        let v: i64 = i32::MIN as i64 - 1;
        assert_eq!(to_bytes(&v).unwrap(), vec![TERM_HEADER, SMALL_BIG_EXT, 4, 1, 0x01, 0, 0, 0x80]);
    }

    #[test]
    fn u128_value() {
        let v: u128 = u128::from(u64::MAX) + 1;
        let mut expected = vec![TERM_HEADER, SMALL_BIG_EXT, 9, 0];
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(to_bytes(&v).unwrap(), expected);
    }

    #[test]
    fn float() {
        let v = 1.5f64;
        let mut expected = vec![TERM_HEADER, NEW_FLOAT_EXT];
        expected.extend_from_slice(&v.to_be_bytes());
        assert_eq!(to_bytes(&v).unwrap(), expected);
    }

    #[test]
    fn float_nan_errors() {
        assert!(matches!(to_bytes(&f64::NAN), Err(Error::InvalidFloat(_))));
    }

    #[test]
    fn float_infinity_errors() {
        assert!(matches!(to_bytes(&f64::INFINITY), Err(Error::InvalidFloat(_))));
    }

    #[test]
    fn list() {
        let v = vec![1u8, 2, 3];
        assert_eq!(
            to_bytes(&v).unwrap(),
            vec![
                TERM_HEADER,
                LIST_EXT,
                0,
                0,
                0,
                3,
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
                SMALL_INTEGER_EXT,
                3,
                NIL_EXT,
            ]
        );
    }

    #[test]
    fn empty_list() {
        let v: Vec<u8> = vec![];
        assert_eq!(to_bytes(&v).unwrap(), vec![TERM_HEADER, NIL_EXT]);
    }

    #[test]
    fn nested_list() {
        let v: Vec<Vec<u8>> = vec![vec![1, 2], vec![]];
        assert_eq!(
            to_bytes(&v).unwrap(),
            vec![
                TERM_HEADER,
                LIST_EXT,
                0,
                0,
                0,
                2,
                // first element: [1, 2]
                LIST_EXT,
                0,
                0,
                0,
                2,
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
                NIL_EXT,
                // second element: []
                NIL_EXT,
                // outer terminator
                NIL_EXT,
            ]
        );
    }

    #[test]
    fn tuple() {
        let v = ("foo", 1u8);
        assert_eq!(
            to_bytes(&v).unwrap(),
            vec![
                TERM_HEADER,
                LIST_EXT,
                0,
                0,
                0,
                2,
                SMALL_ATOM_UTF8_EXT,
                3,
                b'f',
                b'o',
                b'o',
                SMALL_INTEGER_EXT,
                1,
                NIL_EXT,
            ]
        );
    }

    #[test]
    fn tuple_struct_is_untagged_tuple() {
        #[derive(Serialize)]
        struct Pair(u8, u8);
        assert_eq!(
            to_bytes(&Pair(1, 2)).unwrap(),
            vec![
                TERM_HEADER,
                SMALL_TUPLE_EXT,
                3,
                SMALL_ATOM_UTF8_EXT,
                4,
                b'P',
                b'a',
                b'i',
                b'r',
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
            ]
        );
    }

    #[test]
    fn newtype_struct_is_transparent() {
        #[derive(Serialize)]
        struct Wrapper(u8);
        assert_eq!(to_bytes(&Wrapper(7)).unwrap(), vec![TERM_HEADER, SMALL_INTEGER_EXT, 7]);
    }

    #[test]
    fn struct_is_untagged_tuple() {
        #[derive(Serialize)]
        struct Point {
            x: u8,
            y: u8,
        }
        assert_eq!(
            to_bytes(&Point { x: 1, y: 2 }).unwrap(),
            vec![
                TERM_HEADER,
                SMALL_TUPLE_EXT,
                3,
                SMALL_ATOM_UTF8_EXT,
                5,
                b'P',
                b'o',
                b'i',
                b'n',
                b't',
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
            ]
        );
    }

    #[test]
    fn unit_struct_is_bare_atom() {
        #[derive(Serialize)]
        struct Marker;
        assert_eq!(
            to_bytes(&Marker).unwrap(),
            vec![TERM_HEADER, SMALL_ATOM_UTF8_EXT, 6, b'M', b'a', b'r', b'k', b'e', b'r',]
        );
    }

    #[test]
    fn enum_unit_variant_is_bare_atom() {
        #[derive(Serialize)]
        enum E {
            FooBar,
        }
        assert_eq!(
            to_bytes(&E::FooBar).unwrap(),
            vec![TERM_HEADER, SMALL_ATOM_UTF8_EXT, 6, b'F', b'o', b'o', b'B', b'a', b'r',]
        );
    }

    #[test]
    fn enum_newtype_variant_is_transparent() {
        #[derive(Serialize)]
        enum E {
            Wrap(u8),
        }
        assert_eq!(to_bytes(&E::Wrap(7)).unwrap(), vec![TERM_HEADER, SMALL_INTEGER_EXT, 7]);
    }

    #[test]
    fn enum_newtype_variant_around_struct_unwraps_to_struct_fields() {
        #[derive(Serialize)]
        struct Module {
            id: u8,
        }
        #[derive(Serialize)]
        enum Form {
            Mod(Module),
        }
        assert_eq!(
            to_bytes(&Form::Mod(Module { id: 1 })).unwrap(),
            vec![
                TERM_HEADER,
                SMALL_TUPLE_EXT,
                2,
                SMALL_ATOM_UTF8_EXT,
                6,
                b'M',
                b'o',
                b'd',
                b'u',
                b'l',
                b'e',
                SMALL_INTEGER_EXT,
                1,
            ]
        );
    }

    #[test]
    fn enum_tuple_variant_is_untagged_tuple() {
        #[derive(Serialize)]
        enum E {
            Pair(u8, u8),
        }
        assert_eq!(
            to_bytes(&E::Pair(1, 2)).unwrap(),
            vec![
                TERM_HEADER,
                SMALL_TUPLE_EXT,
                3,
                SMALL_ATOM_UTF8_EXT,
                4,
                b'P',
                b'a',
                b'i',
                b'r',
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
            ]
        );
    }

    #[test]
    fn enum_struct_variant_is_untagged_tuple() {
        #[derive(Serialize)]
        enum E {
            Point { x: u8, y: u8 },
        }
        assert_eq!(
            to_bytes(&E::Point { x: 1, y: 2 }).unwrap(),
            vec![
                TERM_HEADER,
                SMALL_TUPLE_EXT,
                3,
                SMALL_ATOM_UTF8_EXT,
                5,
                b'P',
                b'o',
                b'i',
                b'n',
                b't',
                SMALL_INTEGER_EXT,
                1,
                SMALL_INTEGER_EXT,
                2,
            ]
        );
    }

    #[test]
    fn map_is_unsupported() {
        use std::collections::BTreeMap;
        let mut m = BTreeMap::new();
        m.insert("a", 1u8);
        assert!(matches!(to_bytes(&m), Err(Error::UnsupportedType)));
    }

    #[test]
    fn bool_is_unsupported() {
        assert!(matches!(to_bytes(&true), Err(Error::UnsupportedType)));
    }

    #[test]
    fn biguint_small_normalizes_to_small_integer() {
        let v = BigUInt(&[42]);
        assert_eq!(to_bytes(&v).unwrap(), vec![TERM_HEADER, SMALL_INTEGER_EXT, 42]);
    }

    #[test]
    fn biguint_above_u128() {
        let mut bytes = [0u8; 17];
        bytes[16] = 1;
        let v = BigUInt(&bytes);
        let mut expected = vec![TERM_HEADER, SMALL_BIG_EXT, 17, 0];
        expected.extend(std::iter::repeat(0u8).take(16));
        expected.push(1);
        assert_eq!(to_bytes(&v).unwrap(), expected);
    }

    #[test]
    fn biguint_zero() {
        let v = BigUInt(&[]);
        assert_eq!(to_bytes(&v).unwrap(), vec![TERM_HEADER, SMALL_INTEGER_EXT, 0]);
    }

    #[test]
    fn raw_bytes_serialize_as_unsigned_integer() {
        // Three little-endian magnitude bytes normalize to INTEGER_EXT.
        assert_eq!(
            to_bytes(&serde_bytes_compat::Bytes(&[1, 2, 3])).unwrap(),
            vec![TERM_HEADER, INTEGER_EXT, 0, 0x03, 0x02, 0x01]
        );
    }

    #[test]
    fn seq_without_known_length_unsupported() {
        struct Iter;
        impl Serialize for Iter {
            fn serialize<S: serde::Serializer>(
                &self,
                s: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                use serde::ser::SerializeSeq;
                let seq = s.serialize_seq(None)?;
                seq.end()
            }
        }
        assert!(matches!(to_bytes(&Iter), Err(Error::UnsupportedType)));
    }
}

#[cfg(test)]
mod serde_bytes_compat {
    use serde::{Serialize, Serializer};
    pub(crate) struct Bytes<'a>(pub(crate) &'a [u8]);
    impl Serialize for Bytes<'_> {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(self.0)
        }
    }
}
