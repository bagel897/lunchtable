use juniper::GraphQLObject;
use uuid::Uuid;

use crate::models::UserModel;

// #[derive(GraphQLObject)]
struct Login {}

#[derive(GraphQLObject, Default, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub friends: Vec<Uuid>, // login: Login,
                            // calendar_creds: todo!(),
                            // profile_pic:todo!(),
}
impl From<UserModel> for User {
    fn from(value: UserModel) -> Self {
        // TODO: friends
        Self {
            id: value.id,
            name: value.name,
            ..Default::default()
        }
    }
}
