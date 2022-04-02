use super::{Media, Place, Poll, Space, Tweet, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Expansions {
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tweets: Option<Vec<Tweet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spaces: Option<Vec<Space>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media: Option<Vec<Media>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    polls: Option<Vec<Poll>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    places: Option<Vec<Place>>,
}
