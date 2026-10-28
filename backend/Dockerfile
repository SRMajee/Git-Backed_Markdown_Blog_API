# ==========================================
# Stage 1: Builder
# ==========================================
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Install pkg-config and libssl if needed
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy src/main.rs to cache dependency compilation
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Update touch timestamps to force cargo to build the real binary
RUN touch src/main.rs src/lib.rs && cargo build --release

# ==========================================
# Stage 2: Runtime
# ==========================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from builder
COPY --from=builder /app/target/release/git-blog-api /usr/local/bin/git-blog-api

# Default configuration
ENV CONTENT_DIR=/app/content/posts
ENV BIND_ADDR=0.0.0.0:3000
ENV RUST_LOG=git_blog_api=info,tower_http=info

EXPOSE 3000

CMD ["git-blog-api"]
