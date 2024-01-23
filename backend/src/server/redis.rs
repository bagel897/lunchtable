use redis::Client;
use tracing::info;

pub struct Cache {
    client: Client,
}
impl Cache {
    pub fn new() -> Self {
        info!("Connecting to Redis");
        Self {
            client: Client::open("redis://127.0.0.1/").unwrap(),
        }
    }
}
