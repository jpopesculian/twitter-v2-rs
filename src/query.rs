pub(crate) trait ToQuery {
    fn to_query(self, query: &str) -> Vec<(&str, String)>;
}

impl<T> ToQuery for T
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn to_query(self, key: &str) -> Vec<(&str, String)> {
        let value = self
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        if value.is_empty() {
            vec![]
        } else {
            vec![(key, value)]
        }
    }
}

macro_rules! get_req_builder_arg {
    (media_fields) => {
        pub fn media_fields(
            mut self,
            fields: impl IntoIterator<Item = $crate::MediaField>,
        ) -> Self {
            self.req = self.req.query(&fields.to_query("media.fields"));
            self
        }
    };
    (place_fields) => {
        pub fn place_fields(
            mut self,
            fields: impl IntoIterator<Item = $crate::PlaceField>,
        ) -> Self {
            self.req = self.req.query(&fields.to_query("place.fields"));
            self
        }
    };
    (poll_fields) => {
        pub fn poll_fields(mut self, fields: impl IntoIterator<Item = $crate::PollField>) -> Self {
            self.req = self.req.query(&fields.to_query("poll.fields"));
            self
        }
    };
    (user_fields) => {
        pub fn user_fields(mut self, fields: impl IntoIterator<Item = $crate::UserField>) -> Self {
            self.req = self.req.query(&fields.to_query("user.fields"));
            self
        }
    };
    (tweet_fields) => {
        pub fn tweet_fields(
            mut self,
            fields: impl IntoIterator<Item = $crate::TweetField>,
        ) -> Self {
            self.req = self.req.query(&fields.to_query("tweet.fields"));
            self
        }
    };
    (tweet_expansions) => {
        pub fn expansions(
            mut self,
            expansions: impl IntoIterator<Item = $crate::TweetExpansion>,
        ) -> Self {
            self.req = self.req.query(&expansions.to_query("expansions"));
            self
        }
    };
    (user_expansions) => {
        pub fn expansions(
            mut self,
            expansions: impl IntoIterator<Item = $crate::UserExpansion>,
        ) -> Self {
            self.req = self.req.query(&expansions.to_query("expansions"));
            self
        }
    };
}

macro_rules! get_req_builder {
    ($vis:vis struct $class:ident { $($optional_arg:tt),* }) => {
        $vis struct $class<A, T> {
            client: $crate::TwitterApi<A>,
            req: reqwest::RequestBuilder,
            return_ty: std::marker::PhantomData<T>
        }

        impl<A, T> $class<A, T>
        where
            A: $crate::Authorization,
            T: serde::de::DeserializeOwned
        {
            fn new(client: &$crate::TwitterApi<A>, req: reqwest::RequestBuilder) -> Self {
                Self { client: client.clone(), req, return_ty: Default::default() }
            }
            $($crate::query::get_req_builder_arg! { $optional_arg })*
            pub async fn send(self) -> $crate::ApiResult<T> {
                self.client.send(self.req).await
            }
        }
    };
}

pub(crate) use get_req_builder;
pub(crate) use get_req_builder_arg;
