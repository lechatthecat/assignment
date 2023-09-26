# latest version of Rust as of Sep 2023
FROM rust:1.72

RUN mkdir -p /var/log/actix && chmod -R 775 /var/log/actix

WORKDIR /simple_restaurant_api

# Copy the contents of your Rust project to the container
COPY ./simple_restaurant_api .

WORKDIR /frontend
COPY ./frontend .

# Install Node.js 18 and npm
RUN apt-get update && \
    apt-get install -y curl && \
    curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN npm install && npm run build

WORKDIR /simple_restaurant_api

# Build the Rust project
RUN cargo build --release
