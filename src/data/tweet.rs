use super::entity::Entities;
use super::metric::{NonPublicMetrics, OrganicOrPromotedMetrics, PublicMetrics};
use super::withheld::Withheld;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_ids: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextAnnotationDescription {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: ContextAnnotationDescription,
    pub entity: ContextAnnotationDescription,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(usize, usize)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Geo {
    pub place_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GeoCoordinates>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetType {
    Quoted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetKind {
    Quoted,
    RepliedTo,
    Retweeted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub kind: ReferencedTweetKind,
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplySettings {
    Everyone,
    MentionedUsers,
    Followers,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "Twitter Web App")]
    TwitterWebApp,
    #[serde(rename = "Twitter for iPhone")]
    TwitterForIPhone,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
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
    pub in_reply_to_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_public_metrics: Option<NonPublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organic_metrics: Option<OrganicOrPromotedMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub possibly_sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promoted_metrics: Option<OrganicOrPromotedMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_metrics: Option<PublicMetrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withheld: Option<Withheld>,
}
