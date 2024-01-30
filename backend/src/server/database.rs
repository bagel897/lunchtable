use crate::{api::User, core::LunchtableResult};

use super::config::Config;
use sea_orm::DatabaseConnection;

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
    pub async fn create_user(&mut self, user: User) -> LunchtableResult<()> {
        todo!()
    }
}
