use crate::server::run_server;
mod api;
mod core;
mod server;
#[rocket::main]
async fn main() {
    tracing_subscriber::fmt::init();
    run_server().await;
}
