mod authentication;
mod data;
mod error;
mod expansions;
mod fields;
mod id;
mod query;
mod requests;

pub use authentication::Authentication;
pub use data::*;
pub use error::*;
pub use requests::*;

use authentication::RequestAuth;
use expansions::TweetExpansion;
use fields::Field;
use id::ToId;
use query::{FieldsToQuery, ToQuery};
use reqwest::Method;
use reqwest::{Client, Url};

#[derive(Clone, Debug)]
pub struct TwitterApi {
    client: Client,
    base_url: Url,
    auth: Authentication,
}

impl TwitterApi {
    pub fn new(auth: Authentication) -> Self {
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
        req.authenticate(&self.auth)?;
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
    ) -> ApiResult<Vec<Tweet>> {
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
        let res = self
            .send(self.request(Method::POST, "tweets")?.json(tweet))
            .await?;
        // .error_for_status()?;
        let body = res.text().await?;
        println!("{:?}", body);
        Ok(serde_json::from_str(&body).unwrap())
    }

    pub async fn get_users_me(&self) -> ApiResult<Tweet> {
        let res = self.send(self.request(Method::GET, "users/me")?).await?;
        // .error_for_status()?;
        let body = res.text().await?;
        println!("{:?}", body);
        Ok(serde_json::from_str(&body).unwrap())
    }
}
