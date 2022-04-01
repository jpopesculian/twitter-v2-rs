use super::GeoFeature;
use crate::id::StringId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Place {
    pub id: StringId,
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contained_within: Option<Vec<StringId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<GeoFeature>,
}
