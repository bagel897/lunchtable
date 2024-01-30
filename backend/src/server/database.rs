use crate::{
    api::User,
    core::{LunchtableError, LunchtableResult},
    models::{UserActiveModel, UserEntity, UserModel},
};

use super::config::Config;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}
impl Database {
    pub async fn new(config: &Config) -> Self {
        Self {
            connection: sea_orm::Database::connect(config.postgres_url.clone())
                .await
                .unwrap(),
        }
    }
    pub async fn create_user(&self, user: User) -> LunchtableResult<User> {
        let db_user: UserActiveModel = user.into();
        let res = db_user.insert(&self.connection).await?;
        Ok(res.into())
    }
    pub async fn get_user(&self, user: Uuid) -> LunchtableResult<User> {
        let res = UserEntity::find_by_id(user).one(&self.connection).await?;
        match res {
            Some(user) => Ok(user.into()),
            None => Err(LunchtableError::UserNotFound { user }),
        }
    }
}
