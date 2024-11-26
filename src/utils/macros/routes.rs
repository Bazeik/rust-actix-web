#[macro_export]
macro_rules! configure_routes {
    ($app:expr, $db:expr, $($init_fn:path),*) => {{
        $app
            .app_data(actix_web::web::Data::new($db.clone()))
            $(
                .configure($init_fn)
            )*
    }};
}