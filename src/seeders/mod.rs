use sea_orm::{DatabaseConnection, DbErr};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub mod user_seeder;

type Seeder = fn(Arc<DatabaseConnection>) -> Pin<Box<dyn Future<Output = Result<(), DbErr>>>>;

pub fn get_seeders() -> HashMap<&'static str, Seeder> {
    let mut seeders: HashMap<&'static str, Seeder> = HashMap::new();
    seeders.insert("user", |db: Arc<DatabaseConnection>| {
        let db_clone = db.clone();
        Box::pin(async move { user_seeder::seed(db_clone.as_ref()).await })
    });
    seeders
}