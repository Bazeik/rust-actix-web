# Use the official Rust image
FROM rust:1.82.0

# Set the working directory
WORKDIR /app

# Install cargo-watch
RUN cargo install cargo-watch

# Copy the Cargo.toml and Cargo.lock first to leverage Docker caching
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy the rest of the application
COPY . .

# Expose the application's port
EXPOSE 8080

# Use cargo-watch to recompile and restart on code changes
CMD ["cargo", "watch", "-x", "run"]