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

- **`src/`**: The main source directory for the Rust application.
  - **`main.rs`**: The entry point of the application, responsible for initializing and running the web service.
  - **`db_cli.rs`**: Contains the command-line interface (CLI) definitions using the `clap` library. It includes the `Cli` struct and `Commands` enum to parse and handle CLI arguments.
  - **`utils/`**: A directory containing utility modules.
    - **`macros/`**: Contains macro definitions used throughout the application.
      - **`handle_cli.rs`**: Defines a macro to handle CLI commands, such as database migrations and seeding.
    - **`mod.rs`**: The module file that re-exports the submodules within the `utils` directory.

- **`migration/`**: Contains migration-related code and scripts.
  - **`src/`**: Contains Rust code for managing database migrations.
    - **`main.rs`**: The entry point for running migrations, typically using the `sea_orm_migration` library.

- **`Dockerfile`**: Defines the Docker image for the Rust application, including the build process and runtime configuration.

- **`docker-compose.yml`**: Configures multiple services using Docker Compose, such as the Rust application (`rust_backend`), a PostgreSQL database (`db`), and possibly a CLI tool (`sea-orm-cli`).

- **`.env`**: A file containing environment variables for configuring the application, such as database credentials and connection strings.

- **`README.md`**: Provides documentation and instructions for setting up and running the application.

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
