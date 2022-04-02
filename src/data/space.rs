use crate::id::{NumericId, StringId};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SpaceState {
    Live,
    Scheduled,
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SpaceTopic {
    pub id: StringId,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Space {
    pub id: StringId,
    pub state: SpaceState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ticketed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ids: Option<Vec<NumericId>>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<NumericId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_ids: Option<Vec<NumericId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_user_ids: Option<Vec<NumericId>>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_start: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub started_at: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ended_at: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_ids: Option<Vec<StringId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<SpaceTopic>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<usize>,
}
