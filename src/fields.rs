use serde::Serialize;
use strum::Display;

#[macro_export]
macro_rules! fields {
    ($($x:expr),*) => {
        [$($x.into()),*]
    };
}

#[derive(Copy, Clone, Debug)]
pub enum Field {
    Media(MediaField),
    Place(PlaceField),
    Poll(PollField),
    Tweet(TweetField),
    User(UserField),
}

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MediaField {
    DurationMs,
    Height,
    MediaKey,
    PreviewImageUrl,
    Type,
    Url,
    Width,
    PublicMetrics,
    NonPublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
    AltText,
}

impl From<MediaField> for Field {
    fn from(field: MediaField) -> Self {
        Self::Media(field)
    }
}

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum PlaceField {
    ContainedWithin,
    Country,
    CountryCode,
    FullName,
    Geo,
    Id,
    Name,
    PlaceType,
}

impl From<PlaceField> for Field {
    fn from(field: PlaceField) -> Self {
        Self::Place(field)
    }
}

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum PollField {
    DurationMinutes,
    EndDatetime,
    Id,
    Options,
    VotingStatus,
}

impl From<PollField> for Field {
    fn from(field: PollField) -> Self {
        Self::Poll(field)
    }
}

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum TweetField {
    Attachments,
    AuthorId,
    ContextAnnotations,
    ConversationId,
    CreatedAt,
    Entities,
    Geo,
    Id,
    InReplyToUserId,
    Lang,
    NonPublicMetrics,
    PublicMetrics,
    OrganicMetrics,
    PromotedMetrics,
    PossiblySensitive,
    ReferencedTweets,
    ReplySettings,
    Source,
    Text,
    Withheld,
}

impl From<TweetField> for Field {
    fn from(field: TweetField) -> Self {
        Self::Tweet(field)
    }
}

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum UserField {
    CreatedAt,
    Description,
    Entities,
    Id,
    Location,
    Name,
    PinnedTweetId,
    ProfileImageUrl,
    Protected,
    PublicMetrics,
    Url,
    Username,
    Verified,
    Withheld,
}

impl From<UserField> for Field {
    fn from(field: UserField) -> Self {
        Self::User(field)
    }
}
