use crate::api::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{ReplySettings, Tweet};
use crate::id::{IntoNumericId, IntoStringId, StringId};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftTweetGeo {
    pub place_id: StringId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftTweetMedia {
    pub media_ids: Vec<String>,
    pub tagged_user_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftTweetPoll {
    pub options: Vec<String>,
    pub duration_minutes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftTweetReply {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_reply_user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to_tweet_id: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct DraftTweet {
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
    pub reply: Option<DraftTweetReply>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_settings: Option<ReplySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug)]
pub struct TweetBuilder<A> {
    client: TwitterApi<A>,
    url: Url,
    tweet: DraftTweet,
}

impl<A> TweetBuilder<A>
where
    A: Authorization,
{
    pub(crate) fn new(client: &TwitterApi<A>, url: Url) -> Self {
        Self {
            client: client.clone(),
            url,
            tweet: Default::default(),
        }
    }
    pub fn text(&mut self, text: String) -> &mut Self {
        self.tweet.text = Some(text);
        self
    }
    pub fn direct_message_deep_link(&mut self, direct_message_deep_link: String) -> &mut Self {
        self.tweet.direct_message_deep_link = Some(direct_message_deep_link);
        self
    }
    pub fn for_super_followers_only(&mut self, for_super_followers_only: bool) -> &mut Self {
        self.tweet.for_super_followers_only = Some(for_super_followers_only);
        self
    }
    pub fn place_id(&mut self, place_id: impl IntoStringId) -> &mut Self {
        if let Some(geo) = self.tweet.geo.as_mut() {
            geo.place_id = place_id.into_id();
        } else {
            self.tweet.geo = Some(DraftTweetGeo {
                place_id: place_id.into_id(),
            });
        }
        self
    }
    pub fn add_media(
        &mut self,
        media_ids: impl IntoIterator<Item = impl IntoNumericId>,
        tagged_user_ids: impl IntoIterator<Item = impl IntoNumericId>,
    ) -> &mut Self {
        if let Some(media) = self.tweet.media.as_mut() {
            media
                .media_ids
                .extend(media_ids.into_iter().map(|id| id.to_string()));
            media
                .tagged_user_ids
                .extend(tagged_user_ids.into_iter().map(|id| id.to_string()));
        } else {
            self.tweet.media = Some(DraftTweetMedia {
                media_ids: media_ids.into_iter().map(|id| id.to_string()).collect(),
                tagged_user_ids: tagged_user_ids
                    .into_iter()
                    .map(|id| id.to_string())
                    .collect(),
            });
        }
        self
    }
    pub fn poll(
        &mut self,
        options: impl IntoIterator<Item = impl ToString>,
        duration: Duration,
    ) -> &mut Self {
        self.tweet.poll = Some(DraftTweetPoll {
            options: options
                .into_iter()
                .map(|option| option.to_string())
                .collect::<Vec<_>>(),
            duration_minutes: duration.as_secs() / 60,
        });
        self
    }
    pub fn quote_tweet_id(&mut self, id: impl IntoNumericId) -> &mut Self {
        self.tweet.quote_tweet_id = Some(id.to_string());
        self
    }
    pub fn add_exclude_reply_user_id(&mut self, user_id: impl IntoNumericId) -> &mut Self {
        self.add_exclude_reply_user_ids([user_id])
    }
    pub fn add_exclude_reply_user_ids(
        &mut self,
        user_ids: impl IntoIterator<Item = impl IntoNumericId>,
    ) -> &mut Self {
        let mut user_ids = user_ids
            .into_iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>();
        if let Some(reply) = self.tweet.reply.as_mut() {
            if let Some(exclude_reply_user_ids) = reply.exclude_reply_user_ids.as_mut() {
                exclude_reply_user_ids.append(&mut user_ids)
            } else {
                reply.exclude_reply_user_ids = Some(user_ids);
            }
        } else {
            self.tweet.reply = Some(DraftTweetReply {
                exclude_reply_user_ids: Some(user_ids),
                in_reply_to_tweet_id: None,
            });
        }
        self
    }
    pub fn in_reply_to_tweet_id(&mut self, user_id: impl IntoNumericId) -> &mut Self {
        if let Some(reply) = self.tweet.reply.as_mut() {
            reply.in_reply_to_tweet_id = Some(user_id.to_string());
        } else {
            self.tweet.reply = Some(DraftTweetReply {
                exclude_reply_user_ids: None,
                in_reply_to_tweet_id: Some(user_id.to_string()),
            });
        }
        self
    }
    pub fn reply_settings(&mut self, reply_settings: ReplySettings) -> &mut Self {
        self.tweet.reply_settings = Some(reply_settings);
        self
    }
    pub async fn send(&self) -> ApiResult<A, Tweet, ()> {
        self.client
            .send(
                self.client
                    .request(Method::POST, self.url.clone())
                    .json(&self.tweet),
            )
            .await
    }
}

impl<A> Clone for TweetBuilder<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            tweet: self.tweet.clone(),
        }
    }
}
