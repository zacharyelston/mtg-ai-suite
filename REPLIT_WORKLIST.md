# Replit Agent Worklist

This document contains prioritized tasks for the Replit agent to implement.

## Current Branch

```bash
git checkout feature/replit-setup
```

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              FRONT                                       │
│                         (client/ directory)                              │
│  Next.js PWA • React • TailwindCSS • Camera/Image Capture               │
├─────────────────────────────────────────────────────────────────────────┤
│                               API                                        │
│                    (crates/mtg-server/src/api/)                         │
│  REST Endpoints • Request Validation • Response Formatting              │
├─────────────────────────────────────────────────────────────────────────┤
│                             SERVER                                       │
│                  (crates/mtg-server/src/services/)                      │
│  Business Logic • Card Recognition • Fuzzy Matching • AI Integration    │
├─────────────────────────────────────────────────────────────────────────┤
│                               DB                                         │
│                    (crates/mtg-server/src/db/)                          │
│  SQLite/PostgreSQL • Migrations • Card Data • User Data                 │
├─────────────────────────────────────────────────────────────────────────┤
│                             CONFIG                                       │
│                      (.env, config.rs, .replit)                         │
│  Environment Variables • Feature Flags • Connection Strings             │
└─────────────────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Directory | Responsibility |
|-------|-----------|----------------|
| **FRONT** | `client/` | UI components, camera access, API calls, offline caching |
| **API** | `crates/mtg-server/src/api/` | HTTP routes, request parsing, response JSON |
| **SERVER** | `crates/mtg-server/src/services/` | Core logic, card matching, image processing |
| **DB** | `crates/mtg-server/src/db/` | Database queries, migrations, data models |
| **CONFIG** | `.env`, `config.rs` | Environment config, secrets, feature toggles |

---

## Tasks by Layer

### 🔧 CONFIG Layer (Priority: HIGH)

| Task | Status | Description |
|------|--------|-------------|
| C1 | 🔴 TODO | Verify `.env` has `DATABASE_URL`, `PORT`, `RUST_LOG` |
| C2 | 🔴 TODO | Ensure `config.rs` loads all env vars with defaults |
| C3 | 🔴 TODO | Test SQLite connection string works |

**Files:**
- `.env` / `.env.example`
- `crates/mtg-server/src/config.rs`

---

### 🗄️ DB Layer (Priority: HIGH)

| Task | Status | Description |
|------|--------|-------------|
| D1 | 🔴 TODO | Run Flyway migrations (`database/migrations/`) |
| D2 | 🔴 TODO | Load Scryfall card data via `scripts/load_scryfall.py` |
| D3 | 🔴 TODO | Implement `CardRepository` with CRUD operations |
| D4 | 🔴 TODO | Implement `CaptureRepository` for image storage |

**Files:**
- `database/migrations/V001__initial_schema.sql`
- `crates/mtg-server/src/db/mod.rs`
- `crates/mtg-server/src/db/cards.rs`
- `crates/mtg-server/src/db/captures.rs`

**Commands:**
```bash
# Load card data
pip install psycopg2-binary requests
python scripts/load_scryfall.py --bulk-type oracle_cards
```

---

### ⚙️ SERVER Layer (Priority: HIGH)

| Task | Status | Description |
|------|--------|-------------|
| S1 | 🔴 TODO | Implement `CardService` with fuzzy search |
| S2 | 🔴 TODO | Implement `RecognitionService` for OCR pipeline |
| S3 | 🔴 TODO | Implement `HealthService` for system status |

**Files:**
- `crates/mtg-server/src/services/mod.rs`
- `crates/mtg-server/src/services/cards.rs`
- `crates/mtg-server/src/services/recognition.rs`

**Dependencies:**
- `mtg_core::fuzzy::FuzzyMatcher` for card name matching

---

### 🌐 API Layer (Priority: HIGH)

| Task | Status | Description |
|------|--------|-------------|
| A1 | 🔴 TODO | `GET /health` - Health check endpoint |
| A2 | 🔴 TODO | `GET /api/cards` - List/search cards |
| A3 | 🔴 TODO | `GET /api/cards/:id` - Get card by ID |
| A4 | 🔴 TODO | `GET /api/cards/autocomplete` - Card name autocomplete |
| A5 | 🔴 TODO | `POST /api/captures` - Upload image for recognition |
| A6 | 🔴 TODO | `GET /api/captures/:id` - Get recognition result |

**Files:**
- `crates/mtg-server/src/api/mod.rs`
- `crates/mtg-server/src/api/health.rs`
- `crates/mtg-server/src/api/cards.rs`
- `crates/mtg-server/src/api/captures.rs`

**Example Implementation (A1):**
```rust
// crates/mtg-server/src/api/health.rs
pub async fn health() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

---

### 📱 FRONT Layer (Priority: LOW)

| Task | Status | Description |
|------|--------|-------------|
| F1 | 🔴 TODO | Create Next.js app with TypeScript + Tailwind |
| F2 | 🔴 TODO | Card search page with autocomplete |
| F3 | 🔴 TODO | Card detail modal with image |
| F4 | 🔴 TODO | Camera capture component |
| F5 | 🔴 TODO | Recognition result display |

**Files:**
- `client/src/app/page.tsx`
- `client/src/components/CardSearch.tsx`
- `client/src/components/CardDetail.tsx`
- `client/src/components/CameraCapture.tsx`

**Setup:**
```bash
cd client
npx create-next-app@latest . --typescript --tailwind --app
```

---

## Implementation Order

```
1. CONFIG → 2. DB → 3. SERVER → 4. API → 5. FRONT
     │           │         │          │         │
     └───────────┴─────────┴──────────┴─────────┘
                    Dependencies flow left-to-right
```

**Recommended sequence:**
1. **C1-C3**: Get config working
2. **D1-D2**: Database + seed data
3. **S3 + A1**: Health endpoint (verify stack works)
4. **D3 + S1 + A2-A4**: Card search feature
5. **D4 + S2 + A5-A6**: Recognition feature
6. **F1-F5**: Frontend (can be parallel after API is stable)

---

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

# Test autocomplete
curl "http://localhost:8080/api/cards/autocomplete?q=light"
```

---

## Commit Guidelines

1. Create feature branch: `git checkout -b feature/your-feature`
2. Make small, focused commits
3. Run tests before committing: `cargo test`
4. Push and create PR to `develop`

---

## Notes

- **Dev DB**: SQLite (`sqlite:./data/dev.db`)
- **Prod DB**: PostgreSQL (set `DATABASE_URL`)
- **API Format**: All responses are JSON
- **Code Style**: Follow patterns in `crates/mtg-server/src/api/`
