# latest version of Rust as of Sep 2023
FROM rust:1.72

WORKDIR /simple_restaurant_api

# Copy the contents of your Rust project to the container
COPY ./simple_restaurant_api .

# Build the Rust project
RUN cargo build --release
