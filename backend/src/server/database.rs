use crate::{
    api::User,
    core::LunchtableResult,
    models::{UserActiveModel, UserModel},
};

use super::config::Config;
use sea_orm::{ActiveModelTrait, DatabaseConnection};

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
    pub async fn create_user(&self, user: User) -> LunchtableResult<UserModel> {
        let db_user: UserActiveModel = user.into();
        let res = db_user.insert(&self.connection).await?;
        Ok(res)
    }
}
