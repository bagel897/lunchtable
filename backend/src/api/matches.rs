use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use uuid::Uuid;
#[derive(GraphQLObject)]
pub struct Match {
    sender: Uuid,
    reciever: Uuid,
    duration: DateTime<Utc>,
}
