# Replit Agent Worklist

This document contains prioritized tasks for the Replit agent to implement.

## Current Branch

```bash
git checkout feature/replit-setup
```

## Phase 1: Core Server (Priority: HIGH)

### Task 1.1: Verify Server Builds and Runs
- [ ] Run `cargo build` - should complete without errors
- [ ] Run `cargo test --all` - all tests should pass
- [ ] Run `cargo run --bin mtg-server` - server starts on port 8080
- [ ] Verify `/health` endpoint returns 200 OK

### Task 1.2: Implement Health Endpoint
**File:** `crates/mtg-server/src/api/health.rs`

```rust
pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

### Task 1.3: Database Connection
- [ ] Verify SQLite connection works with `DATABASE_URL=sqlite:./data/dev.db`
- [ ] Run migrations (create tables if they don't exist)
- [ ] Test basic CRUD operations

## Phase 2: Card Data (Priority: HIGH)

### Task 2.1: Load Scryfall Data
```bash
pip install psycopg2-binary
python scripts/load_scryfall.py --bulk-type oracle_cards
```

### Task 2.2: Implement Card Search API
**Endpoint:** `GET /api/cards?q=lightning`

**File:** `crates/mtg-server/src/api/cards.rs`

- [ ] Implement `list_cards` handler
- [ ] Add fuzzy search using `mtg_core::fuzzy::FuzzyMatcher`
- [ ] Return JSON with card data

### Task 2.3: Implement Card Autocomplete
**Endpoint:** `GET /api/cards/autocomplete?q=light`

- [ ] Return top 10 matching card names
- [ ] Use fuzzy matching with high confidence threshold

## Phase 3: Recognition API (Priority: MEDIUM)

### Task 3.1: Implement Image Upload
**Endpoint:** `POST /api/captures`

- [ ] Accept base64-encoded image
- [ ] Store in `data/captures/` directory
- [ ] Return capture ID

### Task 3.2: Implement OCR Pipeline
- [ ] Extract text from image (placeholder for now)
- [ ] Use fuzzy matcher to find card name
- [ ] Return recognition result with confidence

## Phase 4: Frontend (Priority: LOW)

### Task 4.1: Create Next.js App
```bash
cd client
npx create-next-app@latest . --typescript --tailwind --app
```

### Task 4.2: Implement Card Browser
- [ ] Search input with autocomplete
- [ ] Card grid with images
- [ ] Card detail modal

### Task 4.3: Implement Camera Capture
- [ ] WebRTC camera access
- [ ] Capture button
- [ ] Send to recognition API

## API Endpoints Summary

| Method | Endpoint | Status | Description |
|--------|----------|--------|-------------|
| GET | `/health` | 🔴 TODO | Health check |
| GET | `/api/cards` | 🔴 TODO | List/search cards |
| GET | `/api/cards/:id` | 🔴 TODO | Get card by ID |
| GET | `/api/cards/autocomplete` | 🔴 TODO | Autocomplete |
| POST | `/api/captures` | 🔴 TODO | Upload image |
| GET | `/api/captures/:id` | 🔴 TODO | Get capture result |

## Testing Commands

```bash
# Build
cargo build

# Test
cargo test --all

# Run server
cargo run --bin mtg-server

# Test health endpoint
curl http://localhost:8080/health

# Test card search
curl "http://localhost:8080/api/cards?q=lightning"
```

## Commit Guidelines

1. Create feature branch: `git checkout -b feature/your-feature`
2. Make small, focused commits
3. Run tests before committing: `cargo test`
4. Push and create PR to `develop`

## Notes

- Use SQLite for development (`sqlite:./data/dev.db`)
- PostgreSQL for production (set `DATABASE_URL`)
- All API responses should be JSON
- Follow existing code patterns in `crates/mtg-server/src/api/`
