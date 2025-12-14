# Scripts

Utility scripts for MTG AI Suite.

## Prerequisites

```bash
pip install psycopg2-binary
```

## Scripts

### load_scryfall.py

Downloads and imports card data from Scryfall's bulk data API.

```bash
# Using environment variable
export DATABASE_URL=postgres://postgres:password@localhost:5432/mtg_ai_suite
python scripts/load_scryfall.py

# Or with explicit URL
python scripts/load_scryfall.py --database-url "postgres://..."

# Different bulk data types
python scripts/load_scryfall.py --bulk-type oracle_cards   # Smallest (~30MB)
python scripts/load_scryfall.py --bulk-type default_cards  # Recommended (~80MB)
python scripts/load_scryfall.py --bulk-type all_cards      # All printings (~400MB)
```

**Bulk Data Types:**

| Type | Size | Description |
|------|------|-------------|
| `oracle_cards` | ~30MB | One card per Oracle ID (unique game objects) |
| `default_cards` | ~80MB | One card per unique name+set (recommended) |
| `all_cards` | ~400MB | Every printing including promos, variants |

**Note:** Scryfall rate limits to 10 requests/second. The bulk data endpoint is exempt but be respectful.
