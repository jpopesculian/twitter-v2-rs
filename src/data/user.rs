use super::entity::{FullTextEntities, UrlEntity};
use super::withheld::Withheld;
use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserUrlEntites {
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<UrlEntity>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<UserUrlEntites>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<FullTextEntities>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPublicMetrics {
    followers_count: usize,
    following_count: usize,
    tweet_count: usize,
    listed_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: NumericId,
    pub name: String,
    pub username: String,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<UserEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_tweet_id: Option<NumericId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<UserPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
}
