# Backend Dockerfile
FROM rust:1.83 as builder

WORKDIR /app

# Copy manifests
COPY backend/Cargo.toml backend/Cargo.lock ./

# Build dependencies (cached layer)
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY backend/src ./src
COPY backend/migrations ./migrations

# Build application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libsqlite3-0 ca-certificates \
        libgl1-mesa-glx libglu1-mesa libosmesa6 libxext6 libxrender1 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/glyptotheka-backend /app/
COPY --from=builder /app/migrations /app/migrations

# Create directories
RUN mkdir -p /app/cache /app/data

# Environment variables
ENV DATABASE_PATH=/app/data/glyptotheka.db
ENV CACHE_DIR=/app/cache
ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/glyptotheka-backend"]
