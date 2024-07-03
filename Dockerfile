# Use the official Rust image as the build environment
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo install --path .

# Use a smaller Debian-based image for the runtime environment
FROM debian:bullseye-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/rust-app /usr/local/bin/rust-app

# Define the command to run the application
CMD ["rust-app"]
