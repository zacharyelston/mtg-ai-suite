# Replit Setup Guide for MTG AI Suite

This document provides step-by-step instructions for setting up and running MTG AI Suite in Replit.

## Prerequisites

- Replit account with access to the workspace
- GitHub repository access

## Initial Setup

### Step 1: Sync with GitHub

```bash
# Reset to match the remote branch
git fetch origin
git reset --hard origin/develop

# Verify you're on the correct branch
git branch
```

### Step 2: Install Rust

Replit should auto-detect Rust from the `Cargo.toml`. If not:

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 3: Install System Dependencies

```bash
# For SQLite (used in development)
apt-get update && apt-get install -y sqlite3 libsqlite3-dev

# For PostgreSQL client (optional, for production)
apt-get install -y libpq-dev
```

### Step 4: Build the Project

```bash
# Build all crates
cargo build

# Run tests to verify
cargo test
```

## Running the Server

### Option A: Development Mode (SQLite)

```bash
# Set environment variables
export DATABASE_URL="sqlite:./data/dev.db"
export RUST_LOG=info
export PORT=8080

# Create data directory
mkdir -p data

# Run the server
cargo run --bin mtg-server
```

### Option B: With PostgreSQL

If you have a PostgreSQL database (e.g., Neon, Supabase):

```bash
export DATABASE_URL="postgres://user:pass@host:5432/dbname"
export RUST_LOG=info
export PORT=8080

cargo run --bin mtg-server
```

## Replit Configuration Files

### .replit

```toml
run = "cargo run --bin mtg-server"
entrypoint = "crates/mtg-server/src/main.rs"

[env]
RUST_LOG = "info"
PORT = "8080"

[nix]
channel = "stable-24_05"

[deployment]
run = ["sh", "-c", "cargo run --release --bin mtg-server"]
```

### replit.nix

```nix
{ pkgs }: {
  deps = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rust-analyzer
    pkgs.sqlite
    pkgs.openssl
    pkgs.pkg-config
  ];
}
```

## Development Workflow

### 1. Make Changes

Edit files in the Replit IDE or sync from GitHub.

### 2. Build and Test

```bash
cargo build
cargo test
```

### 3. Run Locally

```bash
cargo run --bin mtg-server
```

### 4. Commit and Push

```bash
git add -A
git commit -m "Your commit message"
git push origin feature/your-branch
```

## Loading Card Data

Once the server is running with a database:

```bash
# Install Python dependency
pip install psycopg2-binary

# Load Scryfall data
python scripts/load_scryfall.py --bulk-type oracle_cards
```

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build
```

### Database Connection Issues

```bash
# Test SQLite
sqlite3 data/dev.db ".tables"

# Test PostgreSQL
psql $DATABASE_URL -c "SELECT 1"
```

### Port Already in Use

```bash
# Find and kill process on port 8080
lsof -i :8080
kill -9 <PID>
```

## Environment Variables Reference

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | Database connection string |
| `PORT` | No | 8080 | Server port |
| `RUST_LOG` | No | info | Log level (debug, info, warn, error) |
| `REDIS_URL` | No | - | Redis connection (optional) |
| `LLM_API_KEY` | No | - | OpenAI/Anthropic API key |
| `LLM_PROVIDER` | No | openai | LLM provider (openai, anthropic, ollama) |

## Next Steps

1. ✅ Build and run the server
2. ⬜ Load card data from Scryfall
3. ⬜ Test API endpoints
4. ⬜ Set up frontend (when client/ is added)
