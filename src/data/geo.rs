use serde_json::Number;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(Number, Number)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Copy)]
pub enum GeoFeatureKind {
    Feature,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GeoFeature {
    #[serde(rename = "type")]
    pub kind: GeoFeatureKind,
    pub bbox: [Number; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeoCoordinates>,
    pub properties: HashMap<String, serde_json::Value>,
}
