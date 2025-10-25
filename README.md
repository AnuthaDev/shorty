# Shorty - URL Shortener

A fast, simple URL shortener service built with Rust, actix-web, Diesel, and PostgreSQL.

## Features

- 🚀 Fast and lightweight Rust-based service
- 🔗 Shorten any HTTP/HTTPS URL
- 🎨 Modern, responsive web interface
- ✅ URL validation (ensures only valid URLs are shortened)
- 🐳 Docker-ready with docker-compose support
- 📦 Single binary serving both API and web interface
- 🔄 Automatic database migrations

## Tech Stack

- **Rust** - Systems programming language
- **actix-web** - Web framework
- **Diesel** - ORM for PostgreSQL
- **PostgreSQL** - Database
- **Docker** - Containerization

## Quick Start with Docker

The easiest way to run Shorty is using Docker Compose:

```bash
# Clone the repository
git clone <your-repo-url>
cd shorty

# Start the services
docker-compose up -d

# View logs
docker-compose logs -f app
```

The application will be available at `http://localhost:8080`

To stop the services:

```bash
docker-compose down
```

To stop and remove all data:

```bash
docker-compose down -v
```

## Local Development Setup

### Prerequisites

- Rust 1.75 or later
- PostgreSQL 12 or later
- Diesel CLI

### Install Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Setup

1. **Clone the repository**

```bash
git clone <your-repo-url>
cd shorty
```

2. **Configure environment variables**

Copy the example environment file:

```bash
cp .env.example .env
```

Edit `.env` and update the database connection:

```env
DATABASE_URL=postgres://shorty:shorty@localhost/shorty
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
BASE_URL=http://localhost:8080
RUST_LOG=info
```

3. **Setup PostgreSQL**

Create the database:

```bash
createdb shorty
# Or using psql:
psql -c "CREATE DATABASE shorty;"
```

4. **Run migrations**

```bash
diesel migration run
```

5. **Build and run**

```bash
cargo build --release
cargo run --release
```

The application will start at `http://localhost:8080`

## API Documentation

### Endpoints

#### 1. Homepage
```
GET /
```
Returns the web interface for URL shortening.

#### 2. Shorten URL
```
POST /api/shorten
Content-Type: application/json

{
  "url": "https://example.com/very/long/url"
}
```

**Success Response (200):**
```json
{
  "short_url": "http://localhost:8080/abc123",
  "short_code": "abc123"
}
```

**Error Response (400):**
```json
{
  "error": "Invalid URL: must be a valid http/https URL"
}
```

#### 3. Redirect
```
GET /{short_code}
```

Redirects to the original URL (HTTP 302).

**Error Response (404):**
```
Short URL not found
```

### Example Usage with curl

Shorten a URL:
```bash
curl -X POST http://localhost:8080/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.rust-lang.org/"}'
```

Test redirect:
```bash
curl -L http://localhost:8080/abc123
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | Required |
| `SERVER_HOST` | Server bind address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `8080` |
| `BASE_URL` | Base URL for generating short links | `http://localhost:8080` |
| `RUST_LOG` | Logging level | `info` |

## Project Structure

```
shorty/
├── src/
│   ├── main.rs           # Application entry point
│   ├── db.rs             # Database connection pool
│   ├── models.rs         # Diesel models
│   ├── schema.rs         # Database schema (auto-generated)
│   ├── handlers.rs       # HTTP request handlers
│   ├── routes.rs         # Route configuration
│   └── utils.rs          # Utility functions (URL validation, code generation)
├── migrations/           # Database migrations
├── Cargo.toml           # Rust dependencies
├── Dockerfile           # Docker image definition
├── docker-compose.yml   # Docker Compose configuration
└── .env.example         # Example environment variables
```

## Development

### Running tests

```bash
cargo test
```

### Checking code

```bash
cargo clippy
cargo fmt --check
```

### Creating a new migration

```bash
diesel migration generate <migration_name>
```

## Deployment

### Production Considerations

1. **Change default credentials**: Update PostgreSQL username and password in production
2. **Use environment variables**: Never commit `.env` to version control
3. **Set BASE_URL**: Update to your production domain
4. **Enable HTTPS**: Use a reverse proxy (nginx, Caddy) for SSL/TLS
5. **Database backups**: Regular PostgreSQL backups
6. **Monitoring**: Set up logging and monitoring

### Deploy with Docker

For production deployment, update environment variables in `docker-compose.yml`:

```yaml
environment:
  DATABASE_URL: postgres://user:password@db:5432/shorty
  SERVER_HOST: 0.0.0.0
  SERVER_PORT: 8080
  BASE_URL: https://your-domain.com
  RUST_LOG: warn
```

Then deploy:

```bash
docker-compose up -d
```

### Reverse Proxy Example (nginx)

```nginx
server {
    listen 80;
    server_name yourdomain.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
