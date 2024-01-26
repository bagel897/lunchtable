use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum LunchtableError {
    #[error("user not found")]
    UserNotFoundError { user: Uuid },
    #[error("generic redis error")]
    GenericRedisError(#[from] deadpool_redis::redis::RedisError),
}
