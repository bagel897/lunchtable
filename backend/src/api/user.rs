use juniper::GraphQLObject;
use uuid::Uuid;

// #[derive(GraphQLObject)]
struct Login {}

#[derive(GraphQLObject)]
pub struct User {
    id: Uuid,
    name: String,
    friends: Vec<Uuid>, // login: Login,
                        // calendar_creds: todo!(),
                        // profile_pic:todo!(),
}
