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
    (exclude) => {
        pub fn exclude(mut self, exclude: impl IntoIterator<Item = $crate::Exclude>) -> Self {
            self.req = self.req.query(&exclude.to_query("exclude"));
            self
        }
    };
    (max_results) => {
        pub fn max_results(mut self, max_results: usize) -> Self {
            self.req = self.req.query(&(("max_results", max_results)));
            self
        }
    };
    (pagination_token) => {
        pub fn pagination_token(mut self, pagination_token: &str) -> Self {
            self.req = self.req.query(&(("pagination_token", pagination_token)));
            self
        }
    };
    (since_id) => {
        pub fn since_id(mut self, since_id: impl $crate::IntoId) -> Self {
            self.req = self.req.query(&(("since_id", since_id.to_string())));
            self
        }
    };
    (until_id) => {
        pub fn until_id(mut self, until_id: impl $crate::IntoId) -> Self {
            self.req = self.req.query(&(("until_id", until_id.to_string())));
            self
        }
    };
    (start_time) => {
        pub fn start_time(mut self, start_time: time::OffsetDateTime) -> Self {
            self.req = self.req.query(
                &((
                    "start_time",
                    start_time
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap(),
                )),
            );
            self
        }
    };
    (end_time) => {
        pub fn end_time(mut self, end_time: time::OffsetDateTime) -> Self {
            self.req = self.req.query(
                &((
                    "end_time",
                    end_time
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap(),
                )),
            );
            self
        }
    };
}

macro_rules! get_req_builder {
    ($vis:vis struct $class:ident { $($optional_arg:tt),* }) => {
        $vis struct $class<A, T, M> {
            client: $crate::TwitterApi<A>,
            req: reqwest::RequestBuilder,
            return_ty: std::marker::PhantomData<(T, M)>
        }

        impl<A, T, M> $class<A, T, M>
        where
            A: $crate::Authorization,
            T: serde::de::DeserializeOwned,
            M: serde::de::DeserializeOwned
        {
            fn new(client: &$crate::TwitterApi<A>, req: reqwest::RequestBuilder) -> Self {
                Self { client: client.clone(), req, return_ty: Default::default() }
            }
            $($crate::query::get_req_builder_arg! { $optional_arg })*
            pub async fn send(self) -> $crate::ApiResult<T, M> {
                self.client.send(self.req).await
            }
        }

        impl<A, T, M> Clone for $class<A, T, M> {
            fn clone(&self) -> Self {
                Self {
                    client: self.client.clone(),
                    req: self.req.try_clone().unwrap(),
                    return_ty: Default::default()
                }
            }
        }
    };
}

pub(crate) use get_req_builder;
pub(crate) use get_req_builder_arg;
