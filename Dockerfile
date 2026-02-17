# Stage 1: Frontend build
FROM node:20-alpine AS frontend-builder
WORKDIR /app/frontend
# Copy package files for dependency caching
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# Stage 2: Rust backend build
FROM rust:bookworm AS backend-builder
WORKDIR /app

# Copy dependency manifests and create dummy source for caching
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --no-default-features --features web
RUN rm -rf src

# Copy actual source code and rebuild
COPY src/ ./src/
# Copy frontend build artifacts (for static file serving)
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist
RUN cargo build --release --no-default-features --features web

# Stage 3: Runtime (minimal image)
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies (OpenSSL, CA certificates)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Copy binary and static files
COPY --from=backend-builder /app/target/release/copal /app/copal
COPY --from=backend-builder /app/frontend/dist /app/frontend/dist

# Run as non-root user (security best practice)
RUN useradd -m appuser && chown -R appuser:appuser /app
USER appuser

# Set default PORT (can be overridden by container runtime)
ENV PORT=8080

CMD ["/app/copal"]
