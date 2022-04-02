use crate::api::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{ComplianceJob, ComplianceJobKind};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct DraftComplianceJob {
    #[serde(rename = "type")]
    kind: ComplianceJobKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resumable: Option<bool>,
}

#[derive(Debug)]
pub struct ComplianceJobBuilder<A> {
    client: TwitterApi<A>,
    url: Url,
    job: DraftComplianceJob,
}

impl<A> ComplianceJobBuilder<A>
where
    A: Authorization,
{
    pub(crate) fn new(client: &TwitterApi<A>, url: Url, kind: ComplianceJobKind) -> Self {
        Self {
            client: client.clone(),
            url,
            job: DraftComplianceJob {
                kind,
                name: None,
                resumable: None,
            },
        }
    }
    pub fn text(&mut self, name: impl ToString) -> &mut Self {
        self.job.name = Some(name.to_string());
        self
    }
    pub fn resumable(&mut self, resumable: bool) -> &mut Self {
        self.job.resumable = Some(resumable);
        self
    }
    pub async fn send(&self) -> ApiResult<A, ComplianceJob, ()> {
        self.client
            .send(
                self.client
                    .request(Method::POST, self.url.clone())
                    .json(&self.job),
            )
            .await
    }
}

impl<A> Clone for ComplianceJobBuilder<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            url: self.url.clone(),
            job: self.job.clone(),
        }
    }
}
