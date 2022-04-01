use strum::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SpaceStateQuery {
    All,
    Scheduled,
    Live,
}
