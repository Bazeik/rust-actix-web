#[macro_export]
macro_rules! handle_cli {
    ($db:expr) => {{
        use clap::Parser;
        use crate::db_cli::{Cli, Commands};
        use sea_orm_migration::MigratorTrait;
        use crate::seeders::get_seeders;

        // Parse CLI arguments
        let cli = Cli::parse();

        match cli.command {
            Commands::Migrate => {
                migration::Migrator::up($db.as_ref(), None)
                    .await
                    .expect("Failed to run migrations");
                println!("Database migrations completed successfully!");
                std::process::exit(0);
            }
            Commands::Seed { table } => {
                let seeders = get_seeders();

                if table == "all" {
                    for (table_name, seeder) in &seeders {
                        println!("Seeding table: {}", table_name);
                        seeder($db.clone()).await.expect("Failed to seed table");
                    }
                    println!("All tables seeded successfully!");
                } else if let Some(seeder) = seeders.get(table.as_str()) {
                    println!("Seeding table: {}", table);
                    seeder($db.clone()).await.expect("Failed to seed table");
                } else {
                    eprintln!("Seeder not found for table: {}", table);
                }
                std::process::exit(0);
            }
        }
    }};
}