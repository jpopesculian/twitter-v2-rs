use crate::api::TwitterApi;
use crate::authorization::Authorization;
use crate::data::Expansions;
use crate::error::{Error, Result};
use crate::meta::PaginationMeta;
use crate::query::UrlQueryExt;
use async_trait::async_trait;
use reqwest::{Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, ops};
use url::Url;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiPayload<T, M> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<M>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<Expansions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ApiError>>,
}

impl<T, M> ApiPayload<T, M> {
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }
    pub fn meta(&self) -> Option<&M> {
        self.meta.as_ref()
    }
    pub fn includes(&self) -> Option<&[Expansions]> {
        self.includes.as_deref()
    }
    pub fn errors(&self) -> Option<&[ApiError]> {
        self.errors.as_deref()
    }
    pub fn into_data(self) -> Option<T> {
        self.data
    }
    pub fn into_meta(self) -> Option<M> {
        self.meta
    }
    pub fn into_includes(self) -> Option<Vec<Expansions>> {
        self.includes
    }
    pub fn into_errors(self) -> Option<Vec<ApiError>> {
        self.errors
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiErrorItem {
    parameters: HashMap<String, Vec<serde_json::Value>>,
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ApiError {
    pub title: String,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default, with = "crate::utils::serde::status_code")]
    pub status: StatusCode,
    #[serde(default)]
    pub detail: String,
    #[serde(default)]
    pub errors: Vec<ApiErrorItem>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{}] {}", self.status, self.detail))
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug)]
pub struct ApiResponse<A, T, M> {
    client: TwitterApi<A>,
    url: Url,
    payload: ApiPayload<T, M>,
}

impl<A, T, M> ApiResponse<A, T, M> {
    pub(crate) fn new(client: &TwitterApi<A>, url: Url, payload: ApiPayload<T, M>) -> Self {
        Self {
            client: client.clone(),
            url,
            payload,
        }
    }
    pub fn url(&self) -> &Url {
        &self.url
    }
    pub fn payload(&self) -> &ApiPayload<T, M> {
        &self.payload
    }
    pub fn into_payload(self) -> ApiPayload<T, M> {
        self.payload
    }
    pub fn into_data(self) -> Option<T> {
        self.payload.data
    }
    pub fn into_meta(self) -> Option<M> {
        self.payload.meta
    }
    pub fn into_includes(self) -> Option<Vec<Expansions>> {
        self.payload.includes
    }
    pub fn into_errors(self) -> Option<Vec<ApiError>> {
        self.payload.errors
    }
}

impl<A, T, M> ops::Deref for ApiResponse<A, T, M> {
    type Target = ApiPayload<T, M>;
    fn deref(&self) -> &Self::Target {
        &self.payload
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
        self.payload.serialize(serializer)
    }
}

impl<A, T, M> Clone for ApiResponse<A, T, M>
where
    T: Clone,
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            payload: self.payload.clone(),
        }
    }
}

#[async_trait]
pub trait PaginableApiResponse: Sized {
    async fn next_page(&self) -> Result<Option<Self>>;
    async fn previous_page(&self) -> Result<Option<Self>>;
}

#[async_trait]
impl<A, T, M> PaginableApiResponse for ApiResponse<A, T, M>
where
    A: Authorization + Send + Sync,
    T: DeserializeOwned + Send + Sync,
    M: PaginationMeta + DeserializeOwned + Send + Sync,
{
    async fn next_page(&self) -> Result<Option<Self>> {
        if let Some(token) = self.meta().and_then(|m| m.next_token()) {
            let mut url = self.url.clone();
            url.replace_query_val("pagination_token", token);
            Ok(Some(
                self.client
                    .send(self.client.request(Method::GET, url))
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }
    async fn previous_page(&self) -> Result<Option<Self>> {
        if let Some(token) = self.meta().and_then(|m| m.previous_token()) {
            let mut url = self.url.clone();
            url.replace_query_val("pagination_token", token);
            Ok(Some(
                self.client
                    .send(self.client.request(Method::GET, url))
                    .await?,
            ))
        } else {
            Ok(None)
        }
    }
}

pub type ApiResult<A, T, M> = Result<ApiResponse<A, T, M>>;

#[async_trait]
pub(crate) trait ApiResponseExt: Sized {
    async fn api_error_for_status(self) -> Result<Self>;
}

#[async_trait]
impl ApiResponseExt for Response {
    async fn api_error_for_status(self) -> Result<Self> {
        let status = self.status();
        if status.is_success() {
            Ok(self)
        } else {
            let text = self.text().await?;
            Err(Error::Api(
                if let Ok(mut error) = serde_json::from_str::<ApiError>(&text) {
                    error.status = status;
                    error
                } else {
                    ApiError {
                        status,
                        detail: text,
                        ..Default::default()
                    }
                },
            ))
        }
    }
}
