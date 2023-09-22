# latest version of Rust as of Sep 2023
FROM rust:1.72

WORKDIR /simple_restaurant_api

# Copy the contents of your Rust project to the container
COPY ./simple_restaurant_api .

# Build the Rust project
# TODO should be uncommented for prod
# RUN cargo build --release

# Command to run the compiled Rust program
# TODO should be changed to "release"
#CMD ["/simple_restaurant_api/target/release/simple_restaurant_api"]
CMD ["/simple_restaurant_api/target/debug/simple_restaurant_api"]
