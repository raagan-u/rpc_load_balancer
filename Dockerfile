# Use Rust official image as builder
FROM rust:1.75-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    cmake \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty project
WORKDIR /app

# Create the project structure
RUN cargo new --bin lb
WORKDIR /app/lb

# Copy only Cargo.toml and Cargo.lock first (if you have a Cargo.lock)
COPY Cargo.toml ./

# Create a dummy main.rs that will ensure dependencies are cached
RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# Now copy your actual source code
COPY src/lb.rs src/main.rs

# Build the release binary
RUN cargo build --release

# Create the runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/lb/target/release/lb /usr/local/bin/

# Expose the port your load balancer listens on
EXPOSE 8080

# Run the binary
CMD ["lb"]