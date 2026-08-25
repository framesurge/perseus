# Perseus Membership System

A complete user authentication and membership system built with Perseus, PostgreSQL, and integrated email verification.

## Features

- **User Registration** - Email and password registration with validation
- **Email Verification** - OTP-based email confirmation using Mailpit
- **Secure Login** - Session-based authentication
- **Password Hashing** - Argon2 (default) or bcrypt for password security
- **Password Reset** - Email-based password reset flow
- **Session Management** - Secure session tokens with verification

## Prerequisites

1. **PostgreSQL** - Database server
2. **Mailpit** - Local email testing (for development)
3. **Rust** - Latest stable version

## Setup

### 1. Install Mailpit

Mailpit is a local email testing tool that captures all outgoing emails.

```bash
# macOS
brew install mailpit

# Linux (binary)
curl -sL https://raw.githubusercontent.com/axllent/mailpit/develop/install.sh | bash

# Docker
docker run -d --name mailpit -p 1025:1025 -p 8025:8025 axllent/mailpit
```

### 2. Start Mailpit

```bash
mailpit
# SMTP: localhost:1025
# Web UI: http://localhost:8025
```

### 3. Setup PostgreSQL

```bash
# Create database
createdb membership

# Run schema (optional - tables are auto-created)
psql -d membership -f schema.sql
```

### 4. Configure Environment

Copy and modify the `.env` file:

```bash
cp .env.example .env
```

Edit `.env` with your PostgreSQL credentials:

```env
PG__HOST=127.0.0.1
PG__PORT=5432
PG__USER=your_user
PG__PASSWORD=your_password
PG__DBNAME=membership
```

### 5. Run the Application

```bash
# From the persauth directory
perseus serve
```

The application will be available at `http://localhost:8080`

## API Endpoints

All authentication endpoints are prefixed with `/auth`:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/auth/register` | POST | Register a new user |
| `/auth/login` | POST | Login user |
| `/auth/logout` | POST | Logout user |
| `/auth/confirm` | POST | Confirm OTP code |
| `/auth/resend-otp` | POST | Resend OTP code |
| `/auth/profile` | POST | Get user profile |
| `/auth/request-reset` | POST | Request password reset |
| `/auth/reset` | POST | Complete password reset |
| `/health` | GET | Health check |

## Project Structure

```
persauth/
├── src/
│   ├── main.rs              # Entry point and server setup
│   ├── lib.rs               # Library exports
│   ├── server/              # Server-side modules
│   │   ├── mod.rs
│   │   ├── configs.rs       # Configuration management
│   │   ├── errors.rs        # Error types
│   │   ├── mail.rs          # Email sending (Mailpit)
│   │   └── auth/
│   │       ├── mod.rs
│   │       ├── db.rs        # Database operations
│   │       ├── encryption.rs # Password hashing & encryption
│   │       ├── handlers.rs  # API route handlers
│   │       └── model.rs     # Data models
│   └── templates/           # Frontend pages
│       ├── mod.rs
│       ├── index.rs         # Home page
│       ├── login.rs         # Login page
│       └── register.rs      # Registration page
├── .env                     # Environment configuration
├── schema.sql              # Database schema
├── Cargo.toml
└── README.md
```

## Configuration Options

| Variable | Description | Default |
|----------|-------------|---------|
| `PG__HOST` | PostgreSQL host | 127.0.0.1 |
| `PG__PORT` | PostgreSQL port | 5432 |
| `PG__USER` | Database user | postgres |
| `PG__PASSWORD` | Database password | postgres |
| `PG__DBNAME` | Database name | membership |
| `SRV_CNF__HOST` | Server host | 127.0.0.1 |
| `SRV_CNF__PORT` | Server port | 8080 |
| `SRV_CNF__SECRET_KEY` | 32-byte hex key for encryption | (generated) |
| `SRV_CNF__BCRYPT_OR_ARGON` | Use bcrypt (true) or argon2 (false) | false |
| `SRV_CNF__EMAIL_OTP_ENABLED` | Enable OTP verification | true |
| `SRV_CNF__SMTP_HOST` | SMTP server host | 127.0.0.1 |
| `SRV_CNF__SMTP_PORT` | SMTP server port | 1025 |
| `SRV_CNF__SMTP_TLS_OFF` | Disable TLS (for mailpit) | true |

## Security Notes

- Passwords are hashed using Argon2 (or bcrypt)
- Session tokens use SHA-256 hashing
- OTP codes are encrypted with AES-256-GCM
- Constant-time comparison prevents timing attacks
- NFKC normalization applied to passwords

## Development

```bash
# Run in development mode
perseus serve

# Build for production
perseus build

# Check for compilation errors
cargo check
```

## Testing Email

1. Start Mailpit: `mailpit`
2. Open Mailpit UI: `http://localhost:8025`
3. Register a new user
4. Check Mailpit for the OTP email

## License

MIT
