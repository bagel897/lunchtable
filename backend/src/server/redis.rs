use crate::{
    api::Status,
    core::{LunchtableError, LunchtableResult},
};
use deadpool_redis::{
    redis::{AsyncCommands, ErrorKind, RedisError},
    Pool,
};
use tracing::error;
use uuid::Uuid;

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
    fn check_not_found<T>(user: Uuid, result: Result<T, RedisError>) -> LunchtableResult<T> {
        match result {
            Ok(t) => Ok(t),
            Err(e) => {
                if e.kind() == ErrorKind::TypeError {
                    error!("{:?}", e);
                    Err(LunchtableError::UserNotFound { user })
                } else {
                    Err(e.into())
                }
            }
        }
    }
    pub async fn get_status(&self, user: Uuid) -> LunchtableResult<Status> {
        let mut conn = self.pool.get().await.unwrap();
        let res = conn.get(&user.to_bytes_le()).await;
        Self::check_not_found(user, res)
    }
    pub async fn set_status(&self, user: Uuid, status: Status) -> LunchtableResult<()> {
        let mut conn = self.pool.get().await.unwrap();
        let res = conn.set(&user.to_bytes_le(), status).await;
        Self::check_not_found(user, res)
    }
    pub async fn add_status(&self, user: Uuid, status: Status) -> LunchtableResult<()> {
        let mut conn = self.pool.get().await.unwrap();
        let res = conn.set(&user.to_bytes_le(), status).await;
        Self::check_not_found(user, res)
    }
}
