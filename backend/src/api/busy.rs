use chrono::{DateTime, Utc};
use juniper::{GraphQLEnum, GraphQLObject, GraphQLUnion};
#[derive(GraphQLEnum)]
enum Reason {
    MANUAL,
    CALENDAR,
}
#[derive(GraphQLObject)]
pub struct Duration {
    time: Option<DateTime<Utc>>,
}

#[derive(GraphQLObject)]
pub struct Busy {
    till: Duration,
    reason: Reason,
}
#[derive(GraphQLObject)]
pub struct Free {
    till: Duration,
    reason: Reason,
}

#[derive(GraphQLUnion)]
pub enum Status {
    Free(Free),
    Busy(Busy),
}
