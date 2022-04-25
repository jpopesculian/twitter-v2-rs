use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WithheldScope {
    Tweet,
    User,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Withheld {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<bool>,
    pub country_codes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<WithheldScope>,
}
