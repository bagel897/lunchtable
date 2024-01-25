use std::{pin::Pin, time::Duration};

use rocket_db_pools::{deadpool_redis, Database};
// use futures::Stream;
use juniper::{graphql_object, graphql_subscription, EmptyMutation, EmptySubscription, RootNode};
use uuid::Uuid;

use crate::{
    api::{Match, Status},
    core::Error,
};

use super::{database::Database, redis::Cache};

pub(crate) struct Query;
// #[graphql_object(context = Context)]
impl Query {
    async fn simple() -> Uuid {
        Uuid::default()
    }
    async fn get_status<'c>(user: Uuid, cache: &Cache) -> Status {
        cache.get_status(user).await.unwrap()
    }
}

pub(crate) struct Context<'a> {
    cache: &'a Cache,
    // database: &'a DeadpoolDatabase
}
impl<'a> Context<'a> {
    pub fn new(cache: &'a Cache) -> Self {
        Self { cache }
    }
}
// type MeetingStream = Pin<Box<dyn Stream<Item = Result<Match, Error>> + Send>>;
// struct Subscription;
//
// #[graphql_subscription(context = Context)]
// impl Subscription {
//     async fn propose_meetings() -> MeetingStream {
//         let mut interval = tokio::time::interval(Duration::from_secs(5));
//         let stream = async_stream::stream! {
//             let mut counter = 0;
//             loop {
//                 counter += 1;
//                 interval.tick().await;
//         todo!();
//             }
//         };
//
//         Box::pin(stream)
//     }
// }
//
pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub(crate) fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::new(), EmptySubscription::new())
}
