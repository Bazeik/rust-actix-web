mod services;
mod db_cli;
mod seeders;
mod entities;
use dotenv::dotenv;


#[path = "../migration/src/lib.rs"]
mod migration;

mod utils;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from `.env` file
    dotenv().ok();
    // Initialize the database
    let db = init_database!();
    // Start the Actix Web server
    HttpServer::new(move || {
        add_health_check!(
            App::new()
                .app_data(web::Data::new(db.clone()))
                .configure(services::users::init_routes),
            "/"
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}