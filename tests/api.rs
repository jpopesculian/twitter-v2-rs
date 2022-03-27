use futures::prelude::*;
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
            std::fs::File::create("./.oauth2_token.json").expect(".oauth2_token.json not found"),
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
    let api = get_api_app_ctx();
    let res = api
        .post_tweets_search_stream_rule()
        .add("from:TwitterDev -is:retweet")
        .send()
        .await?;
    assert_eq!(res.meta().unwrap().summary.valid, Some(1));
    assert_eq!(res.meta().unwrap().summary.created, Some(1));
    let id = res.into_data().unwrap().pop().unwrap().id;

    let res = api.get_tweets_search_stream_rules().send().await?;
    assert_eq!(res.into_data().unwrap().pop().unwrap().id, id);

    let res = api
        .post_tweets_search_stream_rule()
        .delete_id(id)
        .send()
        .await?;
    assert_eq!(res.meta().unwrap().summary.deleted, Some(1));

    Ok(())
}

#[tokio::test]
async fn get_tweets_sample_stream() -> Result<()> {
    let res = get_api_app_ctx()
        .get_tweets_sample_stream()
        .stream()
        .await?
        .try_next()
        .await?;
    assert!(res.is_some());
    Ok(())
}

#[tokio::test]
async fn get_tweet_retweeted_by() -> Result<()> {
    let _ = get_api_app_ctx()
        .get_tweet_retweeted_by(1354143047324299264)
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn post_user_retweet() -> Result<()> {
    let api = get_api_user_ctx().await;
    let me = api.get_users_me().send().await?.into_data().unwrap();
    let res = api.post_user_retweet(me.id, 1228393702244134912).await?;
    assert!(res.data().unwrap().retweeted);
    let res = api.delete_user_retweet(me.id, 1228393702244134912).await?;
    assert!(!res.data().unwrap().retweeted);
    Ok(())
}

#[tokio::test]
async fn get_tweet_quote_tweets() -> Result<()> {
    let _ = get_api_app_ctx()
        .get_tweet_quote_tweets(1354143047324299264)
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
