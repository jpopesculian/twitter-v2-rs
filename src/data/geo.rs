#[cfg(feature = "arbitrary_precision")]
use serde_json::Number;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Copy)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[cfg(feature = "arbitrary_precision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(Number, Number)>,
    #[cfg(not(feature = "arbitrary_precision"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(f64, f64)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Copy)]
pub enum GeoFeatureKind {
    Feature,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct GeoFeature {
    #[serde(rename = "type")]
    pub kind: GeoFeatureKind,
    #[cfg(feature = "arbitrary_precision")]
    pub bbox: [Number; 4],
    #[cfg(not(feature = "arbitrary_precision"))]
    pub bbox: [f64; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<GeoCoordinates>,
    pub properties: HashMap<String, serde_json::Value>,
}
