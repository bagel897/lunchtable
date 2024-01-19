use chrono::DateTime;
use chrono_tz::Tz;
use juniper::{GraphQLEnum, GraphQLObject, GraphQLUnion};

#[derive(GraphQLEnum)]
enum Reason {
    MANUAL,
    CALENDAR,
}
#[derive(GraphQLObject)]
pub struct Duration {
    time: Option<DateTime<Tz::Etc__UTC>>,
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
