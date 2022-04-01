use std::ops::Deref;
use std::sync::Mutex;
use twitter_v2::authorization::{BearerToken, Oauth2Client, Oauth2Token};
use twitter_v2::TwitterApi;

lazy_static::lazy_static! {
    static ref OAUTH2_TOKEN: Mutex<Oauth2Token> = Mutex::new(serde_json::from_reader(
        std::fs::File::open("./.oauth2_token.json").expect(".oauth2_token.json not found"),
    )
    .expect(".oauth2_token.json not valid json"));
}
async fn get_token() -> Oauth2Token {
    let oauth2_client = Oauth2Client::new(
        std::env::var("CLIENT_ID").expect("could not find CLIENT_ID"),
        std::env::var("CLIENT_SECRET").expect("could not find CLIENT_SECRET"),
        "http://localhost:3000/callback".parse().unwrap(),
    );
    let mut token = OAUTH2_TOKEN.lock().unwrap();
    if oauth2_client
        .refresh_token_if_expired(&mut token)
        .await
        .unwrap()
    {
        serde_json::to_writer(
            std::fs::File::create("./.oauth2_token.json").expect(".oauth2_token.json not found"),
            token.deref(),
        )
        .expect("couldn't save token");
    }
    token.clone()
}
#[allow(dead_code)]
pub async fn get_api_user_ctx() -> TwitterApi<Oauth2Token> {
    TwitterApi::new(get_token().await)
}
#[allow(dead_code)]
pub fn get_api_app_ctx() -> TwitterApi<BearerToken> {
    TwitterApi::new(BearerToken::new(
        std::env::var("APP_BEARER_TOKEN").expect("BEARER_TOKEN not found"),
    ))
}
