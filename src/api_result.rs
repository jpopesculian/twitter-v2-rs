use crate::api::TwitterApi;
use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::{Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub(crate) struct InnerApiResponse<T, M> {
    data: T,
    #[serde(skip_serializing_if = "crate::utils::serde::is_null")]
    meta: M,
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

pub struct ApiResponse<A, T, M> {
    client: TwitterApi<A>,
    url: Url,
    response: InnerApiResponse<T, M>,
}

impl<A, T, M> ApiResponse<A, T, M> {
    pub(crate) fn new(client: &TwitterApi<A>, url: Url, response: InnerApiResponse<T, M>) -> Self {
        Self {
            client: client.clone(),
            url,
            response,
        }
    }
    pub fn data(&self) -> &T {
        &self.response.data
    }
    pub fn meta(&self) -> &M {
        &self.response.meta
    }
    pub fn into_data(self) -> T {
        self.response.data
    }
    pub fn into_meta(self) -> M {
        self.response.meta
    }
}

impl<A, T, M> Serialize for ApiResponse<A, T, M>
where
    T: Serialize,
    M: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.response.serialize(serializer)
    }
}

pub type ApiResult<A, T, M> = Result<ApiResponse<A, T, M>>;

#[async_trait]
pub(crate) trait ApiResponseExt {
    async fn api_json<T: DeserializeOwned, M: DeserializeOwned>(
        self,
    ) -> Result<InnerApiResponse<T, M>>;
}

#[async_trait]
impl ApiResponseExt for Response {
    async fn api_json<T: DeserializeOwned, M: DeserializeOwned>(
        self,
    ) -> Result<InnerApiResponse<T, M>> {
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
