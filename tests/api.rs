use twitter_v2::{Authentication, DraftTweet, TwitterApi};

fn get_api() -> TwitterApi {
    TwitterApi::new(Authentication::Oauth1a {
        consumer_key: std::env::var("CONSUMER_KEY").unwrap(),
        consumer_secret: std::env::var("CONSUMER_SECRET").unwrap(),
        token: std::env::var("TOKEN").unwrap(),
        secret: std::env::var("SECRET").unwrap(),
    })
}

#[tokio::test]
async fn get_tweets() {
    let res = get_api()
        .get_tweets(&[1261326399320715264, 1278347468690915330], None, None)
        .await;
    println!("{:#?}", res);
}

#[tokio::test]
async fn get_tweet() {
    let res = get_api().get_tweet(1261326399320715264, None, None).await;
    println!("{:#?}", res);
}

#[tokio::test]
async fn post_tweet() {
    let res = get_api()
        .post_tweet(&DraftTweet {
            text: Some("Hello, world!".to_string()),
            ..Default::default()
        })
        .await;
    println!("{:#?}", res);
}

#[tokio::test]
async fn get_users_me() {
    let res = get_api().get_users_me().await;
    println!("{:#?}", res);
}
