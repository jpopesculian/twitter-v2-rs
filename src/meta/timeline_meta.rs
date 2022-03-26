use super::pagination::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimelineMeta {
    pub result_count: usize,
    pub newest_id: String,
    pub oldest_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,
}

impl PaginationMeta for TimelineMeta {
    fn next_token(&self) -> Option<&str> {
        self.next_token.as_deref()
    }
    fn previous_token(&self) -> Option<&str> {
        self.previous_token.as_deref()
    }
}
