use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NonPublicMetrics {
    pub impression_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_link_clicks: Option<usize>,
    pub user_profile_clicks: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicOrPromotedMetrics {
    pub impression_count: usize,
    pub like_count: usize,
    pub retweet_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_link_clicks: Option<usize>,
    pub user_profile_clicks: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicMetrics {
    pub retweet_count: usize,
    pub reply_count: usize,
    pub like_count: usize,
    pub quote_count: usize,
}
