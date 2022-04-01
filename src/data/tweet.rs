use super::entity::Entities;
use super::withheld::Withheld;
use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetPublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetNonPublicMetrics {
    pub impression_count: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetPromotedMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impression_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweet_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TweetOrganicMetrics {
    pub impression_count: usize,
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Attachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_ids: Option<Vec<NumericId>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContextAnnotationDescription {
    pub id: NumericId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ContextAnnotation {
    pub domain: ContextAnnotationDescription,
    pub entity: ContextAnnotationDescription,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(usize, usize)>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Geo {
    pub place_id: NumericId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GeoCoordinates>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetType {
    Quoted,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetKind {
    Quoted,
    RepliedTo,
    Retweeted,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub kind: ReferencedTweetKind,
    pub id: NumericId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ReplySettings {
    Everyone,
    MentionedUsers,
    Followers,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tweet {
    pub id: NumericId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<NumericId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<NumericId>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_user_id: Option<NumericId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<TweetNonPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<TweetOrganicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<TweetPromotedMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<TweetPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
}
