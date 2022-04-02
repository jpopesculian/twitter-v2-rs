use strum::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ComplianceJobStatusQuery {
    Created,
    InProgress,
    Failed,
    Complete,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ComplianceJobKindQuery {
    Tweets,
    Users,
}
