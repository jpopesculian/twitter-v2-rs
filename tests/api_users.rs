mod common;

use common::get_api_user_ctx;
use twitter_v2::Result;

#[tokio::test]
async fn get_users_by() -> Result<()> {
    let res = get_api_user_ctx()
        .await
        .get_users_by_usernames(&["TwitterDev", "Twitter"])
        .send()
        .await?;
    assert_eq!(res.data().unwrap().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_user_by_username() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_user_by_username("TwitterDev")
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_users_me() -> Result<()> {
    let _ = get_api_user_ctx().await.get_users_me().send().await?;
    Ok(())
}

#[tokio::test]
async fn manage_user_following() -> Result<()> {
    let twitter_dev_id = 2244994945;
    let api = get_api_user_ctx().await;
    let me = api.get_users_me().send().await?.into_data().unwrap();
    assert!(!api
        .get_user_following(me.id)
        .send()
        .await?
        .into_data()
        .unwrap()
        .into_iter()
        .any(|user| user.id == twitter_dev_id),);
    assert!(
        api.post_user_following(me.id, twitter_dev_id)
            .await?
            .into_data()
            .unwrap()
            .following
    );
    assert!(api
        .get_user_following(me.id)
        .send()
        .await?
        .into_data()
        .unwrap()
        .into_iter()
        .any(|user| user.id == twitter_dev_id),);
    assert!(
        !api.delete_user_following(me.id, twitter_dev_id)
            .await?
            .into_data()
            .unwrap()
            .following
    );
    assert!(!api
        .get_user_following(me.id)
        .send()
        .await?
        .into_data()
        .unwrap()
        .into_iter()
        .any(|user| user.id == twitter_dev_id),);
    Ok(())
}
