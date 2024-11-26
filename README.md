# Rust Web Service

A web service built with Actix-Web framework and database integration.

## Features

- RESTful API endpoints
- Database integration
- Environment configuration support
- Health check endpoint
- User management routes

## Project Structure

The project is organized into several modules and directories, each serving a specific purpose:

```bash

rust-actix-web/
├── Dockerfile                  # Defines the Docker image for the application
├── docker-compose.yml          # Configures multi-service Docker setup
├── .env                        # Environment variables for the application
├── README.md                   # Project documentation and instructions
├── src/                        # Main source code directory
│   ├── main.rs                 # Entry point for the application
│   ├── db_cli.rs               # CLI command definitions using the clap library
│   ├── utils/                  # Utility modules
│   │   ├── macros/             # Macro definitions used across the app
│   │   ├── handle_cli.rs       # Macro for handling CLI commands
│   │   └── mod.rs              # Re-exports submodules in the utils directory
│   ├── services/               # Service modules for the application
│   │   └── users.rs            # Example: User-related services
│   └── entities/               # Entity definitions for database models
│       └── prelude.rs          # Re-exports common entity traits and types
├── migration/                  # Database migration-related code
│   ├── src/                    # Migration source code
│   │   ├── main.rs             # Entry point for running migrations
│   │   └── migrations.rs       # Migration logic and definitions
│   └── Cargo.toml              # Dependencies for the migration system
└── seeders/                    # Seed data for the database
    ├── user_seeder.rs          # Seeds the user table with initial data
    └── mod.rs                  # Seeder module to combine and manage seeders
```

This modular structure allows for a clean separation of concerns, making the application easier to maintain and extend. Each module has a specific responsibility, such as handling CLI commands, managing database migrations, or providing utility functions.

## Prerequisites

- Rust (latest stable version)
- Cargo package manager
- Database (configuration required in `.env` file)

## Getting Started

### 1. Clone the repository 

```bash
git clone https://github.com/yourusername/yourproject.git
```


### 2. Set Up Environment Variables

Create a `.env` file in the root directory of the project and configure your database connection and other environment variables:

```
RUST_BACKTRACE=1
# PostgreSQL Credentials
POSTGRES_USER=          # PostgreSQL username
POSTGRES_PASSWORD=      # PostgreSQL password
POSTGRES_DB=            # PostgreSQL database name

# Application Configuration
DB_HOST=                # Hostname for the database (matches the service name in Docker Compose)
DB_PORT=                # PostgreSQL port
DATABASE_URL=           # Database connection string
```


### 3. Build and Run with Docker Compose

Ensure Docker and Docker Compose are installed and running on your machine. Then, build and start the services:

`docker-compose up --build`

This command will build the Docker images and start the containers as defined in the `docker-compose.yml` file.



This command will build the Docker images and start the containers as defined in the `docker-compose.yml` file.

### 4. Running Custom CLI Commands

We created a cutom migration tool. To run CLI commands for database migrations or seeding, use the following:

- **Migrate the Database:**

  ```bash
  make migrate
  ```

- **Seed the Database:**

  ```bash
  make seed-all
  ```

- **Seed a specific Table:**

  ```bash
  make seed TABLE=table_name
  ```

- **Create SeaORM entities:**

  ```bash
  make generate-entities
  ```

### 5. Accessing the Application

Once the services are running, you can access the web service at to check `http://localhost:8080/`.

### 6. Stopping the Services

To stop the running services, use:

``docker-compose down``


## Development

For development, the Docker setup uses `cargo-watch` to automatically recompile and restart the application when code changes are detected. This is useful for rapid development and testing.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
