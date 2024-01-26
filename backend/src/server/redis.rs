use deadpool_redis::{
    redis::{AsyncCommands, RedisError},
    Pool,
};
use uuid::Uuid;

use crate::api::Status;

use super::config::Config;
#[derive(Clone)]
pub struct Cache {
    pool: Pool,
}
impl Cache {
    pub fn new(config: Config) -> Self {
        Self {
            pool: config
                .redis
                .create_pool(Some(deadpool_redis::Runtime::Tokio1))
                .unwrap(),
        }
    }
    pub async fn get_status(&self, user: Uuid) -> Result<Status, RedisError> {
        let mut conn = self.pool.get().await.unwrap();

        conn.get(&user.to_bytes_le()).await
    }
    pub async fn set_status(&self, user: Uuid, status: Status) -> Result<(), RedisError> {
        let mut conn = self.pool.get().await.unwrap();
        conn.set(&user.to_bytes_le(), status).await?;
        Ok(())
    }
}
