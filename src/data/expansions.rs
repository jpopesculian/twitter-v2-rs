use super::{Media, Place, Poll, Space, Tweet, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Expansions {
    users: Option<Vec<User>>,
    tweets: Option<Vec<Tweet>>,
    spaces: Option<Vec<Space>>,
    media: Option<Vec<Media>>,
    polls: Option<Vec<Poll>>,
    places: Option<Vec<Place>>,
}
