use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimelineMeta {
    result_count: usize,
    newest_id: String,
    oldest_id: String,
    next_token: Option<String>,
    previous_token: Option<String>,
}
