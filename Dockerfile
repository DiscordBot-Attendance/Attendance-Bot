# Use the Rust official image as a base
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy Cargo files first (for caching dependencies)
COPY Cargo.toml Cargo.lock ./

# Create an empty project to cache dependencies
RUN mkdir src && echo 'fn main() {}' > src/main.rs

# Build the project in release mode
RUN cargo build --release

# Use a smaller base image for runtime
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/attendance-bot .

# Copy .env file (if needed)
COPY .env .env

# Expose the application's port
EXPOSE 8080

# Run the API server
CMD ["./attendance-bot"]

