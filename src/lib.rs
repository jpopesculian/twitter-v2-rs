//! Supports all of the Twitter v2 API endpoints, but many remain fairly untested
//! due to the complexity of the API and time restraints. PRs and Issues are very welcome!
//! As this repo currently has limited documentation, please check out the amazing [Twitter API v2
//! Docs](https://developer.twitter.com/en/docs/api-reference-index#twitter-api-v2) for information.
//!
//! # Features
//!
//! * **oauth2**: Included by default. See the examples for how to use.
//! * **native-tls**: Use `native-tls` as TLS backend (default)
//! * **rustls-tls**: Use `rustls` as TLS backend
//!
//! # Example
//!
//! ```
//! use twitter_v2::TwitterApi;
//! use twitter_v2::authorization::{Oauth2Token, BearerToken};
//! use twitter_v2::query::{TweetField, UserField};
//! # use time::macros::datetime;
//!
//! # #[tokio::main]
//! # async fn main() -> twitter_v2::Result<()> {
//! let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
//! let tweet = TwitterApi::new(auth)
//!     .get_tweet(1261326399320715264)
//!     .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
//!     .send()
//!     .await?
//!     .into_data()
//!     .expect("this tweet should exist");
//! assert_eq!(tweet.id, 1261326399320715264);
//! assert_eq!(tweet.author_id.unwrap(), 2244994945);
//! assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));
//!
//! # let stored_oauth2_token = std::fs::read_to_string("./.oauth2_token.json").unwrap();
//! let auth: Oauth2Token = serde_json::from_str(&stored_oauth2_token)?;
//! let my_followers = TwitterApi::new(auth)
//!     .with_user_ctx()
//!     .await?
//!     .get_my_followers()
//!     .user_fields([UserField::Username])
//!     .max_results(20)
//!     .send()
//!     .await?
//!     .into_data();
//! # Ok(())
//! # }
//! ```

#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!("Either `rustls-tls` or `native-tls` feature must be selected");

#[cfg(feature = "oauth2")]
pub extern crate oauth2;

pub mod api;
pub mod api_result;
pub mod authorization;
pub mod data;
pub mod error;
pub mod id;
pub mod meta;
pub mod query;
pub mod requests;
mod utils;

pub use api::{TwitterApi, TwitterApiWithUserCtx};
pub use api_result::{ApiError, ApiPayload, ApiResponse, ApiResult};
pub use authorization::Authorization;
pub use data::{Media, Place, Poll, Space, Tweet, User};
pub use error::{Error, Result};

pub mod prelude {
    pub use crate::api_result::PaginableApiResponse;
    pub use crate::authorization::Authorization;
    pub use crate::id::{IntoNumericId, IntoStringId};
    pub use crate::meta::PaginationMeta;
}
