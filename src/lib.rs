//! Currently a work in progress! Supports many of the tweet related endpoints,
//! with more endpoints being added soon
//!
//! # Features
//!
//! * **oauth2**: Included by default. See the examples for how to use.
//! * **rustls-tls**: Use `rustls` as TLS backend
//! * **native-tls**: Use `native-tls` as TLS backend
//!
//! # Example
//!
//! ```
//! use twitter_v2::{TwitterApi, authorization::BearerToken, query::TweetField};
//! # use time::macros::datetime;
//!
//! # #[tokio::main]
//! # async fn main() -> twitter_v2::Result<()> {
//! let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
//! let res = TwitterApi::new(auth)
//!     .get_tweet(1261326399320715264)
//!     .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
//!     .send()
//!     .await?
//!     .into_data()
//!     .unwrap();
//! assert_eq!(res.id, 1261326399320715264);
//! assert_eq!(res.author_id.unwrap(), 2244994945);
//! assert_eq!(res.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));
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

pub use api::TwitterApi;
pub use api_result::{ApiError, ApiPayload, ApiResponse, ApiResult};
pub use data::{Space, Tweet, User};
pub use error::{Error, Result};

pub mod prelude {
    pub use crate::api_result::PaginableApiResponse;
    pub use crate::authorization::Authorization;
    pub use crate::id::IntoNumericId;
    pub use crate::meta::PaginationMeta;
}
