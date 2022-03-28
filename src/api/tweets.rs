use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{
    Bookmarked, Deleted, Hidden, Liked, Retweeted, StreamRule, Tweet, TweetsCount, User,
};
use crate::id::IntoId;
use crate::meta::{ResultCountMeta, SentMeta, TweetsCountsMeta, TweetsMeta};
use crate::query::{
    GetRelatedTweetsRequestBuilder, GetStreamRulesRequestBuilder, GetTimelineRequestBuilder,
    GetTweetUsersRequestBuilder, GetTweetsCountsRequestBuilder, GetTweetsRequestBuilder,
    GetTweetsSearchRequestBuilder, GetTweetsStreamRequestBuilder, UrlQueryExt,
};
use crate::requests::{StreamRuleBuilder, TweetBuilder, TweetId};
use reqwest::Method;

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
    ) -> GetTweetUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetTweetUsersRequestBuilder::new(
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
    ) -> GetRelatedTweetsRequestBuilder<A, Vec<Tweet>, ResultCountMeta> {
        GetRelatedTweetsRequestBuilder::new(
            self,
            self.url(format!("tweets/{id}/quote_tweets")).unwrap(),
        )
    }
    pub fn get_tweet_liking_users(
        &self,
        id: impl IntoId,
    ) -> GetTweetUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetTweetUsersRequestBuilder::new(
            self,
            self.url(format!("tweets/{id}/liking_users")).unwrap(),
        )
    }
    pub fn get_user_liked_tweets(
        &self,
        id: impl IntoId,
    ) -> GetRelatedTweetsRequestBuilder<A, Vec<Tweet>, ResultCountMeta> {
        GetRelatedTweetsRequestBuilder::new(
            self,
            self.url(format!("users/{id}/liked_tweets")).unwrap(),
        )
    }
    pub async fn post_user_like(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Liked, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{user_id}/likes"))?)
                .json(&TweetId::from(tweet_id)),
        )
        .await
    }
    pub async fn delete_user_like(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Liked, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{user_id}/likes/{tweet_id}"))?,
        ))
        .await
    }
    pub fn get_user_bookmarks(
        &self,
        id: impl IntoId,
    ) -> GetRelatedTweetsRequestBuilder<A, Vec<Tweet>, ResultCountMeta> {
        GetRelatedTweetsRequestBuilder::new(
            self,
            self.url(format!("users/{id}/bookmarks")).unwrap(),
        )
    }
    pub async fn post_user_bookmark(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Bookmarked, ()> {
        self.send(
            self.request(
                Method::POST,
                self.url(format!("users/{user_id}/bookmarks"))?,
            )
            .json(&TweetId::from(tweet_id)),
        )
        .await
    }
    pub async fn delete_user_bookmark(
        &self,
        user_id: impl IntoId,
        tweet_id: impl IntoId,
    ) -> ApiResult<A, Bookmarked, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{user_id}/bookmarks/{tweet_id}"))?,
        ))
        .await
    }
    pub async fn put_tweet_hidden(
        &self,
        id: impl IntoId,
        hidden: bool,
    ) -> ApiResult<A, Hidden, ()> {
        self.send(
            self.request(Method::PUT, self.url(format!("tweets/{id}/hidden"))?)
                .json(&Hidden::from(hidden)),
        )
        .await
    }
}
