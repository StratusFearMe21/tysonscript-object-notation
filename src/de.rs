use std::{borrow::Cow, fmt::Display, iter::Peekable, str::SplitWhitespace};

use serde_core::{
    Deserializer,
    de::{self, EnumAccess, Expected, MapAccess, SeqAccess, Unexpected, VariantAccess, Visitor},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid type: {0}, expected {1}")]
    InvalidType(String, String),
    #[error("invalid value: {0}, expected {1}")]
    InvalidValue(String, String),
    #[error("invalid length: {0}, expected {1}")]
    InvalidLength(usize, String),
    #[error("unknown variant `{0}`, expected one of {0:?}")]
    UnknownVariant(String, &'static [&'static str]),
    #[error("unknown field `{0}`, expected one of {0:?}")]
    UnknownField(String, &'static [&'static str]),
    #[error("missing field `{0}`")]
    MissingField(&'static str),
    #[error("duplicate field `{0}`")]
    DuplicateField(&'static str),
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("{0}")]
    Custom(String),
}

impl de::Error for Error {
    #[cold]
    fn invalid_type(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Error::InvalidType(unexp.to_string(), exp.to_string())
    }

    #[cold]
    fn invalid_value(unexp: Unexpected, exp: &dyn Expected) -> Self {
        Error::InvalidValue(unexp.to_string(), exp.to_string())
    }

    #[cold]
    fn invalid_length(len: usize, exp: &dyn Expected) -> Self {
        Error::InvalidLength(len, exp.to_string())
    }

    #[cold]
    fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
        Error::UnknownVariant(variant.to_owned(), expected)
    }

    #[cold]
    fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
        Error::UnknownField(field.to_owned(), expected)
    }

    #[cold]
    fn missing_field(field: &'static str) -> Self {
        Error::MissingField(field)
    }

    #[cold]
    fn duplicate_field(field: &'static str) -> Self {
        Error::DuplicateField(field)
    }

    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Custom(msg.to_string())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'source> {
    // Prefixes
    Dont,
    InTheory,

    // Suffixes
    ThatShit,
    OhYeah,

    Text(&'source str),
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ThatShit => write!(f, "that shit"),
            Self::Dont => write!(f, "dont"),
            Self::OhYeah => write!(f, "oh yeah"),
            Self::InTheory => write!(f, "in theory"),
            Self::Text(text) => write!(f, "{}", text),
        }
    }
}

pub struct SplitWhitespaceAndFuckin<'source>(SplitWhitespace<'source>);

impl<'source> Iterator for SplitWhitespaceAndFuckin<'source> {
    type Item = &'source str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.0.next()?;

            if next != "fuckin" {
                return Some(next);
            }
        }
    }
}

pub struct TsonLexer<'source>(Peekable<SplitWhitespaceAndFuckin<'source>>);

impl<'source> Iterator for TsonLexer<'source> {
    type Item = Token<'source>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.0.next()?;

        match next {
            "that" if self.0.peek() == Some(&"shit") => {
                self.0.next();
                return Some(Token::ThatShit);
            }
            "dont" => return Some(Token::Dont),
            "oh" if self.0.peek() == Some(&"yeah") => {
                self.0.next();
                return Some(Token::OhYeah);
            }
            "in" if self.0.peek() == Some(&"theory") => {
                self.0.next();
                return Some(Token::InTheory);
            }
            text => return Some(Token::Text(text)),
        }
    }
}

impl<'source> TsonLexer<'source> {
    fn new(source: &'source str) -> TsonLexer<'source> {
        TsonLexer(SplitWhitespaceAndFuckin(source.split_whitespace()).peekable())
    }
}

pub struct TsonDeserializer<'a> {
    reader: Peekable<TsonLexer<'a>>,
    prefix_token: Option<Token<'a>>,
}

impl<'source> TsonDeserializer<'source> {
    pub fn new(str: &'source str) -> Self {
        // let reader = TsonLexer::new(str);
        // for t in reader {
        //     dbg!(t);
        // }
        let reader = TsonLexer::new(str).peekable();
        Self {
            reader,
            prefix_token: None,
        }
    }

    fn next(&mut self) -> Result<Token<'source>, Error> {
        self.reader.next().ok_or(Error::UnexpectedEof)
    }

    fn that_shit(&mut self) -> Result<(), Error> {
        let next_token = self.next()?;
        if next_token != Token::ThatShit {
            return Err(Error::InvalidValue(
                next_token.to_string(),
                Token::ThatShit.to_string(),
            ));
        }
        Ok(())
    }

    fn oh_yeah_or_none(&mut self) -> Result<(), Error> {
        let next_token = self.reader.next();
        let Some(next_token) = next_token else {
            return Ok(());
        };
        if next_token != Token::OhYeah {
            return Err(Error::InvalidValue(
                next_token.to_string(),
                Token::OhYeah.to_string(),
            ));
        }

        Ok(())
    }

    fn text(&mut self) -> Result<&'source str, Error> {
        if self.reader.peek() == Some(&Token::ThatShit) {
            return Ok("");
        }
        let next_token = self.next()?;
        let Token::Text(text) = next_token else {
            return Err(Error::InvalidValue(
                next_token.to_string(),
                String::from("text"),
            ));
        };
        Ok(text)
    }

    fn collapse_that_shit(&mut self) -> Result<Cow<'source, str>, Error> {
        let start_text = self.text()?;

        let start_ptr = start_text.as_ptr();
        let mut end_ptr = unsafe { start_text.as_ptr().offset(start_text.len() as isize) };

        loop {
            let next_token = self.next()?;
            match next_token {
                Token::Text(text) => {
                    end_ptr = unsafe { text.as_ptr().offset(text.len() as isize) };
                }
                Token::ThatShit => break,
                _ => {}
            }
        }

        Ok(Cow::Borrowed(unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                start_ptr,
                end_ptr as usize - start_ptr as usize,
            ))
        }))
    }
}

macro_rules! deserialize_value {
    ($fn_name:ident, $visitor:ident, $expected:expr) => {
        fn $fn_name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let text = self.text()?;
            self.that_shit()?;
            visitor.$visitor(
                text.parse()
                    .map_err(|_| Error::InvalidValue(text.to_string(), String::from($expected)))?,
            )
        }
    };
}

impl<'de> Deserializer<'de> for &mut TsonDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    deserialize_value!(deserialize_i8, visit_i8, "i8");
    deserialize_value!(deserialize_i16, visit_i16, "i16");
    deserialize_value!(deserialize_i32, visit_i32, "i32");
    deserialize_value!(deserialize_i64, visit_i64, "i64");
    deserialize_value!(deserialize_u8, visit_u8, "u8");
    deserialize_value!(deserialize_u16, visit_u16, "u16");
    deserialize_value!(deserialize_u32, visit_u32, "u32");
    deserialize_value!(deserialize_u64, visit_u64, "u64");
    deserialize_value!(deserialize_f32, visit_f32, "f32");
    deserialize_value!(deserialize_f64, visit_f64, "f64");
    deserialize_value!(deserialize_char, visit_char, "char");

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let result = if self.prefix_token == Some(Token::Dont) {
            self.prefix_token = None;
            false
        } else {
            true
        };

        self.that_shit()?;
        visitor.visit_bool(result)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.collapse_that_shit()? {
            Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
            Cow::Owned(s) => visitor.visit_string(s),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.prefix_token == Some(Token::InTheory) {
            self.prefix_token = None;
            self.that_shit()?;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let next_token = self.next()?;
        if next_token != Token::Text("unit") {
            return Err(Error::InvalidType(
                next_token.to_string(),
                String::from("unit"),
            ));
        };
        self.that_shit()?;
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let next_token = self.next()?;
        if next_token != Token::Text(name) {
            return Err(Error::InvalidType(
                next_token.to_string(),
                String::from(name),
            ));
        };
        self.that_shit()?;
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_seq(TsonSeqAccess { deserializer: self })?;

        self.oh_yeah_or_none()?;

        Ok(result)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let result = visitor.visit_map(TsonMapAccess { deserializer: self })?;

        self.oh_yeah_or_none()?;

        Ok(result)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(TsonEnumAccess { deserializer: self })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let next_token = self.next()?;
        let text = match next_token {
            Token::Text(text) => text,
            Token::Dont | Token::InTheory => {
                self.prefix_token = Some(next_token);
                self.text()?
            }
            token => {
                return Err(Error::InvalidValue(
                    token.to_string(),
                    String::from("text, dont, in theory"),
                ));
            }
        };
        visitor.visit_borrowed_str(text)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

pub struct TsonMapAccess<'de, 'a> {
    deserializer: &'a mut TsonDeserializer<'de>,
}

impl<'de, 'a> MapAccess<'de> for TsonMapAccess<'de, 'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        let next_token = self.deserializer.reader.peek();
        if next_token.is_none() || next_token == Some(&Token::OhYeah) {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.deserializer).map(Some)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

pub struct TsonEnumAccess<'de, 'a> {
    deserializer: &'a mut TsonDeserializer<'de>,
}

impl<'de, 'a> EnumAccess<'de> for TsonEnumAccess<'de, 'a> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.deserializer)?;

        Ok((value, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for TsonEnumAccess<'de, 'a> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        self.deserializer.that_shit()?;
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_seq(visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserializer.deserialize_map(visitor)
    }
}

pub struct TsonSeqAccess<'de, 'a> {
    deserializer: &'a mut TsonDeserializer<'de>,
}

impl<'de, 'a> SeqAccess<'de> for TsonSeqAccess<'de, 'a> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        let next_token = self.deserializer.reader.peek();
        if next_token.is_none() || next_token == Some(&Token::OhYeah) {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.deserializer).map(Some)
        }
    }
}
