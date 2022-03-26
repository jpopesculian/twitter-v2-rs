use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TweetsCountsMetaSummary {
    pub created: Option<usize>,
    pub not_created: Option<usize>,
    pub valid: Option<usize>,
    pub invalid: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StreamRuleMeta {
    #[serde(with = "time::serde::rfc3339")]
    pub sent: OffsetDateTime,
    pub summary: TweetsCountsMetaSummary,
}
