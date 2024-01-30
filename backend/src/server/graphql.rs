use super::config::Config;
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use uuid::Uuid;

use crate::api::{Duration, Reason, Status, StatusKind, User};

use super::{database::Database, redis::Cache};

pub(crate) struct Query;
#[graphql_object(context = Context)]
impl Query {
    async fn get_status(user: Uuid, context: &'_ Context) -> FieldResult<Status> {
        context.cache.get_status(user).await.map_err(|e| e.into())
    }

    async fn get_user(user: Uuid, context: &'_ Context) -> FieldResult<User> {
        todo!()
    }
}

pub(crate) struct Mutation;
#[graphql_object(context = Context)]
impl Mutation {
    async fn set_status(
        user: Uuid,
        kind: StatusKind,
        reason: Reason,
        duration: Duration,
        context: &'_ Context,
    ) -> FieldResult<Status> {
        let status = Status {
            kind,
            reason,
            duration,
        };
        context
            .cache
            .set_status(user, status)
            .await
            .map_err(|e| e.into())
    }

    async fn create_user(name: String, context: &'_ Context) -> FieldResult<User> {
        let user = User {
            id: Uuid::new_v4(),
            name,
            ..Default::default()
        };
        context.database.create_user(user.clone()).await?;
        Ok(user)
    }
}
pub(crate) struct Context {
    cache: Cache,
    database: Database,
}
impl Context {
    pub async fn new(config: Config) -> Self {
        Self {
            database: Database::new(&config).await,
            cache: Cache::new(config),
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
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub(crate) fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
