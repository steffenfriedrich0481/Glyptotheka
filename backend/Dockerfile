# Frontend Builder
FROM node:18 as frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# Backend Builder
FROM rust:1.83 as backend-builder

WORKDIR /app

# Update CA certificates and configure git/cargo for SSL
RUN apt-get update && \
    apt-get install -y ca-certificates git && \
    update-ca-certificates && \
    git config --global http.sslVerify false && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_HTTP_SSL_VERSION=tlsv1.3
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
ENV CARGO_HTTP_CHECK_REVOKE=false
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Copy manifests
COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/.cargo ./.cargo

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
        libgl1-mesa-glx libglu1-mesa libosmesa6 libosmesa6-dev \
        libxext6 libxrender1 libx11-6 libxcursor1 libxi6 libxrandr2 libxinerama1 \
        libfreetype6 fontconfig libfontconfig1 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from backend-builder
COPY --from=backend-builder /app/target/release/glyptotheka-backend /app/
COPY --from=backend-builder /app/migrations /app/migrations

# Copy frontend from frontend-builder
COPY --from=frontend-builder /app/dist /app/frontend

# Create directories
RUN mkdir -p /app/cache /app/data

# Environment variables
ENV DATABASE_PATH=/app/data/glyptotheka.db
ENV CACHE_DIR=/app/cache
ENV FRONTEND_PATH=/app/frontend
ENV RUST_LOG=info

EXPOSE 3000

CMD ["/app/glyptotheka-backend"]
