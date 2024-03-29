use crate::{
    api::User,
    core::{LunchtableError, LunchtableResult},
};
use entity::{UserActiveModel, UserEntity, UserModel};
use tracing::info;

use super::config::Config;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Schema, Set,
    TransactionTrait,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}
impl Database {
    pub async fn new(config: &Config) -> Self {
        let connection = sea_orm::Database::connect(config.postgres_url.clone())
            .await
            .unwrap();
        let builder = connection.get_database_backend();
        let schema = Schema::new(builder);
        info!("BRUH");
        let statement = builder.build(
            schema
                .create_table_from_entity(UserEntity)
                // .table("user_table")
                .if_not_exists(),
        );
        info!("BRUH");
        connection.execute(statement).await.unwrap();

        // Migrator::up(&connection, None).await.unwrap();
        Self { connection }
    }

    pub async fn add_friend(&self, friend: Uuid, new_friend: Uuid) -> LunchtableResult<User> {
        let txn = self.connection.begin().await?;
        let res = UserEntity::find_by_id(friend).one(&txn).await?;
        let user: UserModel = match res {
            Some(user) => Ok(user),
            None => Err(LunchtableError::UserNotFound { user: friend }),
        }?;
        let mut friends = user.friends.clone();
        friends.push(new_friend);
        let mut active_user: UserActiveModel = user.into();
        active_user.friends = Set(friends);
        let res = active_user.update(&txn).await?;
        txn.commit().await?;
        Ok(res.into())
    }
    pub async fn create_user(&self, user: User) -> LunchtableResult<User> {
        let _db_user: UserModel = user.into();
        let db_user: UserActiveModel = _db_user.into();
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
