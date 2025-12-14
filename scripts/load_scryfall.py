#!/usr/bin/env python3
"""
Scryfall Bulk Data Loader for MTG AI Suite

Downloads and imports card data from Scryfall's bulk data API into PostgreSQL.

Usage:
    python scripts/load_scryfall.py [--database-url URL] [--bulk-type TYPE]

Bulk types:
    - default_cards: Unique cards (recommended, ~80MB)
    - all_cards: All printings (~400MB)
    - oracle_cards: One per Oracle ID (~30MB)
"""

import argparse
import json
import os
import sys
import time
from datetime import datetime
from typing import Optional
from urllib.request import urlopen, Request
from urllib.error import HTTPError

try:
    import psycopg2
    from psycopg2.extras import execute_values
except ImportError:
    print("Error: psycopg2 not installed. Run: pip install psycopg2-binary")
    sys.exit(1)

SCRYFALL_BULK_API = "https://api.scryfall.com/bulk-data"
USER_AGENT = "MTG-AI-Suite/1.0 (https://github.com/zacharyelston/mtg-ai-suite)"


def get_bulk_data_url(bulk_type: str = "default_cards") -> str:
    """Fetch the download URL for the specified bulk data type."""
    print(f"Fetching bulk data info for '{bulk_type}'...")

    req = Request(SCRYFALL_BULK_API, headers={"User-Agent": USER_AGENT})
    with urlopen(req) as response:
        data = json.loads(response.read().decode())

    for item in data.get("data", []):
        if item.get("type") == bulk_type:
            download_uri = item.get("download_uri")
            size_mb = item.get("size", 0) / (1024 * 1024)
            updated = item.get("updated_at", "unknown")
            print(f"  Found: {size_mb:.1f} MB, updated {updated}")
            return download_uri

    raise ValueError(f"Bulk data type '{bulk_type}' not found")


def download_bulk_data(url: str) -> list:
    """Download and parse the bulk data JSON."""
    print(f"Downloading bulk data...")
    start = time.time()

    req = Request(url, headers={"User-Agent": USER_AGENT})
    with urlopen(req) as response:
        data = json.loads(response.read().decode())

    elapsed = time.time() - start
    print(f"  Downloaded {len(data)} cards in {elapsed:.1f}s")
    return data


def transform_card(card: dict) -> Optional[tuple]:
    """Transform a Scryfall card to database row format."""
    # Skip tokens, emblems, etc.
    if card.get("layout") in ["token", "emblem", "art_series"]:
        return None

    # Skip digital-only cards
    if card.get("digital", False):
        return None

    return (
        card.get("id"),  # scryfall_id
        card.get("oracle_id"),
        card.get("name", "Unknown"),
        card.get("mana_cost"),
        card.get("cmc", 0),
        card.get("type_line", ""),
        card.get("oracle_text"),
        card.get("power"),
        card.get("toughness"),
        json.dumps(card.get("colors", [])),
        json.dumps(card.get("color_identity", [])),
        json.dumps(card.get("keywords", [])),
        card.get("set", ""),
        card.get("set_name", ""),
        card.get("rarity", "common"),
        json.dumps(card.get("image_uris", {})),
        json.dumps(card.get("legalities", {})),
    )


def load_to_database(cards: list, database_url: str, batch_size: int = 1000):
    """Load cards into PostgreSQL database."""
    print(f"Connecting to database...")

    conn = psycopg2.connect(database_url)
    cur = conn.cursor()

    # Transform cards
    print(f"Transforming {len(cards)} cards...")
    rows = []
    for card in cards:
        row = transform_card(card)
        if row:
            rows.append(row)

    print(f"  {len(rows)} cards after filtering")

    # Upsert in batches
    print(f"Upserting to database in batches of {batch_size}...")
    start = time.time()

    insert_sql = """
        INSERT INTO cards (
            scryfall_id, oracle_id, name, mana_cost, cmc, type_line,
            oracle_text, power, toughness, colors, color_identity,
            keywords, set_code, set_name, rarity, image_uris, legalities
        ) VALUES %s
        ON CONFLICT (scryfall_id) DO UPDATE SET
            oracle_id = EXCLUDED.oracle_id,
            name = EXCLUDED.name,
            mana_cost = EXCLUDED.mana_cost,
            cmc = EXCLUDED.cmc,
            type_line = EXCLUDED.type_line,
            oracle_text = EXCLUDED.oracle_text,
            power = EXCLUDED.power,
            toughness = EXCLUDED.toughness,
            colors = EXCLUDED.colors,
            color_identity = EXCLUDED.color_identity,
            keywords = EXCLUDED.keywords,
            set_code = EXCLUDED.set_code,
            set_name = EXCLUDED.set_name,
            rarity = EXCLUDED.rarity,
            image_uris = EXCLUDED.image_uris,
            legalities = EXCLUDED.legalities,
            updated_at = NOW()
    """

    total_inserted = 0
    for i in range(0, len(rows), batch_size):
        batch = rows[i : i + batch_size]
        execute_values(cur, insert_sql, batch)
        total_inserted += len(batch)
        print(f"  Processed {total_inserted}/{len(rows)} cards...")

    conn.commit()
    cur.close()
    conn.close()

    elapsed = time.time() - start
    print(f"  Completed in {elapsed:.1f}s ({len(rows)/elapsed:.0f} cards/sec)")


def main():
    parser = argparse.ArgumentParser(description="Load Scryfall bulk data into PostgreSQL")
    parser.add_argument(
        "--database-url",
        default=os.environ.get("DATABASE_URL"),
        help="PostgreSQL connection URL (default: $DATABASE_URL)",
    )
    parser.add_argument(
        "--bulk-type",
        default="default_cards",
        choices=["default_cards", "all_cards", "oracle_cards"],
        help="Scryfall bulk data type (default: default_cards)",
    )
    parser.add_argument(
        "--batch-size",
        type=int,
        default=1000,
        help="Database insert batch size (default: 1000)",
    )

    args = parser.parse_args()

    if not args.database_url:
        print("Error: DATABASE_URL not set. Use --database-url or set environment variable.")
        sys.exit(1)

    print("=" * 60)
    print("Scryfall Bulk Data Loader")
    print("=" * 60)
    print(f"Bulk type: {args.bulk_type}")
    print(f"Database: {args.database_url.split('@')[-1]}")  # Hide credentials
    print()

    try:
        # Get download URL
        download_url = get_bulk_data_url(args.bulk_type)

        # Download data
        cards = download_bulk_data(download_url)

        # Load to database
        load_to_database(cards, args.database_url, args.batch_size)

        print()
        print("=" * 60)
        print("SUCCESS: Card data loaded!")
        print("=" * 60)

    except Exception as e:
        print(f"\nERROR: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
