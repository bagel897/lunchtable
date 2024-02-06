use entity::UserEntity;
use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);
        manager
            .create_table(
                schema
                    .create_table_from_entity(UserEntity)
                    // .table("user_table")
                    .if_not_exists()
                    .to_owned(),
            )
            .await
        // .create_table(
        //     Table::create()
        //         .table(UserModel::T)
        //         .if_not_exists()
        //         .col(
        //             ColumnDef::new(Post::Id)
        //                 .integer()
        //                 .not_null()
        //                 .auto_increment()
        //                 .primary_key(),
        //         )
        //         .col(ColumnDef::new(Post::Title).string().not_null())
        //         .col(ColumnDef::new(Post::Text).string().not_null())
        //         .to_owned(),
        // )
        // .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        // manager
        //     .drop_table(Table::drop().table(Post::Table).to_owned())
        //     .await
    }
}
