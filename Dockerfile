# Stage 1: Building the application
# Use the official Rust image to build the Rust application
FROM rust:latest as builder

# Create a new empty shell project
WORKDIR /gippitybot

COPY . .

RUN cargo build --release

# Set the startup command to run your binary
CMD ["./target/release/gippitybot"]

