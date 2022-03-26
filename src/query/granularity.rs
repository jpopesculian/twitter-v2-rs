use strum::Display;

#[derive(Copy, Clone, Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Granularity {
    Minute,
    Hour,
    Day,
}
