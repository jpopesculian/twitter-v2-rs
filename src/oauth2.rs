use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use strum_macros::Display;
use url::Url;

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Scope {
    #[strum(serialize = "tweet.read")]
    TweetRead,
    #[strum(serialize = "tweet.write")]
    TweetWrite,
    #[strum(serialize = "tweet.moderate.write")]
    TweetModerateWrite,
    #[strum(serialize = "users.read")]
    UsersRead,
    #[strum(serialize = "follows.read")]
    FollowsRead,
    #[strum(serialize = "follows.write")]
    FollowsWrite,
    #[strum(serialize = "offline.access")]
    OfflineAccess,
    #[strum(serialize = "space.read")]
    SpaceRead,
    #[strum(serialize = "mute.read")]
    MuteRead,
    #[strum(serialize = "mute.write")]
    MuteWrite,
    #[strum(serialize = "like.read")]
    LikeRead,
    #[strum(serialize = "like.write")]
    LikeWrite,
    #[strum(serialize = "list.read")]
    ListRead,
    #[strum(serialize = "list.write")]
    ListWrite,
    #[strum(serialize = "block.read")]
    BlockRead,
    #[strum(serialize = "block.write")]
    BlockWrite,
}

pub struct Oauth2User {
    oauth_client: BasicClient,
}

impl Oauth2User {
    pub fn new(client_id: impl ToString, client_secret: impl ToString, callback_url: Url) -> Self {
        let oauth_client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::from_url("https://twitter.com/i/oauth2/authorize".parse().unwrap()),
            Some(TokenUrl::from_url(
                "https://api.twitter.com/2/oauth2/token".parse().unwrap(),
            )),
        )
        .set_redirect_uri(RedirectUrl::from_url(callback_url));
        Self { oauth_client }
    }
}
