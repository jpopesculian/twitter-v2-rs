use url::Url;

fn to_query_value<T>(value: T) -> String
where
    T: IntoIterator,
    T::Item: ToString,
{
    value
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub(crate) trait ToQuery {
    fn to_query(self, query: &str) -> Vec<(&str, String)>;
}

impl<T> ToQuery for T
where
    T: IntoIterator,
    T::Item: ToString,
{
    fn to_query(self, key: &str) -> Vec<(&str, String)> {
        let value = to_query_value(self);
        if value.is_empty() {
            vec![]
        } else {
            vec![(key, value)]
        }
    }
}

pub trait UrlQueryExt {
    fn append_query_seq<T>(&mut self, key: &str, value: T)
    where
        T: IntoIterator,
        T::Item: ToString;
    fn append_query_val<T>(&mut self, key: &str, value: T)
    where
        T: ToString;
    fn append_query_time(&mut self, key: &str, time: time::OffsetDateTime) {
        self.append_query_val(
            key,
            time.format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        )
    }
}

impl UrlQueryExt for Url {
    fn append_query_seq<T>(&mut self, key: &str, value: T)
    where
        T: IntoIterator,
        T::Item: ToString,
    {
        let value = to_query_value(value);
        if !value.is_empty() {
            self.query_pairs_mut().append_pair(key, &value);
        }
    }
    fn append_query_val<T>(&mut self, key: &str, value: T)
    where
        T: ToString,
    {
        self.query_pairs_mut().append_pair(key, &value.to_string());
    }
}
