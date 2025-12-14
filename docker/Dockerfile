# MTG AI Suite - Production Dockerfile
# Multi-stage build for minimal image size

# ============================================
# Stage 1: Build Rust binaries
# ============================================
FROM rust:1.75-bookworm AS rust-builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binaries
RUN cargo build --release --bin mtg-server --bin mtg-ai-suite

# ============================================
# Stage 2: Build TypeScript frontend
# ============================================
FROM node:20-bookworm AS node-builder

WORKDIR /app/client

# Copy package files
COPY client/package*.json ./

# Install dependencies
RUN npm ci

# Copy source
COPY client ./

# Build
RUN npm run build

# ============================================
# Stage 3: Production image
# ============================================
FROM debian:bookworm-slim AS production

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 mtg

# Copy binaries from rust builder
COPY --from=rust-builder /app/target/release/mtg-server /usr/local/bin/
COPY --from=rust-builder /app/target/release/mtg-ai-suite /usr/local/bin/

# Copy frontend build
COPY --from=node-builder /app/client/.next ./client/.next
COPY --from=node-builder /app/client/public ./client/public
COPY --from=node-builder /app/client/package.json ./client/

# Set ownership
RUN chown -R mtg:mtg /app

# Switch to non-root user
USER mtg

# Expose ports
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["mtg-server"]
