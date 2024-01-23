#[derive(Clone)]
pub struct Database {}
impl Database {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
