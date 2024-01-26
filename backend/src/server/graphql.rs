use super::config::Config;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
use uuid::Uuid;

use crate::api::Status;

use super::{database::Database, redis::Cache};

pub(crate) struct Query;
#[graphql_object(context = Context)]
impl Query {
    async fn simple() -> Uuid {
        Uuid::default()
    }
    async fn get_status<'c>(user: Uuid, context: &'c Context) -> FieldResult<Status> {
        context.cache.get_status(user).await.map_err(|e| e.into())
    }
}

pub(crate) struct Context {
    cache: Cache,
    database: Database,
}
impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            cache: Cache::new(config),
            database: Database::new(),
        }
    }
}
impl juniper::Context for Context {}
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
