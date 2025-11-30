# Build stage
FROM rust:1.83-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy binary from builder
COPY --from=builder /app/target/release/zere /usr/local/bin/zere

# Create config directory
RUN mkdir -p /root/.config/zere

# Set working directory
WORKDIR /root

# Set entrypoint
ENTRYPOINT ["zere"]
CMD ["--help"]
