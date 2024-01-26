use super::graphql::Context;
use rocket::{response::content::RawHtml, routes, State};

use crate::server::graphql::Schema;

use super::{config::Config, graphql::schema};

#[rocket::get("/graphiql")]
fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}
#[rocket::get("/playground")]
fn playground() -> RawHtml<String> {
    juniper_rocket::playground_source("/graphql", None)
}

// GET request accepts query parameters like these:
// ?query=<urlencoded-graphql-query-string>
// &operationName=<optional-name>
// &variables=<optional-json-encoded-variables>
// See details here: https://graphql.org/learn/serving-over-http#get-request
#[rocket::get("/graphql?<request..>")]
async fn get_graphql(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}

#[rocket::post("/graphql", data = "<request>")]
async fn post_graphql(
    context: &State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(schema, context).await
}
pub(crate) async fn run_server() {
    let config = Config::from_env().unwrap();
    _ = rocket::build()
        .manage(Context::new(config))
        .manage(schema())
        .mount(
            "/",
            routes![graphiql, playground, get_graphql, post_graphql],
        )
        .launch()
        .await
        .expect("server to launch");
}
