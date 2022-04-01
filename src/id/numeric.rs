use serde::de::Visitor;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NumericId(u64);

impl NumericId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<NumericId> for u64 {
    fn from(id: NumericId) -> Self {
        id.0
    }
}

impl From<u64> for NumericId {
    fn from(id: u64) -> Self {
        NumericId(id)
    }
}

impl<'a> From<&'a u64> for NumericId {
    fn from(id: &'a u64) -> Self {
        NumericId(*id)
    }
}

impl FromStr for NumericId {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl fmt::Display for NumericId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl PartialEq<u64> for NumericId {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}
impl PartialEq<&u64> for NumericId {
    fn eq(&self, other: &&u64) -> bool {
        &self.0 == *other
    }
}

impl Serialize for NumericId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct NumericIdVisitor;

impl<'de> Visitor<'de> for NumericIdVisitor {
    type Value = NumericId;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a valid unsigned integer or string representing an id")
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NumericId(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v < 0 {
            Err(E::custom("value must be an unsigned integer"))
        } else {
            Ok(NumericId(v as u64))
        }
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse().map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for NumericId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(NumericIdVisitor)
    }
}

pub trait IntoNumericId: fmt::Display {
    fn into_id(self) -> NumericId;
}

impl IntoNumericId for NumericId {
    fn into_id(self) -> NumericId {
        self
    }
}

impl IntoNumericId for u64 {
    fn into_id(self) -> NumericId {
        NumericId(self)
    }
}

impl<'a> IntoNumericId for &'a u64 {
    fn into_id(self) -> NumericId {
        NumericId(*self)
    }
}
