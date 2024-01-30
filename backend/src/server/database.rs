use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct Database {
    connection: DatabaseConnection,
}
impl Database {
    pub fn new() -> Self {
        Self {
            connection: Database::connect(""),
        }
    }
}
impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}
