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
    pub id: NumericId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: ComplianceJobKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ComplianceJobStatus>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub upload_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub upload_expires_at: OffsetDateTime,
    pub download_url: Url,
    #[serde(with = "time::serde::rfc3339")]
    pub download_expires_at: OffsetDateTime,
}
