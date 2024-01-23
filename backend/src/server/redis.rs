use redis::{aio::MultiplexedConnection, AsyncCommands, Client, Commands};
use tracing::info;
use uuid::Uuid;

use crate::api::Status;
#[derive(Clone, Debug)]
pub struct Cache {
    client: MultiplexedConnection,
}
impl Cache {
    pub async fn new() -> Self {
        info!("Connecting to Redis");
        Self {
            client: Client::open("redis://127.0.0.1/")
                .unwrap()
                .get_multiplexed_async_connection()
                .await
                .unwrap(),
        }
    }

    pub async fn get_status(&self, user: Uuid) -> Result<Status, redis::RedisError> {
        self.client.get(&user.to_bytes_le()).await
    }
}
