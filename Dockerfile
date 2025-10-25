# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY diesel.toml ./

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install diesel_cli for running migrations
RUN apt-get update && \
    apt-get install -y wget && \
    wget -q https://github.com/diesel-rs/diesel/releases/download/v2.1.0/diesel_cli-v2.1.0-x86_64-unknown-linux-musl.tar.gz && \
    tar -xzf diesel_cli-v2.1.0-x86_64-unknown-linux-musl.tar.gz && \
    mv diesel /usr/local/bin/ && \
    rm diesel_cli-v2.1.0-x86_64-unknown-linux-musl.tar.gz && \
    apt-get remove -y wget && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/shorty .
COPY --from=builder /app/migrations ./migrations
COPY --from=builder /app/diesel.toml ./

# Create a startup script
RUN echo '#!/bin/bash\n\
set -e\n\
echo "Running database migrations..."\n\
diesel migration run\n\
echo "Starting application..."\n\
exec ./shorty' > /app/start.sh && chmod +x /app/start.sh

# Expose the application port
EXPOSE 8080

# Run the startup script
CMD ["/app/start.sh"]
