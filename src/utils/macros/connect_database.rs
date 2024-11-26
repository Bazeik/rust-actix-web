#[macro_export]
macro_rules! connect_database {
    () => {{
        async {
            use sea_orm::Database;
            use std::sync::Arc;

            // Get database credentials from the environment
            let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
            let db_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
            let db_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
            let db_host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
            let db_port = std::env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());

            // Construct the database URL
            let database_url = format!(
                "postgres://{}:{}@{}:{}/{}",
                db_user, db_password, db_host, db_port, db_name
            );

            // Connect to the database
            let db = Database::connect(&database_url)
                .await
                .expect("Failed to connect to the database");

            // Wrap the database connection in Arc and return
            Arc::new(db)
        }
    }};
}