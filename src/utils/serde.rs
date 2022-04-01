use serde::de::Error as DeError;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub mod status_code {
    use super::*;
    use reqwest::StatusCode;
    use std::convert::TryInto;

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

pub mod option_duration_ms {
    use super::*;
    use time::Duration;

    // serialize Duration as ms
    pub fn serialize<S: Serializer>(
        duration: &Option<Duration>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        duration
            .map(Duration::whole_milliseconds)
            .serialize(serializer)
    }

    // deserialize Duration from ms
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<Duration>, D::Error> {
        Ok(Option::<i64>::deserialize(deserializer)?.map(Duration::milliseconds))
    }
}

pub mod option_duration_mins {
    use super::*;
    use time::Duration;

    // serialize Duration as ms
    pub fn serialize<S: Serializer>(
        duration: &Option<Duration>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        duration.map(Duration::whole_minutes).serialize(serializer)
    }

    // deserialize Duration from ms
    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<Duration>, D::Error> {
        Ok(Option::<i64>::deserialize(deserializer)?.map(Duration::minutes))
    }
}
