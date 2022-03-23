use super::Authorization;
use crate::error::{Error, Result};
use async_trait::async_trait;
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl, StandardRevocableToken,
    TokenResponse, TokenUrl,
};
use reqwest::header::HeaderValue;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use std::time::SystemTime;
use strum::{Display, EnumString};
use tokio::sync::{RwLock, RwLockReadGuard};
use url::Url;

#[derive(Copy, Clone, Debug, EnumString, Display, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum Scope {
    #[strum(serialize = "tweet.read")]
    TweetRead,
    #[strum(serialize = "tweet.write")]
    TweetWrite,
    #[strum(serialize = "tweet.moderate.write")]
    TweetModerateWrite,
    #[strum(serialize = "users.read")]
    UsersRead,
    #[strum(serialize = "follows.read")]
    FollowsRead,
    #[strum(serialize = "follows.write")]
    FollowsWrite,
    #[strum(serialize = "offline.access")]
    OfflineAccess,
    #[strum(serialize = "space.read")]
    SpaceRead,
    #[strum(serialize = "mute.read")]
    MuteRead,
    #[strum(serialize = "mute.write")]
    MuteWrite,
    #[strum(serialize = "like.read")]
    LikeRead,
    #[strum(serialize = "like.write")]
    LikeWrite,
    #[strum(serialize = "list.read")]
    ListRead,
    #[strum(serialize = "list.write")]
    ListWrite,
    #[strum(serialize = "block.read")]
    BlockRead,
    #[strum(serialize = "block.write")]
    BlockWrite,
}

impl From<Scope> for oauth2::Scope {
    fn from(scope: Scope) -> Self {
        oauth2::Scope::new(scope.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct Oauth2Client(BasicClient);

impl Oauth2Client {
    pub fn new(client_id: impl ToString, client_secret: impl ToString, callback_url: Url) -> Self {
        Self(
            BasicClient::new(
                ClientId::new(client_id.to_string()),
                Some(ClientSecret::new(client_secret.to_string())),
                AuthUrl::from_url("https://twitter.com/i/oauth2/authorize".parse().unwrap()),
                Some(TokenUrl::from_url(
                    "https://api.twitter.com/2/oauth2/token".parse().unwrap(),
                )),
            )
            .set_revocation_uri(RevocationUrl::from_url(
                "https://api.twitter.com/2/oauth2/revoke".parse().unwrap(),
            ))
            .set_redirect_uri(RedirectUrl::from_url(callback_url)),
        )
    }

    pub fn auth_url(
        &self,
        challenge: PkceCodeChallenge,
        scopes: impl IntoIterator<Item = Scope>,
    ) -> (Url, CsrfToken) {
        self.0
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(challenge)
            .add_scopes(scopes.into_iter().map(|s| s.into()))
            .url()
    }

    pub async fn request_token(
        &self,
        code: AuthorizationCode,
        verifier: PkceCodeVerifier,
    ) -> Result<RefreshableOauth2Token> {
        let res = self
            .0
            .exchange_code(code)
            .set_pkce_verifier(verifier)
            .request_async(oauth2::reqwest::async_http_client)
            .await?;
        Ok(RefreshableOauth2Token::new(self.clone(), res.try_into()?))
    }

    pub async fn revoke_token(&self, token: StandardRevocableToken) -> Result<()> {
        Ok(self
            .0
            .revoke_token(token)
            .unwrap()
            .request_async(oauth2::reqwest::async_http_client)
            .await?)
    }

    pub async fn refresh_token(&self, token: &RefreshToken) -> Result<BasicTokenResponse> {
        Ok(self
            .0
            .exchange_refresh_token(token)
            .request_async(oauth2::reqwest::async_http_client)
            .await?)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2Token {
    access_token: AccessToken,
    refresh_token: Option<RefreshToken>,
    expires: SystemTime,
    scopes: Vec<Scope>,
}

impl Oauth2Token {
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }
    pub fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }
    pub fn expires(&self) -> SystemTime {
        self.expires
    }
    pub fn is_expired(&self) -> bool {
        self.expires < SystemTime::now()
    }
    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }
    pub fn revokable_token(&self) -> StandardRevocableToken {
        if let Some(refresh_token) = self.refresh_token.as_ref() {
            StandardRevocableToken::RefreshToken(refresh_token.clone())
        } else {
            StandardRevocableToken::AccessToken(self.access_token.clone())
        }
    }
}

impl TryFrom<BasicTokenResponse> for Oauth2Token {
    type Error = Error;
    fn try_from(token: BasicTokenResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            access_token: token.access_token().clone(),
            refresh_token: token.refresh_token().cloned(),
            expires: SystemTime::now()
                + token.expires_in().ok_or_else(|| {
                    Error::Oauth2TokenError(BasicRequestTokenError::Other(
                        "Missing expiration".to_string(),
                    ))
                })?,
            scopes: token
                .scopes()
                .ok_or_else(|| {
                    Error::Oauth2TokenError(BasicRequestTokenError::Other(
                        "Missing scopes".to_string(),
                    ))
                })?
                .iter()
                .map(|s| {
                    s.parse().map_err(|err| {
                        Error::Oauth2TokenError(BasicRequestTokenError::Other(format!(
                            "Invalid scope: {}",
                            err
                        )))
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[async_trait]
impl Authorization for Oauth2Token {
    async fn header(&self, _request: &Request) -> Result<HeaderValue> {
        format!("Bearer {}", self.access_token().secret())
            .parse()
            .map_err(Error::InvalidAuthorizationHeader)
    }
}

#[derive(Clone, Debug)]
pub struct RefreshableOauth2Token {
    oauth_client: Oauth2Client,
    token: Arc<RwLock<Oauth2Token>>,
}

impl RefreshableOauth2Token {
    pub fn new(oauth_client: Oauth2Client, token: Oauth2Token) -> Self {
        Self {
            oauth_client,
            token: Arc::new(RwLock::new(token)),
        }
    }

    pub async fn token(&self) -> RwLockReadGuard<'_, Oauth2Token> {
        self.token.read().await
    }

    pub async fn revoke(&self) -> Result<()> {
        self.oauth_client
            .revoke_token(self.token.read().await.revokable_token())
            .await
    }

    pub async fn refresh(&self) -> Result<()> {
        let mut token = self.token.write().await;
        let res = self
            .oauth_client
            .refresh_token(token.refresh_token.as_ref().ok_or(Error::NoRefreshToken)?)
            .await?;
        *token = res.try_into()?;
        Ok(())
    }
}

#[async_trait]
impl Authorization for RefreshableOauth2Token {
    async fn header(&self, request: &Request) -> Result<HeaderValue> {
        let mut token = self.token.write().await;
        if token.is_expired() {
            let res = self
                .oauth_client
                .refresh_token(token.refresh_token.as_ref().ok_or(Error::NoRefreshToken)?)
                .await?;
            *token = res.try_into()?;
        }
        token.header(request).await
    }
}
