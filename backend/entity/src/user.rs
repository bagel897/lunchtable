use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user_table")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    // login: Login,
    // calendar_creds: todo!(),
    // profile_pic:todo!(),
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "Entity")]
    Friends,
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Friends.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}