use entity::UserModel;
use juniper::GraphQLObject;
use uuid::Uuid;

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
// impl Into<UserActiveModel> for User {
//     fn into(self) -> UserActiveModel {
//         UserActiveModel {}
//     }
// }
impl From<User> for UserModel {
    fn from(value: User) -> Self {
        // TODO: impl friends
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
