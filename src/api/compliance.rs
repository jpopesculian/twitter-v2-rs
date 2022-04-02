use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{ComplianceJob, ComplianceJobKind};
use crate::id::IntoNumericId;
use crate::query::{ComplianceJobKindQuery, GetComplianceJobsRequestBuilder, UrlQueryExt};
use crate::requests::ComplianceJobBuilder;
use reqwest::Method;

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_compliance_jobs(
        &self,
        kind: impl IntoIterator<Item = ComplianceJobKindQuery>,
    ) -> GetComplianceJobsRequestBuilder<A, Vec<ComplianceJob>, ()> {
        let mut url = self.url("compliance/jobs").unwrap();
        url.append_query_seq("type", kind);
        GetComplianceJobsRequestBuilder::new(self, url)
    }
    pub async fn get_compliance_job(
        &self,
        id: impl IntoNumericId,
    ) -> ApiResult<A, ComplianceJob, ()> {
        self.send(self.request(Method::GET, self.url(format!("compliance/job/{id}"))?))
            .await
    }
    pub fn post_compliance_job(&self, kind: ComplianceJobKind) -> ComplianceJobBuilder<A> {
        ComplianceJobBuilder::new(self, self.url("compliance/jobs").unwrap(), kind)
    }
}
