use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Deleted, Retweeted, StreamRule, Tweet, TweetsCount, User};
use crate::id::IntoId;
use crate::meta::{ResultCountMeta, SentMeta, TweetsCountsMeta, TweetsMeta};
use crate::query::{get_req_builder, UrlQueryExt};
use crate::requests::{StreamRuleBuilder, TweetBuilder, TweetId};
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

get_req_builder! {
pub struct GetTweetsSearchRequestBuilder {
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
    sort_order,
    pagination_token
}
}

get_req_builder! {
pub struct GetTweetsCountsRequestBuilder {
    start_time,
    end_time,
    since_id,
    until_id,
    granularity
}
}

get_req_builder! {
pub struct GetStreamRulesRequestBuilder {
    ids
}
}

get_req_builder! {
#[stream]
pub struct GetTweetsStreamRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    backfill
}
}

get_req_builder! {
pub struct GetRetweetedByRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    user_expansions,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetQuoteTweetsRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
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
        ids: impl IntoIterator<Item = impl IntoId>,
    ) -> GetTweetsRequestBuilder<A, Vec<Tweet>, ()> {
        let mut url = self.url("tweets").unwrap();
        url.append_query_seq("ids", ids);
        GetTweetsRequestBuilder::new(self, url)
    }
    pub fn get_tweet(&self, id: impl IntoId) -> GetTweetsRequestBuilder<A, Tweet, ()> {
        GetTweetsRequestBuilder::new(self, self.url(format!("tweets/{id}")).unwrap())
    }
    pub fn post_tweet(&self) -> TweetBuilder<A> {
        TweetBuilder::new(self, self.url("tweets").unwrap())
    }
    pub async fn delete_tweet(&self, id: impl IntoId) -> ApiResult<A, Deleted, ()> {
        self.send(self.request(Method::DELETE, self.url(format!("tweets/{id}"))?))
            .await
    }
    pub fn get_user_tweets(
        &self,
        user_id: impl IntoId,
    ) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        GetTimelineRequestBuilder::new(self, self.url(format!("users/{user_id}/tweets")).unwrap())
    }
    pub fn get_user_mentions(
        &self,
        user_id: impl IntoId,
    ) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        GetTimelineRequestBuilder::new(self, self.url(format!("users/{user_id}/mentions")).unwrap())
    }
    pub fn get_tweets_search_recent(
        &self,
        query: impl ToString,
    ) -> GetTweetsSearchRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        let mut url = self.url("tweets/search/recent").unwrap();
        url.append_query_val("query", query);
        GetTweetsSearchRequestBuilder::new(self, url)
    }
    pub fn get_tweets_search_all(
        &self,
        query: impl ToString,
    ) -> GetTweetsSearchRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        let mut url = self.url("tweets/search/all").unwrap();
        url.append_query_val("query", query);
        GetTweetsSearchRequestBuilder::new(self, url)
    }
    pub fn get_tweets_counts_recent(
        &self,
        query: impl ToString,
    ) -> GetTweetsCountsRequestBuilder<A, Vec<TweetsCount>, TweetsCountsMeta> {
        let mut url = self.url("tweets/counts/recent").unwrap();
        url.append_query_val("query", query);
        GetTweetsCountsRequestBuilder::new(self, url)
    }
    pub fn get_tweets_counts_all(
        &self,
        query: impl ToString,
    ) -> GetTweetsCountsRequestBuilder<A, Vec<TweetsCount>, TweetsCountsMeta> {
        let mut url = self.url("tweets/counts/all").unwrap();
        url.append_query_val("query", query);
        GetTweetsCountsRequestBuilder::new(self, url)
    }
    pub fn get_tweets_search_stream_rules(
        &self,
    ) -> GetStreamRulesRequestBuilder<A, Vec<StreamRule>, SentMeta> {
        GetStreamRulesRequestBuilder::new(self, self.url("tweets/search/stream/rules").unwrap())
    }
    pub fn get_tweets_search_stream(&self) -> GetTweetsStreamRequestBuilder<A, Tweet, SentMeta> {
        GetTweetsStreamRequestBuilder::new(self, self.url("tweets/search/stream").unwrap())
    }
    pub fn post_tweets_search_stream_rule(&self) -> StreamRuleBuilder<A> {
        StreamRuleBuilder::new(self, self.url("tweets/search/stream/rules").unwrap())
    }
    pub fn get_tweets_sample_stream(&self) -> GetTweetsStreamRequestBuilder<A, Tweet, SentMeta> {
        GetTweetsStreamRequestBuilder::new(self, self.url("tweets/sample/stream").unwrap())
    }
    pub fn get_tweet_retweeted_by(
        &self,
        id: impl IntoId,
    ) -> GetRetweetedByRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRetweetedByRequestBuilder::new(
            self,
            self.url(format!("tweets/{id}/retweeted_by")).unwrap(),
        )
    }
    pub async fn post_user_retweet(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Retweeted, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{user_id}/retweets"))?)
                .json(&TweetId::from(tweet_id)),
        )
        .await
    }
    pub async fn delete_user_retweet(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Retweeted, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{user_id}/retweets/{tweet_id}"))?,
        ))
        .await
    }
    pub fn get_tweet_quote_tweets(
        &self,
        id: impl IntoId,
    ) -> GetQuoteTweetsRequestBuilder<A, Vec<Tweet>, ResultCountMeta> {
        GetQuoteTweetsRequestBuilder::new(
            self,
            self.url(format!("tweets/{id}/quote_tweets")).unwrap(),
        )
    }
}
