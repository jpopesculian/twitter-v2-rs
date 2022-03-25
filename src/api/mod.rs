mod tweets;
mod users;

use crate::api_result::{ApiResponseExt, ApiResult};
use crate::authorization::Authorization;
use crate::error::Result;
use reqwest::header::AUTHORIZATION;
use reqwest::Method;
use reqwest::{Client, Url};
use serde::de::DeserializeOwned;
use std::sync::Arc;

#[derive(Debug)]
pub struct TwitterApi<A> {
    client: Client,
    base_url: Url,
    auth: Arc<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn new(auth: A) -> Self {
        Self {
            client: Client::new(),
            base_url: Url::parse("https://api.twitter.com/2/").unwrap(),
            auth: Arc::new(auth),
        }
    }

    pub fn request(&self, method: Method, url: &str) -> Result<reqwest::RequestBuilder> {
        Ok(self.client.request(method, self.base_url.join(url)?))
    }

    pub async fn send<T: DeserializeOwned, M: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> ApiResult<T, M> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);
        self.client.execute(req).await?.api_json().await
    }
}

impl<A> Clone for TwitterApi<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            auth: self.auth.clone(),
        }
    }
}
