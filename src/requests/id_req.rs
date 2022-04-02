use serde::{Deserialize, Serialize};

macro_rules! id_req {
    ($pub:vis struct $class:ident { $field:ident }) => {
        #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
        pub struct $class {
            $field: $crate::id::NumericId,
        }

        impl<T> From<T> for $class
        where
            T: $crate::id::IntoNumericId,
        {
            fn from(id: T) -> Self {
                Self {
                    $field: id.into_id(),
                }
            }
        }
    };
}

id_req! { pub struct TweetId { tweet_id } }
id_req! { pub struct TargetUserId { target_user_id } }
id_req! { pub struct UserId { user_id } }
id_req! { pub struct ListId { list_id } }
