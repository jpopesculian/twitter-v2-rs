use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Blocking, Following, List, Muting, Pinned, Space, Tweet, User};
use crate::error::Result;
use crate::id::{IntoNumericId, NumericId};
use crate::meta::{ResultCountMeta, SimpleResultCountMeta, TweetsMeta};
use crate::query::{
    GetPaginatedListsRequestBuilder, GetRelatedUsersRequestBuilder, GetSpacesRequestBuilder,
    GetTimelineRequestBuilder,
};

pub struct TwitterApiWithUserCtx<A> {
    user_id: NumericId,
    client: TwitterApi<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub async fn with_user_ctx(&self) -> Result<TwitterApiWithUserCtx<A>> {
        let user_id = self.get_users_me().send().await?.into_data().unwrap().id;
        Ok(TwitterApiWithUserCtx {
            user_id,
            client: self.clone(),
        })
    }
}

impl<A> TwitterApiWithUserCtx<A>
where
    A: Authorization,
{
    pub fn get_my_owned_lists(
        &self,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        self.client.get_user_owned_lists(self.user_id)
    }
    pub fn get_my_list_memberships(
        &self,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        self.client.get_user_list_memberships(self.user_id)
    }
    pub fn get_my_followed_lists(
        &self,
    ) -> GetPaginatedListsRequestBuilder<A, Vec<List>, ResultCountMeta> {
        self.client.get_user_followed_lists(self.user_id)
    }
    pub async fn post_my_followed_list(
        &self,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.client
            .post_user_followed_list(self.user_id, list_id)
            .await
    }
    pub async fn delete_my_followed_list(
        &self,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.client
            .delete_user_followed_list(self.user_id, list_id)
            .await
    }
    pub async fn post_my_pinned_list(
        &self,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Pinned, ()> {
        self.client
            .post_user_pinned_list(self.user_id, list_id)
            .await
    }
    pub async fn delete_my_pinned_list(
        &self,
        list_id: impl IntoNumericId,
    ) -> ApiResult<A, Pinned, ()> {
        self.client
            .delete_user_pinned_list(self.user_id, list_id)
            .await
    }
    pub fn get_my_spaces(&self) -> GetSpacesRequestBuilder<A, Vec<Space>, SimpleResultCountMeta> {
        self.client.get_spaces_by_creator_ids([self.user_id])
    }
    pub fn get_my_tweets(&self) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        self.client.get_user_tweets(self.user_id)
    }
    pub fn get_my_mentions(&self) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        self.client.get_user_tweets(self.user_id)
    }
    pub fn get_my_followers(&self) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        self.client.get_user_followers(self.user_id)
    }
    pub fn get_my_following(&self) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        self.client.get_user_following(self.user_id)
    }
    pub async fn post_my_following(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.client
            .post_user_following(self.user_id, target_user_id)
            .await
    }
    pub async fn delete_my_following(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Following, ()> {
        self.client
            .delete_user_following(self.user_id, target_user_id)
            .await
    }
    pub fn get_my_blocking(&self) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ResultCountMeta> {
        self.client.get_user_blocking(self.user_id)
    }
    pub async fn post_my_blocking(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Blocking, ()> {
        self.client
            .post_user_blocking(self.user_id, target_user_id)
            .await
    }
    pub async fn delete_my_blocking(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Blocking, ()> {
        self.client
            .delete_user_blocking(self.user_id, target_user_id)
            .await
    }
    pub async fn post_my_muting(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Muting, ()> {
        self.client
            .post_user_muting(self.user_id, target_user_id)
            .await
    }
    pub async fn delete_my_muting(
        &self,
        target_user_id: impl IntoNumericId,
    ) -> ApiResult<A, Muting, ()> {
        self.client
            .delete_user_muting(self.user_id, target_user_id)
            .await
    }
}
