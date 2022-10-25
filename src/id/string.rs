use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringId(String);

impl StringId {
    pub fn new(id: String) -> Self {
        Self(id)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<StringId> for String {
    fn from(id: StringId) -> Self {
        id.0
    }
}

impl From<String> for StringId {
    fn from(id: String) -> Self {
        StringId(id)
    }
}

impl<'a> From<&'a String> for StringId {
    fn from(id: &'a String) -> Self {
        StringId(id.to_string())
    }
}

impl<'a> From<&'a str> for StringId {
    fn from(id: &'a str) -> Self {
        StringId(id.to_string())
    }
}

impl FromStr for StringId {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for StringId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<String> for StringId {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}
impl PartialEq<str> for StringId {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
impl PartialEq<&str> for StringId {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl Serialize for StringId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for StringId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(StringId(String::deserialize(deserializer)?))
    }
}

pub trait IntoStringId: fmt::Display {
    fn into_id(self) -> StringId;
}

impl IntoStringId for StringId {
    fn into_id(self) -> StringId {
        self
    }
}

impl IntoStringId for String {
    fn into_id(self) -> StringId {
        StringId(self)
    }
}

impl<'a> IntoStringId for &'a String {
    fn into_id(self) -> StringId {
        StringId(self.to_string())
    }
}

impl<'a> IntoStringId for &'a str {
    fn into_id(self) -> StringId {
        StringId(self.to_string())
    }
}
