mod common;

use common::get_api_user_ctx;
use twitter_v2::Result;

#[tokio::test]
async fn get_space() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_space("1DXxyRYNejbKM")
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_spaces() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_spaces(["1DXxyRYNejbKM", "1nAJELYEEPvGL"])
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_spaces_by_creator_ids() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_spaces_by_creator_ids([2244994945, 6253282])
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_spaces_search() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_spaces_search("hello")
        .send()
        .await?;
    Ok(())
}
