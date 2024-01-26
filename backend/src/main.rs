use crate::server::run_server;
use tracing::info;

mod api;
mod core;
mod server;
#[rocket::main]
async fn main() {
    tracing_subscriber::fmt::init();
    println!("Hello, world!");
    run_server().await;
}
