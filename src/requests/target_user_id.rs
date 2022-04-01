use crate::id::{Id, IntoId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TargetUserId {
    target_user_id: Id,
}

impl<T> From<T> for TargetUserId
where
    T: IntoId,
{
    fn from(id: T) -> Self {
        Self {
            target_user_id: id.into_id(),
        }
    }
}
