use super::get_req_builder;

get_req_builder! {
pub struct GetTweetsRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions
}
}

get_req_builder! {
pub struct GetTimelineRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    exclude,
    start_time,
    end_time,
    since_id,
    until_id,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetTweetsSearchRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    exclude,
    start_time,
    end_time,
    since_id,
    until_id,
    max_results,
    sort_order,
    pagination_token
}
}

get_req_builder! {
pub struct GetTweetsCountsRequestBuilder {
    start_time,
    end_time,
    since_id,
    until_id,
    granularity
}
}

get_req_builder! {
pub struct GetStreamRulesRequestBuilder {
    ids
}
}

get_req_builder! {
#[stream]
pub struct GetTweetsStreamRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    backfill
}
}

get_req_builder! {
pub struct GetTweetUsersRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    user_expansions,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetRelatedTweetsRequestBuilder {
    media_fields,
    user_fields,
    poll_fields,
    tweet_fields,
    place_fields,
    tweet_expansions,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetUsersRequestBuilder {
    user_fields,
    tweet_fields,
    user_expansions
}
}

get_req_builder! {
pub struct GetRelatedUsersRequestBuilder {
    user_fields,
    tweet_fields,
    user_expansions,
    max_results,
    pagination_token
}
}

get_req_builder! {
pub struct GetSpacesRequestBuilder {
    space_expansions,
    space_fields,
    topic_fields,
    user_fields
}
}

get_req_builder! {
pub struct GetSpacesSearchRequestBuilder {
    space_expansions,
    space_fields,
    topic_fields,
    user_fields,
    space_state
}
}
