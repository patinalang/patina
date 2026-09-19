use std::fmt;

use etf::{
    TERM_HEADER, write_atom, write_float, write_integer, write_integer_bytes, write_list_header,
    write_nil, write_tuple_header,
};
use serde::{Serialize, ser};

use crate::num_utils::trim_trailing_zeroes;

#[derive(Debug)]
pub(crate) enum Error {
    /// A low-level ETF writing failure.
    Etf(etf::Error),
    /// A Serde error raised by a `Serialize` implementation.
    Message(String),
    /// A Rust value whose shape has no ETF mapping.
    UnsupportedType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Etf(error) => fmt::Display::fmt(error, f),
            Error::Message(message) => f.write_str(message),
            Error::UnsupportedType => f.write_str("unsupported type"),
        }
    }
}

impl std::error::Error for Error {}

impl From<etf::Error> for Error {
    fn from(error: etf::Error) -> Error {
        Error::Etf(error)
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

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
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        write_integer(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        write_float(&mut self.output, v.into())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        write_float(&mut self.output, v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        write_integer(&mut self.output, Into::<u32>::into(v))?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        write_atom(&mut self.output, v)?;
        Ok(())
    }

    /// Serializes an *unsigned* integer given little endian bytes.
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let v = trim_trailing_zeroes(v);

        match v {
            [] => write_integer(&mut self.output, 0u8)?,
            [a] => write_integer(&mut self.output, *a)?,
            [a, b] => write_integer(&mut self.output, u16::from_le_bytes([*a, *b]))?,
            [a, b, c] => write_integer(&mut self.output, u32::from_le_bytes([*a, *b, *c, 0]))?,
            [a, b, c, d] => write_integer(&mut self.output, u32::from_le_bytes([*a, *b, *c, *d]))?,
            _ => write_integer_bytes(&mut self.output, 0, v)?,
        }
        Ok(())
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
        write_nil(&mut self.output)?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        write_atom(&mut self.output, name)?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        write_atom(&mut self.output, variant)?;
        Ok(())
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
            Some(0) => {}
            Some(len) => write_list_header(&mut self.output, len)?,
            None => return Err(Error::UnsupportedType),
        }
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
            write_tuple_header(&mut self.output, len + 2)?;
            write_atom(&mut self.output, "type")?;
            Some((1, name.strip_prefix("__type__").unwrap()))
        } else if name.starts_with("__attribute__") {
            write_tuple_header(&mut self.output, len + 2)?;
            write_atom(&mut self.output, "attribute")?;
            Some((1, name.strip_prefix("__attribute__").unwrap()))
        } else if name != "__elide__" {
            write_tuple_header(&mut self.output, len + 1)?;
            write_atom(&mut self.output, name)?;
            None
        } else {
            write_tuple_header(&mut self.output, len)?;
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
        write_nil(&mut self.output)?;
        Ok(())
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
        write_nil(&mut self.output)?;
        Ok(())
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
            write_atom(&mut self.serializer.output, name)?;
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
