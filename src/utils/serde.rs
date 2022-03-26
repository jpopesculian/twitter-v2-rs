use reqwest::StatusCode;
use serde::de::Error as DeError;
use serde::de::Visitor;
use serde::ser::Impossible;
use serde::{Deserializer, Serialize, Serializer};
use std::convert::TryInto;
use std::fmt;

struct NullChecker;

macro_rules! null {
    ($name:ident $($ty:ty),*: impossible) => {
        fn $name(self, $(_: $ty),*) -> Result<Impossible<Self::Ok, Self::Error>, Self::Error> {
            Err(Default::default())
        }
    };
    ($name:ident $($ty:ty),*: $bool:literal) => {
        fn $name(self, $(_: $ty),*) -> Result<Self::Ok, Self::Error> {
            Ok($bool)
        }
    };
    ($name:ident T $($ty:ty),*: impossible) => {
        fn $name<T: ?Sized>(self, $(_: $ty),*) -> Result<Impossible<Self::Ok, Self::Error>, Self::Error> where T: Serialize {
            Ok(Impossible<Self::Ok, Self::Error>)
        }
    };
    ($name:ident T $($ty:ty),*: $bool:literal) => {
        fn $name<T: ?Sized>(self, $(_: $ty),*) -> Result<Self::Ok, Self::Error> where T: Serialize {
            Ok($bool)
        }
    };
}

impl Serializer for NullChecker {
    type Ok = bool;
    type Error = fmt::Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;
    null!(serialize_unit: true);
    null!(serialize_none: true);
    null!(serialize_some T &T: false);
    null!(serialize_bool bool: false);
    null!(serialize_bytes &[u8]: false);
    null!(serialize_char char: false);
    null!(serialize_str &str: false);
    null!(serialize_i8 i8: false);
    null!(serialize_i16 i16: false);
    null!(serialize_i32 i32: false);
    null!(serialize_i64 i64: false);
    null!(serialize_u8 u8: false);
    null!(serialize_u16 u16: false);
    null!(serialize_u32 u32: false);
    null!(serialize_u64 u64: false);
    null!(serialize_f32 f32: false);
    null!(serialize_f64 f64: false);
    null!(serialize_unit_struct &'static str: false);
    null!(serialize_unit_variant &'static str, u32, &'static str: false);
    null!(serialize_newtype_struct T &'static str, &T: false);
    null!(serialize_newtype_variant T &'static str, u32, &'static str, &T: false);
    null!(serialize_seq Option<usize>: impossible);
    null!(serialize_tuple usize: impossible);
    null!(serialize_tuple_struct &'static str, usize: impossible);
    null!(serialize_tuple_variant &'static str, u32, &'static str, usize: impossible);
    null!(serialize_map Option<usize>: impossible);
    null!(serialize_struct &'static str, usize: impossible);
    null!(serialize_struct_variant &'static str, u32, &'static str, usize: impossible);
}

pub fn is_null<T: Serialize>(item: &T) -> bool {
    matches!(item.serialize(NullChecker), Ok(true))
}

pub mod status_code {
    use super::*;

    // serialize StatusCode as u16
    pub fn serialize<S: Serializer>(
        status_code: &StatusCode,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        status_code.as_u16().serialize(serializer)
    }

    struct StatusCodeVisitor;

    impl<'de> Visitor<'de> for StatusCodeVisitor {
        type Value = StatusCode;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter
                .write_str("expecting a valid HTTP status code (status-code in RFC 7230 et al.)")
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            (v as u16).try_into().map_err(E::custom)
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            (v as u16).try_into().map_err(E::custom)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            v.try_into().map_err(E::custom)
        }
        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            v.try_into().map_err(E::custom)
        }
    }

    // deserialize StatusCode from u16
    pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<StatusCode, D::Error> {
        deserializer.deserialize_any(StatusCodeVisitor)
    }
}
