# Database Migrations

This directory contains Flyway database migrations for MTG AI Suite.

## Prerequisites

- PostgreSQL 16+ with pgvector extension
- Flyway CLI or Docker

## Running Migrations

### Option 1: Docker (Recommended)

```bash
docker run --rm \
  -v $(pwd)/migrations:/flyway/sql \
  -e FLYWAY_URL=jdbc:postgresql://host.docker.internal:5432/mtg_ai_suite \
  -e FLYWAY_USER=postgres \
  -e FLYWAY_PASSWORD=your_password \
  flyway/flyway migrate
```

### Option 2: Flyway CLI

```bash
flyway -configFiles=flyway.conf migrate
```

### Option 3: Docker Compose

The main `docker-compose.yml` can run migrations automatically on startup.

## Migration Naming Convention

Flyway uses versioned migrations with this naming pattern:

```
V{version}__{description}.sql
```

Examples:
- `V001__initial_schema.sql`
- `V002__add_card_prices.sql`
- `V003__add_collection_tracking.sql`

## Current Schema

### Tables

| Table | Description |
|-------|-------------|
| `cards` | Scryfall card data cache |
| `card_embeddings` | Vector embeddings for semantic search |
| `captures` | Card recognition attempts |
| `users` | User accounts |
| `api_keys` | API authentication keys |
| `decks` | User deck lists |
| `deck_cards` | Cards in decks (many-to-many) |
| `game_sessions` | Game tracking |

### Extensions

- `uuid-ossp` - UUID generation
- `vector` - pgvector for embeddings
