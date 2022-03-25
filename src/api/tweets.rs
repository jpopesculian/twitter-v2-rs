use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Deleted, Tweet};
use crate::error::Result;
use crate::id::ToId;
use crate::query::get_req_builder;
use crate::query::ToQuery;
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

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_tweets(
        &self,
        ids: impl IntoIterator<Item = impl ToId>,
    ) -> Result<GetTweetsRequestBuilder<A, Vec<Tweet>>> {
        Ok(GetTweetsRequestBuilder::new(
            self,
            self.request(Method::GET, "tweets")?
                .query(&ids.to_query("ids")),
        ))
    }
    pub fn get_tweet(&self, id: impl ToId) -> Result<GetTweetsRequestBuilder<A, Tweet>> {
        Ok(GetTweetsRequestBuilder::new(
            self,
            self.request(Method::GET, &format!("tweets/{id}"))?,
        ))
    }
    pub async fn post_tweet(&self, tweet: &DraftTweet) -> ApiResult<Tweet> {
        self.send(self.request(Method::POST, "tweets")?.json(tweet))
            .await
    }

    pub async fn delete_tweet(&self, id: impl ToId) -> ApiResult<Deleted> {
        self.send(self.request(Method::DELETE, &format!("tweets/{id}"))?)
            .await
    }
}
