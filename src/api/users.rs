use super::TwitterApi;
use crate::authorization::Authorization;
use crate::data::User;
use crate::id::IntoId;
use crate::meta::NoMeta;
use crate::query::get_req_builder;
use crate::utils::percent_encode;
use crate::UrlQueryExt;
use reqwest::Method;

get_req_builder! {
pub struct GetUsersRequestBuilder {
    user_fields,
    tweet_fields,
    user_expansions
}
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_users(
        &self,
        ids: impl IntoIterator<Item = impl IntoId>,
    ) -> GetUsersRequestBuilder<A, Vec<User>, NoMeta> {
        let mut url = self.url("users").unwrap();
        url.append_query_seq("ids", ids);
        GetUsersRequestBuilder::new(self, url)
    }
    pub fn get_user(&self, id: impl IntoId) -> GetUsersRequestBuilder<A, User, NoMeta> {
        GetUsersRequestBuilder::new(self, self.url(format!("users/{id}")).unwrap())
    }
    pub fn get_users_by_usernames(
        &self,
        usernames: impl IntoIterator<Item = impl ToString>,
    ) -> GetUsersRequestBuilder<A, Vec<User>, NoMeta> {
        let mut url = self.url("users/by").unwrap();
        url.append_query_seq("usernames", usernames);
        GetUsersRequestBuilder::new(self, url)
    }
    pub fn get_user_by_username(
        &self,
        username: impl ToString,
    ) -> GetUsersRequestBuilder<A, User, NoMeta> {
        GetUsersRequestBuilder::new(
            self,
            self.url(format!(
                "users/by/username/{}",
                percent_encode(&username.to_string())
            ))
            .unwrap(),
        )
    }
    pub fn get_users_me(&self) -> GetUsersRequestBuilder<A, User, NoMeta> {
        GetUsersRequestBuilder::new(self, self.url("users/me").unwrap())
    }
}
