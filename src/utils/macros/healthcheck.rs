#[macro_export]
macro_rules! add_health_check {
    ($app:expr, $path:expr) => {
        $app.route($path, actix_web::web::get().to(|| async {
            actix_web::HttpResponse::Ok().body("Server is up and running!")
        }))
    };
}