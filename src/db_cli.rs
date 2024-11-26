use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "POS Seeder CLI")]
#[command(about = "Manage database migrations and seeders", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run database migrations
    Migrate,
    /// Seed the database
    Seed {
        /// Specify the table to seed or use 'all' for all tables
        #[arg(short, long, default_value = "all")]
        table: String,
    },
}