use crate::fields::Field;
use serde::Serialize;

pub(crate) trait ToQuery {
    fn to_query(self, query: &str) -> Vec<(&str, String)>;
}

impl<T> ToQuery for T
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn to_query(self, key: &str) -> Vec<(&str, String)> {
        let value = self
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",");
        if value.is_empty() {
            vec![]
        } else {
            vec![(key, value)]
        }
    }
}

#[derive(Default, Serialize)]
pub(crate) struct FieldsQuery {
    #[serde(rename = "media.fields")]
    media: Option<String>,
    #[serde(rename = "place.fields")]
    place: Option<String>,
    #[serde(rename = "poll.fields")]
    poll: Option<String>,
    #[serde(rename = "tweet.fields")]
    tweet: Option<String>,
    #[serde(rename = "user.fields")]
    user: Option<String>,
}

impl<'a> FromIterator<&'a Field> for FieldsQuery {
    fn from_iter<T: IntoIterator<Item = &'a Field>>(iter: T) -> Self {
        let mut fields = FieldsQuery::default();
        for field in iter {
            match field {
                Field::Media(media) => {
                    if fields.media.is_none() {
                        fields.media = Some(String::new());
                    }
                    let field = fields.media.as_mut().unwrap();
                    field.push_str(&media.to_string());
                    field.push(',');
                }
                Field::Place(place) => {
                    if fields.place.is_none() {
                        fields.place = Some(String::new());
                    }
                    let field = fields.place.as_mut().unwrap();
                    field.push_str(&place.to_string());
                    field.push(',');
                }
                Field::Poll(poll) => {
                    if fields.poll.is_none() {
                        fields.poll = Some(String::new());
                    }
                    let field = fields.poll.as_mut().unwrap();
                    field.push_str(&poll.to_string());
                    field.push(',');
                }
                Field::Tweet(tweet) => {
                    if fields.tweet.is_none() {
                        fields.tweet = Some(String::new());
                    }
                    let field = fields.tweet.as_mut().unwrap();
                    field.push_str(&tweet.to_string());
                    field.push(',');
                }
                Field::User(user) => {
                    if fields.user.is_none() {
                        fields.user = Some(String::new());
                    }
                    let field = fields.user.as_mut().unwrap();
                    field.push_str(&user.to_string());
                    field.push(',');
                }
            }
        }
        if let Some(field) = fields.media.as_mut() {
            let _ = field.pop();
        }
        if let Some(field) = fields.place.as_mut() {
            let _ = field.pop();
        }
        if let Some(field) = fields.poll.as_mut() {
            let _ = field.pop();
        }
        if let Some(field) = fields.tweet.as_mut() {
            let _ = field.pop();
        }
        if let Some(field) = fields.user.as_mut() {
            let _ = field.pop();
        }
        fields
    }
}

pub(crate) trait FieldsToQuery {
    fn to_fields_query(self) -> FieldsQuery;
}

impl<'a, T> FieldsToQuery for T
where
    T: IntoIterator<Item = &'a Field>,
{
    fn to_fields_query(self) -> FieldsQuery {
        self.into_iter().collect()
    }
}
