# Start from the latest Rust image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/hello_world

# Copy the source code into the container
COPY . .


# Build the Rust project
RUN cargo build --release

# Set the start command to run the compiled binary
CMD ["./target/release/hello_world"]

