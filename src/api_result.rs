use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse<T, M> {
    pub data: T,
    #[serde(skip_serializing_if = "crate::utils::serde::is_null")]
    pub meta: M,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiError {
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(with = "crate::utils::serde::status_code")]
    pub status: StatusCode,
    pub detail: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{}] {}", self.status, self.detail))
    }
}

impl std::error::Error for ApiError {}

pub type ApiResult<T, M> = Result<ApiResponse<T, M>>;

#[async_trait]
pub(crate) trait ApiResponseExt {
    async fn api_json<T: DeserializeOwned, M: DeserializeOwned>(self) -> ApiResult<T, M>;
}

#[async_trait]
impl ApiResponseExt for Response {
    async fn api_json<T: DeserializeOwned, M: DeserializeOwned>(self) -> ApiResult<T, M> {
        let status = self.status();
        if status.is_success() {
            Ok(self.json().await?)
        } else {
            let text = self.text().await?;
            Err(Error::Api(if let Ok(error) = serde_json::from_str(&text) {
                error
            } else {
                ApiError {
                    title: String::new(),
                    kind: String::new(),
                    status,
                    detail: text,
                }
            }))
        }
    }
}
