use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SentMeta {
    #[serde(with = "time::serde::rfc3339")]
    sent: OffsetDateTime,
}
