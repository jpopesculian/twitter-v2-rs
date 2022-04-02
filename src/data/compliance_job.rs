use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use url::Url;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceJobStatus {
    InProgress,
    Failed,
    Complete,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceJobKind {
    Tweets,
    Users,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ComplianceJob {
    id: NumericId,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "type")]
    kind: ComplianceJobKind,
    status: Option<ComplianceJobStatus>,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
    upload_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    upload_expires_at: OffsetDateTime,
    download_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    download_expires_at: OffsetDateTime,
}
