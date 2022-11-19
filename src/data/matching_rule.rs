use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MatchingRule {
    pub id: String,
    pub tag: String,
}
