use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Deleted, Following, IsMember, List, Pinned, Tweet, Updated, User};
use crate::id::IntoNumericId;
use crate::meta::ResultCountMeta;
use crate::query::{
    GetLimitedRelatedTweetsRequestBuilder, GetListsRequestBuilder, GetPaginatedListsRequestBuilder,
    GetRelatedUsersRequestBuilder,
};
use crate::requests::{ListBuilder, ListId, UserId};
use reqwest::Method;

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_list(&self, id: impl IntoNumericId) -> GetListsRequestBuilder<A, List, ()> {
        GetListsRequestBuilder::new(self, self.url(format!("lists/{id}")).unwrap())
    }
    pub fn get_user_owned_lists(
        &self,
        id: impl IntoNumericId,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        GetPaginatedListsRequestBuilder::new(
            self,
            self.url(format!("users/{id}/owned_lists")).unwrap(),
        )
    }
    pub fn post_list(&self, name: impl ToString) -> ListBuilder<A, List> {
        let mut builder = ListBuilder::new(self, self.url("lists").unwrap(), Method::POST);
        builder.name(name);
        builder
    }
    pub fn put_list(&self, id: impl IntoNumericId) -> ListBuilder<A, Updated> {
        ListBuilder::new(self, self.url(format!("lists/{id}")).unwrap(), Method::PUT)
    }
    pub async fn delete_list(&self, id: impl IntoNumericId) -> ApiResult<A, Deleted, ()> {
        self.send(self.request(Method::DELETE, self.url(format!("lists/{id}"))?))
            .await
    }
    pub fn get_list_tweets(
        &self,
        id: impl IntoNumericId,
    ) -> GetLimitedRelatedTweetsRequestBuilder<A, Vec<Tweet>, ResultCountMeta> {
        GetLimitedRelatedTweetsRequestBuilder::new(
            self,
            self.url(format!("lists/{id}/tweets")).unwrap(),
        )
    }
    pub fn get_user_list_memberships(
        &self,
        id: impl IntoNumericId,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        GetPaginatedListsRequestBuilder::new(
            self,
            self.url(format!("users/{id}/list_memberships")).unwrap(),
        )
    }
    pub fn get_list_members(
        &self,
        id: impl IntoNumericId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("list/{id}/members")).unwrap())
    }
    pub async fn post_list_member(
        &self,
        id: impl IntoNumericId,
        user_id: impl IntoNumericId,
    ) -> ApiResult<A, IsMember, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("lists/{id}/members"))?)
                .json(&UserId::from(user_id)),
        )
        .await
    }
    pub async fn delete_list_member(
        &self,
        id: impl IntoNumericId,
        user_id: impl IntoNumericId,
    ) -> ApiResult<A, IsMember, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("lists/{id}/members/{user_id}"))?,
        ))
        .await
    }
    pub fn get_list_followers(
        &self,
        id: impl IntoNumericId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("list/{id}/followers")).unwrap())
    }
    pub fn get_user_followed_lists(
        &self,
        id: impl IntoNumericId,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        GetPaginatedListsRequestBuilder::new(
            self,
            self.url(format!("users/{id}/followed_lists")).unwrap(),
        )
    }
    pub async fn post_user_followed_list(
        &self,
        id: impl IntoNumericId,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.send(
            self.request(
                Method::POST,
                self.url(format!("users/{id}/followed_lists"))?,
            )
            .json(&ListId::from(list_id)),
        )
        .await
    }
    pub async fn delete_user_followed_list(
        &self,
        id: impl IntoNumericId,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{id}/followed_lists/{list_id}"))?,
        ))
        .await
    }
    pub fn get_user_pinned_lists(
        &self,
        id: impl IntoNumericId,
    ) -> GetListsRequestBuilder<A, Vec<List>, ()> {
        GetListsRequestBuilder::new(self, self.url(format!("users/{id}/lists")).unwrap())
    }
    pub async fn post_user_pinned_list(
        &self,
        id: impl IntoNumericId,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Pinned, ()> {
        self.send(
            self.request(Method::POST, self.url(format!("users/{id}/pinned_lists"))?)
                .json(&ListId::from(list_id)),
        )
        .await
    }
    pub async fn delete_user_pinned_list(
        &self,
        id: impl IntoNumericId,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Pinned, ()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("users/{id}/pinned_lists/{list_id}"))?,
        ))
        .await
    }
}
