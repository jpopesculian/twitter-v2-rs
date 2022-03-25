use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Deleted, Tweet};
use crate::error::Result;
use crate::meta::TimelineMeta;
use crate::query::{get_req_builder, ToId, ToQuery};
use crate::requests::DraftTweet;
use reqwest::Method;

get_req_builder! {
pub struct GetTweetsRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions
}
}

get_req_builder! {
pub struct GetTimelineRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    exclude,
    start_time,
    end_time,
    since_id,
    until_id,
    max_results,
    pagination_token
}
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_tweets(
        &self,
        ids: impl IntoIterator<Item = impl ToId>,
    ) -> Result<GetTweetsRequestBuilder<A, Vec<Tweet>, Option<()>>> {
        Ok(GetTweetsRequestBuilder::new(
            self,
            self.request(Method::GET, "tweets")?
                .query(&ids.to_query("ids")),
        ))
    }
    pub fn get_tweet(
        &self,
        id: impl ToId,
    ) -> Result<GetTweetsRequestBuilder<A, Tweet, Option<()>>> {
        Ok(GetTweetsRequestBuilder::new(
            self,
            self.request(Method::GET, &format!("tweets/{id}"))?,
        ))
    }
    pub fn get_user_tweets(
        &self,
        user_id: impl ToId,
    ) -> Result<GetTimelineRequestBuilder<A, Vec<Tweet>, TimelineMeta>> {
        Ok(GetTimelineRequestBuilder::new(
            self,
            self.request(Method::GET, &format!("users/{user_id}/tweets"))?,
        ))
    }
    pub fn get_user_mentions(
        &self,
        user_id: impl ToId,
    ) -> Result<GetTimelineRequestBuilder<A, Vec<Tweet>, TimelineMeta>> {
        Ok(GetTimelineRequestBuilder::new(
            self,
            self.request(Method::GET, &format!("users/{user_id}/mentions"))?,
        ))
    }
    pub async fn post_tweet(&self, tweet: &DraftTweet) -> ApiResult<Tweet, Option<()>> {
        self.send(self.request(Method::POST, "tweets")?.json(tweet))
            .await
    }

    pub async fn delete_tweet(&self, id: impl ToId) -> ApiResult<Deleted, Option<()>> {
        self.send(self.request(Method::DELETE, &format!("tweets/{id}"))?)
            .await
    }
}
