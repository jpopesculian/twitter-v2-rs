# twitter-v2

Currently a work in progress! Supports many of the tweet related endpoints,
with more endpoints being added soon

## Features

* `oauth2`: Included by default. See the examples for how to use.

## Example

```rust
use twitter_v2::{TwitterApi, authorization::BearerToken, query::TweetField};

let auth = BearerToken::new(std::env::var("APP_BEARER_TOKEN").unwrap());
let res = TwitterApi::new(auth)
    .get_tweet(1261326399320715264)
    .tweet_fields([TweetField::AuthorId, TweetField::CreatedAt])
    .send()
    .await?
    .into_data()
    .unwrap();
assert_eq!(res.id, 1261326399320715264);
assert_eq!(res.author_id.unwrap(), 2244994945);
assert_eq!(res.created_at.unwrap(), datetime!(2020-05-15 16:03:42 UTC));
```

License: MIT OR Apache-2.0
