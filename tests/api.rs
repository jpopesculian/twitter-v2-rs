use twitter_v2::{BearerToken, DraftTweet, TwitterApi};

fn get_api() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}

#[tokio::test]
async fn get_tweets() {
    let res = get_api()
        .get_tweets(&[1261326399320715264, 1278347468690915330], None, None)
        .await;
    assert!(res.is_ok(), "{res:?}");
    assert_eq!(res.unwrap().data.len(), 2);
}

#[tokio::test]
async fn get_tweet() {
    let res = get_api().get_tweet(1261326399320715264, None, None).await;
    assert!(res.is_ok(), "{res:?}")
}

#[tokio::test]
async fn post_tweet() {
    let res = get_api()
        .post_tweet(&DraftTweet {
            text: Some("Hello, world!".to_string()),
            ..Default::default()
        })
        .await;
    assert!(res.is_ok(), "{res:?}");
}

#[tokio::test]
async fn get_users_me() {
    let res = get_api().get_users_me().await;
    assert!(res.is_ok(), "{res:?}");
}
