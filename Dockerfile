# Use Rust base image
FROM rust:1.75-bullseye as builder

# Install Protocol Buffers compiler
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy proto files (needed for build)
COPY proto/ proto/

# Copy source code
COPY src/ src/

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder stage
COPY --from=builder /app/target/release/crypto-wallet-api /usr/local/bin/crypto-wallet-api

# Create app user
RUN useradd -r -s /bin/false appuser
USER appuser

# Expose port
EXPOSE 8080

# Set environment variables for Railway
ENV HOST=0.0.0.0
ENV PORT=8080
ENV GRPC_PORT=9090

# Run the application
CMD ["crypto-wallet-api"]