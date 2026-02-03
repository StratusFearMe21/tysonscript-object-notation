use std::{collections::HashMap, fmt::Display, io::Write};

use serde_core::{
    Serialize, Serializer,
    ser::{
        self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
};
use thiserror::Error;

pub struct TsonSerializer<W: Write> {
    writer: W,
    field_stack: Vec<&'static str>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred while writing to writer")]
    Io(#[from] std::io::Error),
    #[error("Map key must be stringable")]
    KeyMustBeStringable,
    #[error("Error during serialization: `{0}`")]
    Custom(String),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

macro_rules! serialize_value {
    ($fn_name:ident, $v:ty) => {
        fn $fn_name(self, v: $v) -> Result<Self::Ok, Self::Error> {
            self.prefix_that_shit(b" ")?;
            self.writer.write_fmt(format_args!("{}", v))?;
            self.suffix_that_shit()
        }
    };
}

impl<'a, W: Write> TsonSerializer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            field_stack: Vec::new(),
        }
    }

    pub fn into_inner(self) -> W {
        debug_assert!(self.field_stack.is_empty());
        self.writer
    }

    fn prefix_that_variant_type_shit(&mut self, variant: &'static str) -> Result<(), Error> {
        if let Some(key) = self.field_stack.last() {
            self.writer.write_all(b"\n")?;
            self.writer.write_all(key.as_bytes())?;
            if !key.is_empty() {
                self.writer.write_all(b" ")?;
            }
        }

        self.writer.write_all(variant.as_bytes())?;
        self.writer.write_all(b"\n")?;

        Ok(())
    }

    fn prefix_that_shit(&mut self, after: &[u8]) -> Result<(), Error> {
        if let Some(key) = self.field_stack.last() {
            self.writer.write_all(key.as_bytes())?;
            if !key.is_empty() {
                self.writer.write_all(after)?;
            }
        }

        Ok(())
    }

    fn suffix_that_shit(&mut self) -> Result<(), Error> {
        if !self.field_stack.is_empty() {
            self.writer.write_all(b" that shit\n")?;
        }

        Ok(())
    }

    fn serialize_struct_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        self.field_stack.push(key);
        value.serialize(&mut *self)?;
        self.field_stack.pop();

        Ok(())
    }

    fn end_struct(&mut self) -> Result<(), Error> {
        if self.field_stack.len() > 0 {
            self.writer.write_all(b"oh yeah\n\n")?;
        }

        Ok(())
    }
}

impl<'a, W: Write> Serializer for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    serialize_value!(serialize_i8, i8);
    serialize_value!(serialize_i16, i16);
    serialize_value!(serialize_i32, i32);
    serialize_value!(serialize_i64, i64);
    serialize_value!(serialize_u8, u8);
    serialize_value!(serialize_u16, u16);
    serialize_value!(serialize_u32, u32);
    serialize_value!(serialize_u64, u64);
    serialize_value!(serialize_f32, f32);
    serialize_value!(serialize_f64, f64);
    serialize_value!(serialize_char, char);
    serialize_value!(serialize_str, &str);

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if !v {
            self.writer.write_all(b"dont fuckin")?;
        } else {
            self.writer.write_all(b"fuckin")?;
        }

        if let Some(key) = self.field_stack.last() {
            self.writer.write_all(b" ")?;
            self.writer.write_all(key.as_bytes())?;
        }

        self.suffix_that_shit()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.prefix_that_shit(b"\n")?;
        for i in v {
            self.writer.write_fmt(format_args!("{} that shit\n", *i))?;
        }
        self.writer.write_all(b"oh yeah\n")?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(b"in theory")?;

        if let Some(key) = self.field_stack.last() {
            self.writer.write_all(b" ")?;
            self.writer.write_all(key.as_bytes())?;
        }

        self.suffix_that_shit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.prefix_that_shit(b" ")?;
        self.writer.write_all(b"unit")?;
        self.suffix_that_shit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.prefix_that_shit(b" ")?;
        self.writer.write_all(name.as_bytes())?;
        self.suffix_that_shit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.prefix_that_shit(b" ")?;
        self.writer.write_all(variant.as_bytes())?;
        self.suffix_that_shit()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.prefix_that_variant_type_shit(variant)?;
        value.serialize(&mut *self)?;
        self.writer.write_all(b"oh yeah\n")?;
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.field_stack.last() == Some(&"") {
            self.prefix_that_variant_type_shit("fuckin")?;
        } else {
            self.prefix_that_shit(b"\n")?;
        }
        self.field_stack.push("");

        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.prefix_that_variant_type_shit(variant)?;
        self.field_stack.push("");

        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if !self.field_stack.is_empty() {
            self.writer.write_all(b"\n")?;
        }
        self.prefix_that_shit(b"\n")?;

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.prefix_that_variant_type_shit(variant)?;

        Ok(self)
    }
}

impl<'a, W: Write> SerializeSeq for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.field_stack.pop();
        self.end_struct()
    }
}

impl<'a, W: Write> SerializeTuple for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.field_stack.pop();
        self.end_struct()
    }
}

impl<'a, W: Write> SerializeTupleStruct for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.field_stack.pop();
        self.end_struct()
    }
}

impl<'a, W: Write> SerializeTupleVariant for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.field_stack.pop();
        self.end_struct()
    }
}

impl<'a, W: Write> SerializeMap for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, W: Write> SerializeStruct for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct()
    }
}

impl<'a, W: Write> SerializeStructVariant for &'a mut TsonSerializer<W> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_field(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct()
    }
}
