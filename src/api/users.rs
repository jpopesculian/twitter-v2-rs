use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Blocking, Following, Muting, User};
use crate::id::IntoId;
use crate::meta::ResultCountMeta;
use crate::query::{GetRelatedUsersRequestBuilder, GetUsersRequestBuilder, UrlQueryExt};
use crate::requests::TargetUserId;
use crate::utils::percent_encode;
use reqwest::Method;

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_users(
        &self,
        ids: impl IntoIterator<Item = impl IntoId>,
    ) -> GetUsersRequestBuilder<A, Vec<User>, ()> {
        let mut url = self.url("users").unwrap();
        url.append_query_seq("ids", ids);
        GetUsersRequestBuilder::new(self, url)
    }
    pub fn get_user(&self, id: impl IntoId) -> GetUsersRequestBuilder<A, User, ()> {
        GetUsersRequestBuilder::new(self, self.url(format!("users/{id}")).unwrap())
    }
    pub fn get_users_by_usernames(
        &self,
        usernames: impl IntoIterator<Item = impl ToString>,
    ) -> GetUsersRequestBuilder<A, Vec<User>, ()> {
        let mut url = self.url("users/by").unwrap();
        url.append_query_seq("usernames", usernames);
        GetUsersRequestBuilder::new(self, url)
    }
    pub fn get_user_by_username(
        &self,
        username: impl ToString,
    ) -> GetUsersRequestBuilder<A, User, ()> {
        GetUsersRequestBuilder::new(
            self,
            self.url(format!(
                "users/by/username/{}",
                percent_encode(&username.to_string())
            ))
            .unwrap(),
        )
    }
    pub fn get_users_me(&self) -> GetUsersRequestBuilder<A, User, ()> {
        GetUsersRequestBuilder::new(self, self.url("users/me").unwrap())
    }
    pub fn get_user_followers(
        &self,
        id: impl IntoId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("users/{id}/followers")).unwrap())
    }
    pub fn get_user_following(
        &self,
        id: impl IntoId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("users/{id}/following")).unwrap())
    }
    pub async fn post_user_following(
        &self,
        id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Following, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{id}/following"))?)
                .json(&TargetUserId::from(target_user_id)),
        )
        .await
    }
    pub async fn delete_user_following(
        &self,
        source_user_id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Following, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{source_user_id}/following/{target_user_id}"))?,
        ))
        .await
    }
    pub fn get_user_blocking(
        &self,
        id: impl IntoId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("users/{id}/blocking")).unwrap())
    }
    pub async fn post_user_blocking(
        &self,
        id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Blocking, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{id}/blocking"))?)
                .json(&TargetUserId::from(target_user_id)),
        )
        .await
    }
    pub async fn delete_user_blocking(
        &self,
        source_user_id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Blocking, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{source_user_id}/blocking/{target_user_id}"))?,
        ))
        .await
    }
    pub fn get_user_muting(
        &self,
        id: impl IntoId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("users/{id}/muting")).unwrap())
    }
    pub async fn post_user_muting(
        &self,
        id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Muting, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{id}/muting"))?)
                .json(&TargetUserId::from(target_user_id)),
        )
        .await
    }
    pub async fn delete_user_muting(
        &self,
        source_user_id: impl IntoId,
        target_user_id: impl IntoId,
    ) -> ApiResult<A, Muting, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{source_user_id}/muting/{target_user_id}"))?,
        ))
        .await
    }
}
