pub struct DbConfig {
    pub db_type: String,           // Type of the database
    pub connection_string: String, // Connection string to the database
    //pub username: Option<String>, // Optional username for authentication
    //pub password: Option<String>, // Optional password for authentication
    pub database_name: String, // Name of the database to use
}

impl DbConfig {
    // Implement the new function to initialize DbConfig
    pub fn new(db_type: &str, connection_string: String, db_name: &str) -> Self {
        Self {
            db_type: db_type.to_string(),
            connection_string: connection_string,
            database_name: db_name.to_string(),
        }
    }
}
