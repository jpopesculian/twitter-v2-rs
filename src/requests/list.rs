use std::marker::PhantomData;

use crate::api::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
struct DraftList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

#[derive(Debug)]
pub struct ListBuilder<A, T> {
    client: TwitterApi<A>,
    url: Url,
    method: Method,
    list: DraftList,
    return_ty: PhantomData<T>,
}

impl<A, T> ListBuilder<A, T>
where
    A: Authorization,
    T: DeserializeOwned,
{
    pub(crate) fn new(client: &TwitterApi<A>, url: Url, method: Method) -> Self {
        Self {
            client: client.clone(),
            url,
            method,
            list: Default::default(),
            return_ty: Default::default(),
        }
    }
    pub fn name(&mut self, name: impl ToString) -> &mut Self {
        self.list.name = Some(name.to_string());
        self
    }
    pub fn description(&mut self, description: impl ToString) -> &mut Self {
        self.list.description = Some(description.to_string());
        self
    }
    pub fn private(&mut self, private: bool) -> &mut Self {
        self.list.private = Some(private);
        self
    }
    pub async fn send(&self) -> ApiResult<A, T, ()> {
        self.client
            .send(
                self.client
                    .request(self.method.clone(), self.url.clone())
                    .json(&self.list),
            )
            .await
    }
}

impl<A, T> Clone for ListBuilder<A, T> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            method: self.method.clone(),
            list: self.list.clone(),
            return_ty: Default::default(),
        }
    }
}
