use rand::distributions::{Alphanumeric, DistString};
use twitter_v2::{BearerToken, PaginableApiResponse, TweetBuilder, TwitterApi};

fn get_api_user_ctx() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("USER_BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
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
async fn get_tweets() {
    let res = get_api_user_ctx()
        .get_tweets(&[1261326399320715264, 1278347468690915330])
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data().len(), 2);
}

#[tokio::test]
async fn get_tweet() {
    let res = get_api_user_ctx()
        .get_tweet(1261326399320715264)
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err())
}

async fn send_and_delete_tweet(tweet: TweetBuilder<BearerToken>) {
    let res = tweet.send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let id = res.unwrap().data().id;
    let res = get_api_user_ctx().delete_tweet(id).await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert!(res.unwrap().data().deleted)
}

#[tokio::test]
async fn manage_tweet() {
    send_and_delete_tweet(get_api_user_ctx().post_tweet().text(rand_str(20)).clone()).await
}

#[tokio::test]
async fn get_users() {
    let res = get_api_user_ctx()
        .get_users(&[2244994945, 6253282])
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data().len(), 2);
}

#[tokio::test]
async fn get_user() {
    let res = get_api_user_ctx().get_user(2244994945).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_user_tweets_paginated() {
    let api = get_api_user_ctx();
    let res = api.get_user_tweets(2244994945).max_results(10).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let page1 = res.unwrap();

    let res = page1.next_page().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = res.unwrap();
    assert!(res.is_some());
    let page2 = res.unwrap();
    assert_ne!(page2.meta().oldest_id, page1.meta().oldest_id);

    let res = page2.next_page().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = res.unwrap();
    assert!(res.is_some());
    let page3 = res.unwrap();
    assert_ne!(page3.meta().oldest_id, page2.meta().oldest_id);
    assert_ne!(page3.meta().oldest_id, page1.meta().oldest_id);

    let res = page3.previous_page().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let res = res.unwrap();
    assert!(res.is_some());
    let page2_again = res.unwrap();
    assert_eq!(page2_again.meta().oldest_id, page2.meta().oldest_id);
}

#[tokio::test]
async fn get_tweets_search_recent() {
    let api = get_api_user_ctx();
    let res = api
        .get_tweets_search_recent("from:TwitterDev -is:retweet")
        .max_results(10)
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_tweets_counts_recent() {
    let api = get_api_app_ctx();
    let res = api
        .get_tweets_counts_recent("from:TwitterDev -is:retweet")
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_users_by() {
    let res = get_api_user_ctx()
        .get_users_by_usernames(&["TwitterDev", "Twitter"])
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data().len(), 2);
}

#[tokio::test]
async fn get_user_by_username() {
    let res = get_api_user_ctx()
        .get_user_by_username("TwitterDev")
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_users_me() {
    let res = get_api_user_ctx().get_users_me().send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}
