# Stage 1: Building the application
# Use the official Rust image to build the Rust application
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin gippitybot
WORKDIR /gippitybot

COPY . .

# Build for release
RUN rm ./target/release/deps/gippitybot*
RUN cargo build --release

# Set the startup command to run your binary
CMD ["./target/release/gippitybot"]

