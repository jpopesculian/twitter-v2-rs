use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TweetsCountsMeta {
    total_tweet_count: usize,
}
