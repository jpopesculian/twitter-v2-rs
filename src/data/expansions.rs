use super::{Tweet, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expansions {
    users: Option<Vec<User>>,
    tweets: Option<Vec<Tweet>>,
}
