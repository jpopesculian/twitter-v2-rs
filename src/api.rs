use crate::authorization::Authorization;
use crate::data::{Tweet, User};
use crate::error::{ApiResult, Result};
use crate::expansions::TweetExpansion;
use crate::fields::Field;
use crate::id::ToId;
use crate::query::{FieldsToQuery, ToQuery};
use crate::requests::DraftTweet;
use reqwest::header::AUTHORIZATION;
use reqwest::Method;
use reqwest::{Client, Url};

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

    async fn send(&self, req: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);
        Ok(self.client.execute(req).await?)
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
        Ok(self.send(req).await?.error_for_status()?.json().await?)
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
        Ok(self.send(req).await?.error_for_status()?.json().await?)
    }

    pub async fn post_tweet(&self, tweet: &DraftTweet) -> ApiResult<Tweet> {
        Ok(self
            .send(self.request(Method::POST, "tweets")?.json(tweet))
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_users_me(&self) -> ApiResult<User> {
        Ok(self
            .send(self.request(Method::GET, "users/me")?)
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
