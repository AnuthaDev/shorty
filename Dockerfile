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

# Install diesel_cli with only postgres feature in the builder stage
RUN cargo install diesel_cli --no-default-features --features postgres

# Runtime stage
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary and diesel_cli from builder stage
COPY --from=builder /app/target/release/shorty .
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
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
