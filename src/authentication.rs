use crate::error::{Error, Result};
use async_trait::async_trait;
use reqwest::header::HeaderValue;
use reqwest::Request;
use std::collections::BTreeSet;
use std::fmt;

#[async_trait]
pub trait Authorization {
    async fn header(&self, request: &Request) -> Result<HeaderValue>;
}

#[derive(Clone)]
pub struct BearerToken(String);

impl BearerToken {
    pub fn new(bearer: impl ToString) -> Self {
        Self(bearer.to_string())
    }
}

impl fmt::Debug for BearerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bearer").finish()
    }
}

#[async_trait]
impl Authorization for BearerToken {
    async fn header(&self, request: &Request) -> Result<HeaderValue> {
        format!("Bearer {}", self.0)
            .parse()
            .map_err(Error::InvalidAuthorizationHeader)
    }
}

#[derive(Clone)]
pub struct Oauth1aToken(oauth1::Token);

impl Oauth1aToken {
    pub fn new(
        consumer_key: impl ToString,
        consumer_secret: impl ToString,
        token: impl ToString,
        secret: impl ToString,
    ) -> Self {
        Self(oauth1::Token::from_parts(
            consumer_key.to_string(),
            consumer_secret.to_string(),
            token.to_string(),
            secret.to_string(),
        ))
    }
}
impl fmt::Debug for Oauth1aToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Oauth1a")
            .field("consumer_key", &self.0.client.identifier)
            .field("token", &self.0.token.identifier)
            .finish()
    }
}

#[async_trait]
impl Authorization for Oauth1aToken {
    async fn header(&self, request: &Request) -> Result<HeaderValue> {
        let method = request.method().as_str();
        let url = {
            let mut url = request.url().clone();
            url.set_query(None);
            url.set_fragment(None);
            url
        };
        let request = request.url().query_pairs().collect::<BTreeSet<_>>();
        oauth1::authorize(method, url, &request, &self.0, oauth1::HmacSha1)
            .parse()
            .map_err(Error::InvalidAuthorizationHeader)
    }
}
