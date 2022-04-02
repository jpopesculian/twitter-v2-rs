# twitter-v2

Supports all of the Twitter v2 API endpoints, but many remain fairly untested
due to the complexity of the API and time restraints. PRs and Issues are very welcome!
As this repo currently has limited documentation, please check out the amazing [Twitter API v2
Docs](https://developer.twitter.com/en/docs/api-reference-index#twitter-api-v2) for information.

## Features

* **oauth2**: Included by default. See the examples for how to use.
* **native-tls**: Use `native-tls` as TLS backend (default)
* **rustls-tls**: Use `rustls` as TLS backend

## Example

```rust
use twitter_v2::TwitterApi;
use twitter_v2::authorization::{Oauth2Token, BearerToken};
use twitter_v2::query::{TweetField, UserField};

let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
let tweet = TwitterApi::new(auth)
    .get_tweet(1261326399320715264)
    .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
    .send()
    .await?
    .into_data()
    .expect("this tweet should exist");
assert_eq!(tweet.id, 1261326399320715264);
assert_eq!(tweet.author_id.unwrap(), 2244994945);
assert_eq!(tweet.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));

let auth: Oauth2Token = serde_json::from_str(&stored_oauth2_token)?;
let my_followers = TwitterApi::new(auth)
    .with_user_ctx()
    .await?
    .get_my_followers()
    .user_fields([UserField::Username])
    .max_results(20)
    .send()
    .await?
    .into_data();
```

License: MIT OR Apache-2.0
