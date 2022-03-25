use crate::api_result::{ApiResponseExt, ApiResult};
use crate::authorization::Authorization;
use crate::data::{Deleted, Tweet, User};
use crate::error::Result;
use crate::expansions::TweetExpansion;
use crate::fields::Field;
use crate::id::ToId;
use crate::query::{FieldsToQuery, ToQuery};
use crate::requests::DraftTweet;
use reqwest::header::AUTHORIZATION;
use reqwest::Method;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;

#[derive(Clone, Debug)]
pub struct TwitterApi<A> {
    client: Client,
    base_url: Url,
    auth: A,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn new(auth: A) -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse("https://api.twitter.com/2/").unwrap(),
            auth,
        }
    }

    fn request(&self, method: Method, url: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self.client.request(method, self.base_url.join(url)?))
    }

    async fn send<T: DeserializeOwned>(&self, req: reqwest::RequestBuilder) -> ApiResult<T> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);
        self.client.execute(req).await?.api_json().await
    }

    pub async fn get_tweets(
        &self,
        ids: impl IntoIterator<Item = impl ToId>,
        fields: Option<&[Field]>,
        expansions: Option<&[TweetExpansion]>,
    ) -> ApiResult<Vec<Tweet>> {
        let mut req = self
            .request(Method::GET, "tweets")?
            .query(&ids.to_query("ids"));
        if let Some(fields) = fields {
            req = req.query(&fields.to_fields_query());
        }
        if let Some(expansions) = expansions {
            req = req.query(&expansions.to_query("expansions"));
        }
        self.send(req).await
    }

    pub async fn get_tweet(
        &self,
        id: impl ToId,
        fields: Option<&[Field]>,
        expansions: Option<&[TweetExpansion]>,
    ) -> ApiResult<Tweet> {
        let mut req = self.request(Method::GET, &format!("tweets/{id}"))?;
        if let Some(fields) = fields {
            req = req.query(&fields.to_fields_query());
        }
        if let Some(expansions) = expansions {
            req = req.query(&expansions.to_query("expansions"));
        }
        self.send(req).await
    }

    pub async fn post_tweet(&self, tweet: &DraftTweet) -> ApiResult<Tweet> {
        self.send(self.request(Method::POST, "tweets")?.json(tweet))
            .await
    }

    pub async fn delete_tweet(&self, id: impl ToId) -> ApiResult<Deleted> {
        self.send(self.request(Method::DELETE, &format!("tweets/{id}"))?)
            .await
    }

    pub async fn get_users_me(&self) -> ApiResult<User> {
        self.send(self.request(Method::GET, "users/me")?).await
    }
}
