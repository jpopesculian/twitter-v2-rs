use crate::error::{Error, Result};
use reqwest::header::AUTHORIZATION;
use reqwest::Request;
use std::collections::BTreeSet;
use std::fmt;

#[derive(Clone)]
pub enum Authentication {
    Bearer(String),
    Oauth1a {
        consumer_key: String,
        consumer_secret: String,
        token: String,
        secret: String,
    },
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bearer(..) => f.debug_tuple("Bearer").finish(),
            Self::Oauth1a {
                consumer_key,
                token,
                ..
            } => f
                .debug_struct("Oauth1a")
                .field("consumer_key", consumer_key)
                .field("token", token)
                .finish(),
        }
    }
}

pub trait RequestAuth {
    fn authenticate(&mut self, authentication: &Authentication) -> Result<()>;
}

impl RequestAuth for Request {
    fn authenticate(&mut self, authentication: &Authentication) -> Result<()> {
        let header = match authentication {
            Authentication::Bearer(bearer) => {
                format!("Bearer {bearer}")
            }
            Authentication::Oauth1a {
                consumer_key,
                consumer_secret,
                token,
                secret,
            } => {
                let method = self.method().as_str();
                let url = {
                    let mut url = self.url().clone();
                    url.set_query(None);
                    url.set_fragment(None);
                    url
                };
                let request = self.url().query_pairs().collect::<BTreeSet<_>>();
                let token = oauth1::Token::from_parts(consumer_key, consumer_secret, token, secret);
                oauth1::authorize(method, url, &request, &token, oauth1::HmacSha1)
            }
        };
        let _ = self.headers_mut().insert(
            AUTHORIZATION,
            header.parse().map_err(Error::InvalidAuthorizationHeader)?,
        );
        Ok(())
    }
}
