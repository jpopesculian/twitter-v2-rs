mod common;

use common::get_api_user_ctx;
use twitter_v2::{query::ListField, Result};

#[tokio::test]
async fn get_list() -> Result<()> {
    let _ = get_api_user_ctx().await.get_list(84839422).send().await?;
    Ok(())
}

#[tokio::test]
async fn get_user_owned_lists() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_user_owned_lists(2244994945)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn manage_lists() -> Result<()> {
    const LIST_NAME: &str = "Test List";
    const LIST_DESC: &str = "Test List description";
    let api = get_api_user_ctx().await;
    let id = api
        .post_list(LIST_NAME)
        .send()
        .await?
        .into_data()
        .unwrap()
        .id;
    let list = api
        .get_list(id)
        .list_fields([ListField::Description])
        .send()
        .await?
        .into_data()
        .unwrap();
    assert_eq!(list.id, id);
    assert_eq!(list.name, LIST_NAME);
    assert_eq!(list.description.as_deref(), Some(""));
    assert!(
        api.put_list(id)
            .description(LIST_DESC)
            .send()
            .await?
            .into_data()
            .unwrap()
            .updated
    );
    let list = api
        .get_list(id)
        .list_fields([ListField::Description])
        .send()
        .await?
        .into_data()
        .unwrap();
    assert_eq!(list.id, id);
    assert_eq!(list.name, LIST_NAME);
    assert_eq!(list.description.as_deref(), Some(LIST_DESC));
    assert!(api.delete_list(id).await?.into_data().unwrap().deleted);
    Ok(())
}

#[tokio::test]
async fn get_list_tweets() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_list_tweets(84839422)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_list_members() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_list_members(84839422)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_list_followers() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_list_followers(84839422)
        .send()
        .await?;
    Ok(())
}
