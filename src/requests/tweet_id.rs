use crate::id::{Id, IntoId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TweetId {
    tweet_id: Id,
}

impl<T> From<T> for TweetId
where
    T: IntoId,
{
    fn from(id: T) -> Self {
        Self {
            tweet_id: id.into_id(),
        }
    }
}
