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
