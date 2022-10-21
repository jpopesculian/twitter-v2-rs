use crate::id::StringId;
use serde::{Deserialize, Serialize};
use time::Duration;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MediaType {
    AnimatedGif,
    Photo,
    Video,
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MediaVariant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url : Option<Url>
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MediaPublicMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MediaNonPublicMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_0_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_25_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_50_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_75_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_100_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MediaPromotedOrOrganicMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_0_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_25_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_50_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_75_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_100_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Media {
    pub media_key: StringId,
    #[serde(rename = "type")]
    pub kind: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(
        with = "crate::utils::serde::option_duration_ms",
        rename = "duration_ms",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration: Option<Duration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<MediaPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<MediaNonPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<MediaPromotedOrOrganicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<MediaPromotedOrOrganicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<MediaVariant>>,
}
