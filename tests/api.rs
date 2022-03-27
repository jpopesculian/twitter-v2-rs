use rand::distributions::{Alphanumeric, DistString};
use std::ops::Deref;
use std::sync::Mutex;
use twitter_v2::{
    BearerToken, Oauth2Client, Oauth2Token, PaginableApiResponse, Result, TwitterApi,
};

lazy_static::lazy_static! {
    static ref OAUTH2_TOKEN: Mutex<Oauth2Token> = Mutex::new(serde_json::from_reader(
        std::fs::File::open("./.oauth2_token.json").expect(".oauth2_token.json not found"),
    )
    .expect(".oauth2_token.json not valid json"));
}
async fn get_token() -> Oauth2Token {
    let mut token = OAUTH2_TOKEN.lock().unwrap();
    if token.is_expired() {
        let oauth2_client = Oauth2Client::new(
            std::env::var("CLIENT_ID").expect("could not find CLIENT_ID"),
            std::env::var("CLIENT_SECRET").expect("could not find CLIENT_SECRET"),
            "http://localhost:3000/callback".parse().unwrap(),
        );
        *token = oauth2_client
            .refresh_token(token.refresh_token().unwrap())
            .await
            .unwrap()
            .try_into()
            .unwrap();
        serde_json::to_writer(
            std::fs::File::open("./.oauth2_token.json").expect(".oauth2_token.json not found"),
            token.deref(),
        )
        .expect("couldn't save token");
    }
    token.clone()
}
async fn get_api_user_ctx() -> TwitterApi<Oauth2Token> {
    TwitterApi::new(get_token().await)
}
fn get_api_app_ctx() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("APP_BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

fn rand_str(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

#[tokio::test]
async fn get_tweets() -> Result<()> {
    let res = get_api_user_ctx()
        .await
        .get_tweets(&[1261326399320715264, 1278347468690915330])
        .send()
        .await?;
    assert_eq!(res.data().unwrap().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_tweet() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_tweet(1261326399320715264)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn manage_tweet() -> Result<()> {
    let api = get_api_user_ctx().await;
    let res = api.post_tweet().text(rand_str(20)).send().await?;
    let id = res.data().unwrap().id;
    let res = api.delete_tweet(id).await?;
    assert!(res.data().unwrap().deleted);
    Ok(())
}

#[tokio::test]
async fn get_users() -> Result<()> {
    let res = get_api_user_ctx()
        .await
        .get_users(&[2244994945, 6253282])
        .send()
        .await?;
    assert_eq!(res.data().unwrap().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_user() -> Result<()> {
    let _ = get_api_user_ctx().await.get_user(2244994945).send().await?;
    Ok(())
}

#[tokio::test]
async fn get_user_tweets_paginated() -> Result<()> {
    let api = get_api_user_ctx().await;
    let page1 = api
        .get_user_tweets(2244994945)
        .max_results(10)
        .send()
        .await?;

    let page2 = page1.next_page().await?.unwrap();
    assert_ne!(
        page2.meta().unwrap().oldest_id,
        page1.meta().unwrap().oldest_id
    );

    let page3 = page2.next_page().await?.unwrap();
    assert_ne!(
        page3.meta().unwrap().oldest_id,
        page2.meta().unwrap().oldest_id
    );
    assert_ne!(
        page3.meta().unwrap().oldest_id,
        page1.meta().unwrap().oldest_id
    );

    let page2_again = page3.previous_page().await?.unwrap();
    assert_eq!(
        page2_again.meta().unwrap().oldest_id,
        page2.meta().unwrap().oldest_id
    );
    Ok(())
}

#[tokio::test]
async fn get_tweets_search_recent() -> Result<()> {
    let _ = get_api_user_ctx()
        .await
        .get_tweets_search_recent("from:TwitterDev -is:retweet")
        .max_results(10)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_tweets_counts_recent() -> Result<()> {
    let _ = get_api_app_ctx()
        .get_tweets_counts_recent("from:TwitterDev -is:retweet")
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn manage_tweets_search_stream_rule() -> Result<()> {
    let res = get_api_app_ctx()
        .post_tweets_search_stream_rule()
        .add("from:TwitterDev -is:retweet")
        .send()
        .await?;
    let res = get_api_app_ctx()
        .get_tweets_search_stream_rules()
        .send()
        .await?;
    let res = get_api_app_ctx()
        .post_tweets_search_stream_rule()
        .delete_id(1508010426864476163)
        .send()
        .await?;
    Ok(())
}

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
