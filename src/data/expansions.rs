use super::{Media, Place, Poll, Space, Tweet, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Expansions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweets: Option<Vec<Tweet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaces: Option<Vec<Space>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<Media>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polls: Option<Vec<Poll>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub places: Option<Vec<Place>>,
}
