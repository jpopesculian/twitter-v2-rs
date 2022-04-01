use crate::id::NumericId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct StreamRule {
    pub id: NumericId,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
