# Multi-stage Dockerfile for Yoshi Bot
# Stage 1: Build the Rust application
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release --features all-platforms && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the real application
# Touch main.rs to force rebuild
RUN touch src/main.rs && \
    cargo build --release --features all-platforms

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1000 yoshi && \
    mkdir -p /app && \
    chown -R yoshi:yoshi /app

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/yoshi_bot /app/yoshi_bot

# Copy example config
COPY config.example.toml /app/config.example.toml

# Change ownership
RUN chown -R yoshi:yoshi /app

# Switch to non-root user
USER yoshi

# Health check (adjust port if needed)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -x yoshi_bot || exit 1

# Run the bot
CMD ["/app/yoshi_bot"]
