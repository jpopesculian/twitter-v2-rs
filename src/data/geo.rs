use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(f64, f64)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum GeoFeatureKind {
    Feature,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GeoFeature {
    #[serde(rename = "type")]
    pub kind: GeoFeatureKind,
    pub bbox: [f64; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeoCoordinates>,
    pub properties: HashMap<String, serde_json::Value>,
}
