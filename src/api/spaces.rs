use super::TwitterApi;
use crate::authorization::Authorization;
use crate::data::{Space, Tweet, User};
use crate::id::{IntoNumericId, IntoStringId};
use crate::meta::SimpleResultCountMeta;
use crate::query::{
    GetRelatedTweetsRequestBuilder, GetRelatedUsersRequestBuilder, GetSpacesRequestBuilder,
    GetSpacesSearchRequestBuilder, UrlQueryExt,
};

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_spaces(
        &self,
        ids: impl IntoIterator<Item = impl IntoStringId>,
    ) -> GetSpacesRequestBuilder<A, Vec<Space>, ()> {
        let mut url = self.url("spaces").unwrap();
        url.append_query_seq("ids", ids);
        GetSpacesRequestBuilder::new(self, url)
    }
    pub fn get_space(&self, id: impl IntoStringId) -> GetSpacesRequestBuilder<A, Space, ()> {
        GetSpacesRequestBuilder::new(self, self.url(format!("spaces/{id}")).unwrap())
    }
    pub fn get_spaces_by_creator_ids(
        &self,
        user_ids: impl IntoIterator<Item = impl IntoNumericId>,
    ) -> GetSpacesRequestBuilder<A, Vec<Space>, SimpleResultCountMeta> {
        let mut url = self.url("spaces/by/creator_ids").unwrap();
        url.append_query_seq("user_ids", user_ids);
        GetSpacesRequestBuilder::new(self, url)
    }
    pub fn get_space_buyers(
        &self,
        id: impl IntoStringId,
    ) -> GetRelatedUsersRequestBuilder<A, Vec<User>, ()> {
        GetRelatedUsersRequestBuilder::new(self, self.url(format!("spaces/{id}/buyers")).unwrap())
    }
    pub fn get_space_tweets(
        &self,
        id: impl IntoStringId,
    ) -> GetRelatedTweetsRequestBuilder<A, Vec<Tweet>, ()> {
        GetRelatedTweetsRequestBuilder::new(self, self.url(format!("spaces/{id}/tweets")).unwrap())
    }
    pub fn get_spaces_search(
        &self,
        query: impl ToString,
    ) -> GetSpacesSearchRequestBuilder<A, Vec<Space>, SimpleResultCountMeta> {
        let mut url = self.url("spaces/search").unwrap();
        url.append_query_val("query", query);
        GetSpacesSearchRequestBuilder::new(self, url)
    }
}
