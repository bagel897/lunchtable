use super::redis::Cache;

pub struct Database {
    cache: Cache,
}
impl Database {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
        }
    }
}
