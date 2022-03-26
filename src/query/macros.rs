macro_rules! get_req_builder_arg {
    (media_fields) => {
        pub fn media_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::MediaField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("media.fields", fields);
            self
        }
    };
    (place_fields) => {
        pub fn place_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::PlaceField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("place.fields", fields);
            self
        }
    };
    (poll_fields) => {
        pub fn poll_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::PollField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("poll.fields", fields);
            self
        }
    };
    (user_fields) => {
        pub fn user_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::UserField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("user.fields", fields);
            self
        }
    };
    (tweet_fields) => {
        pub fn tweet_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::TweetField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("tweet.fields", fields);
            self
        }
    };
    (tweet_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::TweetExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (user_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::UserExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (exclude) => {
        pub fn exclude(&mut self, exclude: impl IntoIterator<Item = $crate::Exclude>) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("exclude", exclude);
            self
        }
    };
    (max_results) => {
        pub fn max_results(&mut self, max_results: usize) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("max_results", max_results);
            self
        }
    };
    (pagination_token) => {
        pub fn pagination_token(&mut self, pagination_token: &str) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url
                .append_query_val("pagination_token", pagination_token);
            self
        }
    };
    (since_id) => {
        pub fn since_id(&mut self, since_id: impl $crate::IntoId) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("since_id", since_id);
            self
        }
    };
    (until_id) => {
        pub fn until_id(&mut self, until_id: impl $crate::IntoId) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("until_id", until_id);
            self
        }
    };
    (start_time) => {
        pub fn start_time(&mut self, start_time: time::OffsetDateTime) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_time("start_time", start_time);
            self
        }
    };
    (end_time) => {
        pub fn end_time(&mut self, end_time: time::OffsetDateTime) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_time("end_time", end_time);
            self
        }
    };
}

macro_rules! get_req_builder {
    ($vis:vis struct $class:ident { $($optional_arg:tt),* }) => {
        $vis struct $class<A, T, M> {
            client: $crate::TwitterApi<A>,
            url: url::Url,
            return_ty: std::marker::PhantomData<(T, M)>
        }

        impl<A, T, M> $class<A, T, M>
        where
            A: $crate::Authorization,
            T: serde::de::DeserializeOwned,
            M: serde::de::DeserializeOwned
        {
            fn new(client: &$crate::TwitterApi<A>, url: url::Url) -> Self {
                Self { client: client.clone(), url, return_ty: Default::default() }
            }
            $($crate::query::get_req_builder_arg! { $optional_arg })*
            pub async fn send(self) -> $crate::ApiResult<A, T, M> {
                self.client
                    .send(self.client.request(Method::GET, self.url.clone())).await
            }
        }

        impl<A, T, M> Clone for $class<A, T, M> {
            fn clone(&self) -> Self {
                Self {
                    client: self.client.clone(),
                    url: self.url.clone(),
                    return_ty: Default::default()
                }
            }
        }
    };
}

pub(crate) use get_req_builder;
pub(crate) use get_req_builder_arg;
