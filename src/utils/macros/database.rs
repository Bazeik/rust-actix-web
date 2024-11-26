#[macro_export]
macro_rules! init_database {
    () => {{
        async {
            use crate::handle_cli;

            // Connect to the database
            let db = connect_database!().await;

            // Only handle CLI commands if arguments are provided
            if std::env::args().len() > 1 {
                handle_cli!(db.clone());
            }

            // Return the database connection for Actix Web server
            db
        }
        .await
    }};
}