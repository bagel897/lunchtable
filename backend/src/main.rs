use tracing::info;

mod api;
mod core;
mod server;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("BRUH");
    println!("Hello, world!");
}
