pub mod handlers;
pub mod model;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::get().to(handlers::list_users))
            .route("/", web::post().to(handlers::create_user)),
    );
}