use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_keys: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnnotationDescription {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnnotation {
    pub domain: ContextAnnotationDescription,
    pub entity: ContextAnnotationDescription,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlImage {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct HashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnnotationEntity {
    pub start: usize,
    pub end: usize,
    pub probability: f32,
    #[serde(rename = "type")]
    pub kind: String,
    pub normalized_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CashtagEntity {
    pub start: usize,
    pub end: usize,
    pub tag: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionEntity {
    pub start: usize,
    pub end: usize,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entitites {
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

#[derive(Debug, Serialize, Deserialize)]
pub enum GeoCoordinatesKind {
    Point,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoCoordinates {
    #[serde(rename = "type")]
    pub kind: GeoCoordinatesKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<(usize, usize)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    pub place_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<GeoCoordinates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonPublicMetrics {
    pub impression_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_link_clicks: Option<usize>,
    pub user_profile_clicks: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganicOrPromotedMetrics {
    pub impression_count: usize,
    pub like_count: usize,
    pub retweet_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_link_clicks: Option<usize>,
    pub user_profile_clicks: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    pub quote_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetType {
    Quoted,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetKind {
    Quoted,
    RepliedTo,
    Retweeted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub kind: ReferencedTweetKind,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplySettings {
    Everyone,
    MentionedUsers,
    Followers,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "Twitter Web App")]
    TwitterWebApp,
    #[serde(rename = "Twitter for iPhone")]
    TwitterForIPhone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Withheld {
    pub copyright: bool,
    pub country_codes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entitites>,
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
