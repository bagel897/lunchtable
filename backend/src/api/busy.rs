use chrono::{DateTime, Utc};
use deadpool_redis::redis;
use juniper::{GraphQLEnum, GraphQLObject, GraphQLUnion};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
#[derive(GraphQLEnum, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
enum Reason {
    MANUAL,
    CALENDAR,
}
#[derive(GraphQLObject, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Duration {
    time: Option<DateTime<Utc>>,
}

#[derive(GraphQLObject, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Busy {
    till: Duration,
    reason: Reason,
}
#[derive(GraphQLObject, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Free {
    till: Duration,
    reason: Reason,
}

#[derive(GraphQLUnion, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub enum Status {
    Free(Free),
    Busy(Busy),
}
