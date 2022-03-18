use crate::data::ReplySettings;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DraftTweetGeo {
    pub place_id: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DraftTweetMedia {
    pub media_ids: Vec<String>,
    pub tagged_user_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DraftTweetPoll {
    pub options: Vec<String>,
    pub duration_minutes: usize,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DraftTweetReply {
    pub exclude_reply_user_ids: Option<Vec<String>>,
    pub in_reply_to_tweet_id: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DraftTweet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_message_deep_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_super_followers_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<DraftTweetGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<DraftTweetMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<DraftTweetPoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_tweet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
