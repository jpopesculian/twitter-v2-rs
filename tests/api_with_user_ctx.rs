mod common;

use common::{get_api_app_ctx, get_api_user_ctx};
use twitter_v2::Result;

#[tokio::test]
async fn with_user_ctx() -> Result<()> {
    assert!(get_api_user_ctx().await.with_user_ctx().await.is_ok());
    assert!(get_api_app_ctx().with_user_ctx().await.is_err());
    Ok(())
}
