use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use rocket_db_pools::{
    deadpool_redis::{self, redis::AsyncCommands},
    Database,
};
use tracing::info;
use uuid::Uuid;

use crate::api::Status;
#[derive(Database)]
#[database("cache")]
pub struct Cache(deadpool_redis::Pool);
impl Cache {
    pub async fn get_status(&self, user: Uuid) -> Result<Status, redis::RedisError> {
        let conn = self.get().await.unwrap();
        conn.get(&user.to_bytes_le()).await
    }
    pub async fn set_status(&self, user: Uuid, status: Status) -> Result<(), redis::RedisError> {
        let conn = self.get().await.unwrap();
        conn.set(&user.to_bytes_le(), status).await?;
        Ok(())
    }
}
