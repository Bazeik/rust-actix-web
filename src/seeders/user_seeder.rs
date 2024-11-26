use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue::Set};
use crate::entities::prelude::{User, UserActiveModel};
use crate::utils::helpers::password_hash::hash_password;
use chrono::Utc;

/// Seeds the user table with initial data
pub async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // Clean the table before seeding
    println!("Cleaning user table...");
    User::delete_many().exec(db).await?;
    println!("Users table cleaned!");

    println!("Seeding user table...");

    // Current timestamp for created_at and updated_at
    let now = Utc::now().naive_utc();

    // Sample user data
    let users_data = vec![
        UserActiveModel {
            username: Set("user0".to_owned()),
            password: Set(hash_password("12345678")),
            email: Set("user0@example.com".to_owned()),
            full_name: Set("Administrator".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        },
        UserActiveModel {
            username: Set("user1".to_owned()),
            password: Set(hash_password("12345678")),
            email: Set("user1@example.com".to_owned()),
            full_name: Set("User One".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        },
        UserActiveModel {
            username: Set("user2".to_owned()),
            password: Set(hash_password("12345678")),
            email: Set("user2@example.com".to_owned()),
            full_name: Set("User Two".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        },
    ];

    // Insert users into the database
    User::insert_many(users_data).exec(db).await?;

    println!("Users table seeded successfully!");
    Ok(())
}