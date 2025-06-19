# Build stage
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create app directory
WORKDIR /usr/src/app

# Copy manifest files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application
RUN touch src/main.rs && \
    cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates libgcc

# Create a non-root user to run the app
RUN addgroup -g 1000 app && \
    adduser -D -u 1000 -G app app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/crypto-wallet-api /usr/local/bin/crypto-wallet-api

# Set ownership
RUN chown app:app /usr/local/bin/crypto-wallet-api

# Switch to non-root user
USER app

# Expose port
EXPOSE 8080

# Set environment variables
ENV APP_HOST=0.0.0.0
ENV APP_PORT=8080
ENV APP_LOG_LEVEL=info

# Run the binary
CMD ["crypto-wallet-api"]