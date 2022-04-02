macro_rules! get_req_builder_arg {
    (ids) => {
        pub fn ids(
            &mut self,
            ids: impl IntoIterator<Item = impl $crate::id::IntoNumericId>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("ids", ids);
            self
        }
    };
    (media_fields) => {
        pub fn media_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::MediaField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("media.fields", fields);
            self
        }
    };
    (place_fields) => {
        pub fn place_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::PlaceField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("place.fields", fields);
            self
        }
    };
    (poll_fields) => {
        pub fn poll_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::PollField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("poll.fields", fields);
            self
        }
    };
    (user_fields) => {
        pub fn user_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::UserField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("user.fields", fields);
            self
        }
    };
    (tweet_fields) => {
        pub fn tweet_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::TweetField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("tweet.fields", fields);
            self
        }
    };
    (space_fields) => {
        pub fn space_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::SpaceField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("space.fields", fields);
            self
        }
    };
    (topic_fields) => {
        pub fn topic_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::TopicField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("topic.fields", fields);
            self
        }
    };
    (list_fields) => {
        pub fn list_fields(
            &mut self,
            fields: impl IntoIterator<Item = $crate::query::ListField>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("list.fields", fields);
            self
        }
    };
    (tweet_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::query::TweetExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (user_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::query::UserExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (space_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::query::SpaceExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (list_expansions) => {
        pub fn expansions(
            &mut self,
            expansions: impl IntoIterator<Item = $crate::query::ListExpansion>,
        ) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_seq("expansions", expansions);
            self
        }
    };
    (exclude) => {
        pub fn exclude(
            &mut self,
            exclude: impl IntoIterator<Item = $crate::query::Exclude>,
        ) -> &mut Self {
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
        pub fn since_id(&mut self, since_id: impl $crate::id::IntoNumericId) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("since_id", since_id);
            self
        }
    };
    (until_id) => {
        pub fn until_id(&mut self, until_id: impl $crate::id::IntoNumericId) -> &mut Self {
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
    (sort_order) => {
        pub fn sort_order(&mut self, sort_order: $crate::query::SortOrder) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("sort_order", sort_order);
            self
        }
    };
    (granularity) => {
        pub fn granularity(&mut self, granularity: $crate::query::Granularity) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("granularity", granularity);
            self
        }
    };
    (backfill) => {
        pub fn backfill(&mut self, backfill: std::time::Duration) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url
                .append_query_val("backfill_minutes", backfill.as_secs() / 60);
            self
        }
    };
    (space_state) => {
        pub fn state(&mut self, state: $crate::query::SpaceStateQuery) -> &mut Self {
            use $crate::query::UrlQueryExt;
            self.url.append_query_val("state", state);
            self
        }
    };
}

macro_rules! get_req_builder_verb {
    (send) => {
        pub async fn send(&self) -> $crate::ApiResult<A, T, M> {
            self.client
                .send(self.client.request(reqwest::Method::GET, self.url.clone()))
                .await
        }
    };
    (stream) => {
        pub async fn stream(
            &self,
        ) -> $crate::Result<
            impl futures::stream::Stream<Item = $crate::Result<$crate::ApiPayload<T, M>>>,
        > {
            self.client
                .stream(self.client.request(reqwest::Method::GET, self.url.clone()))
                .await
        }
    };
}

macro_rules! get_req_builder {
    ($vis:vis struct $class:ident { $($optional_arg:tt),* }) => {
        get_req_builder!{#[send] $vis struct $class { $($optional_arg),* }}
    };
    (#[$verb:tt] $vis:vis struct $class:ident { $($optional_arg:tt),* }) => {
        $vis struct $class<A, T, M> {
            client: $crate::TwitterApi<A>,
            url: url::Url,
            return_ty: std::marker::PhantomData<(T, M)>
        }

        impl<A, T, M> $class<A, T, M>
        where
            A: $crate::authorization::Authorization,
            T: serde::de::DeserializeOwned,
            M: serde::de::DeserializeOwned
        {
            pub(crate) fn new(client: &$crate::TwitterApi<A>, url: url::Url) -> Self {
                Self { client: client.clone(), url, return_ty: Default::default() }
            }
            $($crate::query::get_req_builder_arg! { $optional_arg })*
            $crate::query::get_req_builder_verb! { $verb }
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
pub(crate) use get_req_builder_verb;
