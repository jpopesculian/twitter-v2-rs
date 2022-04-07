use crate::id::NumericId;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct PollOption {
    pub position: usize,
    pub label: String,
    pub votes: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PollVotingStatus {
    Open,
    Closed,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Poll {
    pub id: NumericId,
    pub options: Vec<PollOption>,
    #[serde(
        with = "crate::utils::serde::option_duration_mins",
        rename = "duration_minutes",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration: Option<Duration>,
    #[serde(
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_datetime: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voting_status: Option<PollVotingStatus>,
}
