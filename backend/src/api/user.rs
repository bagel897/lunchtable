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
