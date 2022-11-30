use crate::id::NumericId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MatchingRule {
    pub id: NumericId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
