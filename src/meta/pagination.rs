pub trait PaginationMeta {
    fn next_token(&self) -> Option<&str>;
    fn previous_token(&self) -> Option<&str>;
}
