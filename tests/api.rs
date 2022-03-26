use rand::distributions::{Alphanumeric, DistString};
use twitter_v2::{Authorization, BearerToken, TweetBuilder, TwitterApi};

fn get_api() -> TwitterApi<impl Authorization> {
    TwitterApi::new(BearerToken::new(
        std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

fn rand_str(len: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), len)
}

#[tokio::test]
async fn get_tweets() {
    let res = get_api()
        .get_tweets(&[1261326399320715264, 1278347468690915330])
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data.len(), 2);
}

#[tokio::test]
async fn get_tweet() {
    let res = get_api().get_tweet(1261326399320715264).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err())
}

async fn send_and_delete_tweet(tweet: TweetBuilder<impl Authorization>) {
    let res = tweet.send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    let id = res.unwrap().data.id;
    let res = get_api().delete_tweet(id).await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert!(res.unwrap().data.deleted)
}

#[tokio::test]
async fn manage_tweet() {
    send_and_delete_tweet(get_api().post_tweet().text(rand_str(20)).clone()).await
}

#[tokio::test]
async fn get_users() {
    let res = get_api().get_users(&[2244994945, 6253282]).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data.len(), 2);
}

#[tokio::test]
async fn get_user() {
    let res = get_api().get_user(2244994945).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_user_tweets() {
    let res = get_api().get_user_tweets(2244994945).send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_users_by() {
    let res = get_api()
        .get_users_by_usernames(&["TwitterDev", "Twitter"])
        .send()
        .await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
    assert_eq!(res.unwrap().data.len(), 2);
}

#[tokio::test]
async fn get_user_by_username() {
    let res = get_api().get_user_by_username("TwitterDev").send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}

#[tokio::test]
async fn get_users_me() {
    let res = get_api().get_users_me().send().await;
    assert!(res.is_ok(), "{}", res.unwrap_err());
}
