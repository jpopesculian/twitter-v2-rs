use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachments {
    pub media_keys: Option<Vec<String>>,
    pub poll_ids: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnnotationDescription {
    pub id: String,
    pub name: String,
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
    pub images: Option<Vec<UrlImage>>,
    pub status: Option<usize>,
    pub title: Option<String>,
    pub description: Option<String>,
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
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entitites {
    pub urls: Option<Vec<UrlEntity>>,
    pub hashtags: Option<Vec<HashtagEntity>>,
    pub annotations: Option<Vec<AnnotationEntity>>,
    pub cashtags: Option<Vec<CashtagEntity>>,
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
    pub coordinates: Option<(usize, usize)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    pub place_id: String,
    pub coordinates: Option<GeoCoordinates>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NonPublicMetrics {
    pub impression_count: usize,
    pub url_link_clicks: Option<usize>,
    pub user_profile_clicks: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrganicOrPromotedMetrics {
    pub impression_count: usize,
    pub like_count: usize,
    pub retweet_count: usize,
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
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub attachments: Option<Attachments>,
    pub author_id: Option<String>,
    pub context_annotations: Option<Vec<ContextAnnotation>>,
    pub conversation_id: Option<String>,
    pub entities: Option<Entitites>,
    pub geo: Option<Geo>,
    pub in_reply_to_user_id: Option<String>,
    pub lang: Option<String>,
    pub non_public_metrics: Option<NonPublicMetrics>,
    pub organic_metrics: Option<OrganicOrPromotedMetrics>,
    pub possibly_sensitive: Option<bool>,
    pub promoted_metrics: Option<OrganicOrPromotedMetrics>,
    pub public_metrics: Option<PublicMetrics>,
    pub referenced_tweets: Option<Vec<ReferencedTweet>>,
    pub reply_settings: Option<ReplySettings>,
    pub source: Option<Source>,
    pub withheld: Option<Withheld>,
}
