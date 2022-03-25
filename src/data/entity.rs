use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UrlImage {
    pub url: Url,
    pub width: usize,
    pub height: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UrlEntity {
    pub start: usize,
    pub end: usize,
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<UrlImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unwound_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnotationEntity {
    pub start: usize,
    pub end: usize,
    pub probability: f32,
    #[serde(rename = "type")]
    pub kind: String,
    pub normalized_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentionEntity {
    pub start: usize,
    pub end: usize,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashtags: Option<Vec<HashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtags: Option<Vec<CashtagEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<MentionEntity>>,
}
