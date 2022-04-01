use crate::id::{IntoNumericId, NumericId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct TargetUserId {
    target_user_id: NumericId,
}

impl<T> From<T> for TargetUserId
where
    T: IntoNumericId,
{
    fn from(id: T) -> Self {
        Self {
            target_user_id: id.into_id(),
        }
    }
}
