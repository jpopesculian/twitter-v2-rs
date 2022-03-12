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
    pub direct_message_deep_link: Option<String>,
    pub for_super_followers_only: Option<bool>,
    pub geo: Option<DraftTweetGeo>,
    pub media: Option<DraftTweetMedia>,
    pub poll: Option<DraftTweetPoll>,
    pub quote_tweet_id: Option<String>,
    pub reply_settings: Option<ReplySettings>,
    pub text: Option<String>,
}
