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
pub use data::{Tweet, User};
pub use error::{Error, Result};
pub use id::{Id, IntoId};

pub mod prelude {
    pub use crate::api_result::PaginableApiResponse;
    pub use crate::authorization::Authorization;
    pub use crate::meta::PaginationMeta;
    pub use crate::IntoId;
}
