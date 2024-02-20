use chrono::{DateTime, Utc};
use deadpool_redis::redis;
use juniper::{GraphQLEnum, GraphQLObject, GraphQLScalar};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
#[derive(GraphQLEnum, Clone, Copy, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub enum Reason {
    MANUAL,
    CALENDAR,
}
#[derive(GraphQLEnum, Clone, Copy, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub enum StatusKind {
    Free,
    Busy,
}
#[derive(GraphQLScalar, Clone, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
#[graphql(transparent)]
pub struct Duration {
    time: DateTime<Utc>,
}

#[derive(GraphQLObject, Clone, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Status {
    pub kind: StatusKind,
    pub duration: Option<Duration>,
    pub reason: Reason,
}
