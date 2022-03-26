use rand::distributions::{Alphanumeric, DistString};
use twitter_v2::{BearerToken, PaginableApiResponse, Result, TweetBuilder, TwitterApi};

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
async fn get_tweets() -> Result<()> {
    let res = get_api_user_ctx()
        .get_tweets(&[1261326399320715264, 1278347468690915330])
        .send()
        .await?;
    assert_eq!(res.data().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_tweet() -> Result<()> {
    let _ = get_api_user_ctx()
        .get_tweet(1261326399320715264)
        .send()
        .await?;
    Ok(())
}

async fn send_and_delete_tweet(tweet: TweetBuilder<BearerToken>) -> Result<()> {
    let res = tweet.send().await?;
    let id = res.data().id;
    let res = get_api_user_ctx().delete_tweet(id).await?;
    assert!(res.data().deleted);
    Ok(())
}

#[tokio::test]
async fn manage_tweet() -> Result<()> {
    send_and_delete_tweet(get_api_user_ctx().post_tweet().text(rand_str(20)).clone()).await?;
    Ok(())
}

#[tokio::test]
async fn get_users() -> Result<()> {
    let res = get_api_user_ctx()
        .get_users(&[2244994945, 6253282])
        .send()
        .await?;
    assert_eq!(res.data().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_user() -> Result<()> {
    let _ = get_api_user_ctx().get_user(2244994945).send().await?;
    Ok(())
}

#[tokio::test]
async fn get_user_tweets_paginated() -> Result<()> {
    let api = get_api_user_ctx();
    let page1 = api
        .get_user_tweets(2244994945)
        .max_results(10)
        .send()
        .await?;

    let page2 = page1.next_page().await?.unwrap();
    assert_ne!(page2.meta().oldest_id, page1.meta().oldest_id);

    let page3 = page2.next_page().await?.unwrap();
    assert_ne!(page3.meta().oldest_id, page2.meta().oldest_id);
    assert_ne!(page3.meta().oldest_id, page1.meta().oldest_id);

    let page2_again = page3.previous_page().await?.unwrap();
    assert_eq!(page2_again.meta().oldest_id, page2.meta().oldest_id);
    Ok(())
}

#[tokio::test]
async fn get_tweets_search_recent() -> Result<()> {
    let _ = get_api_user_ctx()
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
async fn get_tweets_search_stream_rule() -> Result<()> {
    let _ = get_api_app_ctx()
        .get_tweets_search_stream_rules()
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn post_tweets_search_stream_rule() -> Result<()> {
    let res = get_api_app_ctx()
        .post_tweets_search_stream_rule()
        .add("from:TwitterDev -is:retweet")
        .dry_run()
        .send()
        .await?;
    assert_eq!(res.meta().summary.valid, Some(1));
    Ok(())
}

#[tokio::test]
async fn get_users_by() -> Result<()> {
    let res = get_api_user_ctx()
        .get_users_by_usernames(&["TwitterDev", "Twitter"])
        .send()
        .await?;
    assert_eq!(res.data().len(), 2);
    Ok(())
}

#[tokio::test]
async fn get_user_by_username() -> Result<()> {
    let _ = get_api_user_ctx()
        .get_user_by_username("TwitterDev")
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn get_users_me() -> Result<()> {
    let _ = get_api_user_ctx().get_users_me().send().await?;
    Ok(())
}
