use crate::api_result::ApiError;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Api(#[from] ApiError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("Invalid Authorization header value: {_0}")]
    InvalidAuthorizationHeader(InvalidHeaderValue),
    #[cfg(feature = "oauth2")]
    #[error(transparent)]
    Oauth2TokenError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::basic::BasicErrorResponse,
        >,
    ),
    #[cfg(feature = "oauth2")]
    #[error(transparent)]
    Oauth2RevocationError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::basic::BasicRevocationErrorResponse,
        >,
    ),
    #[cfg(feature = "oauth2")]
    #[error("No refresh token found. Try using the `offline.access` scope")]
    NoRefreshToken,
    #[error("Other: {_0}")]
    Custom(String),
}

impl Error {
    pub fn custom(message: impl ToString) -> Self {
        Self::Custom(message.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
