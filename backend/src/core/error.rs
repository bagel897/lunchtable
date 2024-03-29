use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum LunchtableError {
    #[error("user not found {user:?}")]
    UserNotFound { user: Uuid },
    #[error("generic redis error {0}")]
    GenericRedis(#[from] deadpool_redis::redis::RedisError),
    #[error("postgres error")]
    GenericPostgres(#[from] sea_orm::DbErr),
    #[error("Serialization/Deserialization error {0}")]
    Serde(#[from] serde_json::Error),
}
pub type LunchtableResult<T> = Result<T, LunchtableError>;
