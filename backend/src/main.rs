use crate::server::run_server;
use tracing::info;

mod api;
mod core;
mod server;
#[rocket::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("BRUH");
    println!("Hello, world!");
    run_server().await;
}

fn demo_ownership() {
    let i = 0;
    let j = &i;
    let m = &i;
    let k = &i;
    let l = &i;
    return i;
}
