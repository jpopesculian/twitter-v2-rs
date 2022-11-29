use super::entity::{FullTextEntities, UrlEntity};
use super::withheld::Withheld;
use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserUrlEntites {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<UrlEntity>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
pub struct UserEntities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<UserUrlEntites>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FullTextEntities>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserPublicMetrics {
    pub followers_count: usize,
    pub following_count: usize,
    pub tweet_count: usize,
    pub listed_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "arbitrary_precision", derive(Eq))]
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::serde::empty_string_is_none"
    )]
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
