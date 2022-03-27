use super::pagination::PaginationMeta;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResultCountMeta {
    pub result_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_token: Option<String>,
}

impl PaginationMeta for ResultCountMeta {
    fn next_token(&self) -> Option<&str> {
        self.next_token.as_deref()
    }
    fn previous_token(&self) -> Option<&str> {
        self.previous_token.as_deref()
    }
}
