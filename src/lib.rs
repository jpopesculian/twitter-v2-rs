#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!("Either `rustls-tls` or `native-tls` feature must be selected");

#[cfg(feature = "oauth2")]
pub extern crate oauth2;

mod api;
mod api_result;
mod authorization;
mod data;
mod error;
mod expansions;
mod fields;
mod id;
mod query;
mod requests;
mod utils;

pub use api::*;
pub use api_result::*;
pub use authorization::*;
pub use data::*;
pub use error::*;
pub use expansions::*;
pub use fields::*;
pub use requests::*;
