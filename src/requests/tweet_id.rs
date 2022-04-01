use crate::id::{IntoNumericId, NumericId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TweetId {
    tweet_id: NumericId,
}

impl<T> From<T> for TweetId
where
    T: IntoNumericId,
{
    fn from(id: T) -> Self {
        Self {
            tweet_id: id.into_id(),
        }
    }
}
