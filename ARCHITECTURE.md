# MTG AI Suite - Architecture & Design Standards

## Overview

This document defines the architecture, design standards, and conventions for the MTG AI Suite. It serves as the authoritative reference for AI agents and developers building this system.

---

## ⚠️ FIRST PRINCIPLES - READ THIS FIRST

These rules take precedence over all other guidelines in this document.

### 1. Modularity Above All

```
RULE: Every file must have a single, clear responsibility.
      Use the filesystem as your organizational structure.
      If you're unsure where code belongs, create a new module.
```

### 2. File Size Limit

```
RULE: Maximum 500 lines per source file.
      If a file exceeds 500 lines, split it into smaller modules.
      No exceptions.
```

**Why?**
- Easier to understand, review, and test
- Faster compilation (Rust)
- Better git diffs and merge conflict resolution
- Forces good separation of concerns

**How to split:**
```
# Before (bad): api.rs with 800 lines
api.rs

# After (good): api/ directory with focused modules
api/
├── mod.rs          # Re-exports, max 50 lines
├── captures.rs     # Capture endpoints
├── cards.rs        # Card endpoints
├── auth.rs         # Auth endpoints
└── middleware.rs   # Shared middleware
```

### 3. Reduce Complexity

```
RULE: Prefer simple, readable code over clever code.
      If a function is hard to explain, refactor it.
      Cyclomatic complexity should stay low.
```

**Guidelines:**
- Functions: max 50 lines, ideally under 25
- Nesting: max 3 levels deep
- Parameters: max 5, use structs for more
- Dependencies: minimize coupling between modules

### 4. Commit and Test Often

```
RULE: Commit frequently with atomic, logical changes.
      Run tests before every commit.
      Each commit should leave the codebase in a working state.
```

**Commit workflow:**
```bash
# 1. Make small, focused change
# 2. Run tests
cargo test
npm test

# 3. Commit with descriptive message
git commit -m "feat(recognition): add fuzzy matching for card names"

# 4. Repeat
```

### 5. Push Only When It Works

```
RULE: Never push broken code to shared branches.
      All tests must pass before pushing.
      CI should never be red on main/develop.
```

**Push workflow:**
```bash
# 1. Run full test suite
cargo test --all
cd client && npm test && npm run build

# 2. Run lints
cargo clippy -- -D warnings
npm run lint

# 3. Only then push
git push
```

### Quick Reference

| Rule | Limit | Action if Exceeded |
|------|-------|-------------------|
| File size | 500 lines | Split into modules |
| Function size | 50 lines | Extract helper functions |
| Nesting depth | 3 levels | Refactor or early return |
| Parameters | 5 params | Use config/options struct |
| Commit frequency | Every logical change | Commit more often |
| Push condition | All tests pass | Fix before pushing |

---

## Technology Stack

### Core Languages

| Component | Language | Runtime/Framework | Rationale |
|-----------|----------|-------------------|-----------|
| **Backend API** | Rust | Axum + Tokio | Performance, memory safety, single binary |
| **CLI Tool** | Rust | clap | Instant startup, easy distribution |
| **Core Library** | Rust | - | Shared logic between server and WASM |
| **WASM Module** | Rust | wasm-bindgen | Client-side performance (OCR, matching) |
| **Frontend** | TypeScript | Next.js 14 | PWA, React ecosystem |
| **LLM Scripts** | Python | - | Training, LangChain (optional sidecar) |

### Infrastructure

| Component | Technology | Notes |
|-----------|------------|-------|
| Database | PostgreSQL 16 + pgvector | Relational + vector search |
| Cache | Redis | Sessions, rate limiting |
| Object Storage | S3-compatible | Card images, captures |
| Message Queue | Redis Streams | Async job processing |
| Container | Docker | Single-container deployment |

---

## Project Structure

```
mtg-ai-suite/
├── .github/
│   └── workflows/
│       ├── ci.yml              # Continuous Integration
│       ├── release.yml         # Release builds
│       └── deploy.yml          # Deployment pipeline
│
├── crates/                     # Rust workspace
│   ├── mtg-core/              # Shared core library
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── card.rs        # Card data structures
│   │   │   ├── recognition.rs # Recognition algorithms
│   │   │   ├── fuzzy.rs       # Fuzzy matching
│   │   │   └── error.rs       # Error types
│   │   ├── Cargo.toml
│   │   └── tests/
│   │
│   ├── mtg-server/            # Backend API server
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── api/           # Route handlers
│   │   │   ├── services/      # Business logic
│   │   │   ├── db/            # Database layer
│   │   │   ├── auth/          # Authentication
│   │   │   └── config.rs      # Configuration
│   │   ├── Cargo.toml
│   │   ├── tests/
│   │   └── migrations/        # SQL migrations
│   │
│   ├── mtg-cli/               # CLI tool
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   └── commands/      # CLI commands
│   │   ├── Cargo.toml
│   │   └── tests/
│   │
│   └── mtg-wasm/              # WebAssembly module
│       ├── src/
│       │   └── lib.rs
│       ├── Cargo.toml
│       └── tests/
│
├── client/                     # TypeScript frontend
│   ├── src/
│   │   ├── app/               # Next.js app router
│   │   ├── components/        # React components
│   │   ├── hooks/             # Custom hooks
│   │   ├── lib/               # Utilities
│   │   ├── services/          # API client
│   │   ├── store/             # Zustand stores
│   │   └── types/             # TypeScript types
│   ├── public/
│   ├── tests/
│   │   ├── unit/
│   │   ├── integration/
│   │   └── e2e/
│   ├── package.json
│   ├── tsconfig.json
│   └── next.config.js
│
├── scripts/                    # Development scripts
│   ├── setup.sh               # Initial setup
│   ├── seed-db.sh             # Database seeding
│   └── build-wasm.sh          # WASM build
│
├── docker/
│   ├── Dockerfile             # Production image
│   ├── Dockerfile.dev         # Development image
│   └── docker-compose.yml     # Local development
│
├── docs/
│   ├── PRD_BACKEND.md
│   ├── PRD_FRONTEND.md
│   └── API.md                 # API documentation
│
├── references/
│   └── LITERATURE_REVIEW.md
│
├── test-data/                  # Test fixtures
│   ├── images/                # Test card images
│   ├── fixtures/              # JSON fixtures
│   └── mocks/                 # Mock data
│
├── Cargo.toml                  # Rust workspace root
├── Cargo.lock
├── project.yaml               # Project roadmap
├── ARCHITECTURE.md            # This file
├── README.md
├── LICENSE
└── .gitignore
```

---

## Design Principles

### 1. Separation of Concerns

```
┌─────────────────────────────────────────────────────────────┐
│                      Presentation Layer                      │
│                   (Client, CLI, API Routes)                  │
├─────────────────────────────────────────────────────────────┤
│                      Application Layer                       │
│                   (Services, Use Cases)                      │
├─────────────────────────────────────────────────────────────┤
│                       Domain Layer                           │
│              (Core Types, Business Logic)                    │
├─────────────────────────────────────────────────────────────┤
│                    Infrastructure Layer                      │
│            (Database, External APIs, Storage)                │
└─────────────────────────────────────────────────────────────┘
```

### 2. Error Handling

All errors must be:
- **Typed**: Use `thiserror` for Rust, custom error types for TypeScript
- **Traceable**: Include context for debugging
- **User-friendly**: Provide actionable messages

```rust
// Rust error example
#[derive(Debug, thiserror::Error)]
pub enum RecognitionError {
    #[error("Image too small: {width}x{height}, minimum is {min_width}x{min_height}")]
    ImageTooSmall { width: u32, height: u32, min_width: u32, min_height: u32 },
    
    #[error("OCR failed: {reason}")]
    OcrFailed { reason: String },
    
    #[error("No card match found for text: {text}")]
    NoMatch { text: String },
}
```

### 3. Configuration

All configuration via environment variables with sensible defaults:

```rust
// Rust config
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    
    pub database_url: String,
    
    #[serde(default)]
    pub redis_url: Option<String>,
    
    pub llm_api_key: Option<String>,
}

fn default_port() -> u16 { 8080 }
```

### 4. API Design

RESTful with consistent patterns:

```
GET    /api/v1/resources          # List
POST   /api/v1/resources          # Create
GET    /api/v1/resources/:id      # Read
PUT    /api/v1/resources/:id      # Update (full)
PATCH  /api/v1/resources/:id      # Update (partial)
DELETE /api/v1/resources/:id      # Delete
```

Response format:
```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "ISO8601"
  }
}
```

Error format:
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Human readable message",
    "details": { ... }
  },
  "meta": {
    "request_id": "uuid",
    "timestamp": "ISO8601"
  }
}
```

---

## Testing Standards

### Test Pyramid

```
        ┌─────────┐
        │   E2E   │  10% - Critical user journeys
        ├─────────┤
        │ Integr. │  20% - API, database, external services
        ├─────────┤
        │  Unit   │  70% - Functions, modules, components
        └─────────┘
```

### Rust Testing

```rust
// Unit test - same file
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuzzy_match_exact() {
        let matcher = FuzzyMatcher::new(vec!["Lightning Bolt".into()]);
        let result = matcher.find("Lightning Bolt");
        assert_eq!(result.unwrap().name, "Lightning Bolt");
        assert_eq!(result.unwrap().confidence, 1.0);
    }
    
    #[test]
    fn test_fuzzy_match_typo() {
        let matcher = FuzzyMatcher::new(vec!["Lightning Bolt".into()]);
        let result = matcher.find("Lightening Bolt"); // Common typo
        assert_eq!(result.unwrap().name, "Lightning Bolt");
        assert!(result.unwrap().confidence > 0.8);
    }
}

// Integration test - tests/ directory
#[tokio::test]
async fn test_capture_upload_flow() {
    let app = spawn_test_app().await;
    let client = reqwest::Client::new();
    
    let response = client
        .post(&format!("{}/api/v1/captures", app.address))
        .header("Authorization", format!("Bearer {}", app.api_key))
        .json(&json!({
            "image": base64_test_image(),
            "metadata": { "captured_at": "2024-01-01T00:00:00Z" }
        }))
        .send()
        .await
        .unwrap();
    
    assert_eq!(response.status(), 201);
}
```

### TypeScript Testing

```typescript
// Unit test - Vitest
import { describe, it, expect } from 'vitest';
import { processImage } from './image-processor';

describe('processImage', () => {
  it('should resize large images', async () => {
    const largeImage = createTestImage(4000, 3000);
    const result = await processImage(largeImage);
    expect(result.width).toBeLessThanOrEqual(1920);
  });
  
  it('should preserve aspect ratio', async () => {
    const image = createTestImage(1600, 1200); // 4:3
    const result = await processImage(image);
    const ratio = result.width / result.height;
    expect(ratio).toBeCloseTo(4/3, 2);
  });
});

// E2E test - Playwright
import { test, expect } from '@playwright/test';

test('card scanning flow', async ({ page }) => {
  await page.goto('/scan');
  
  // Upload test image
  await page.setInputFiles('input[type="file"]', 'test-data/images/lightning-bolt.jpg');
  
  // Wait for recognition
  await expect(page.getByText('Lightning Bolt')).toBeVisible({ timeout: 10000 });
  
  // Confirm card
  await page.getByRole('button', { name: 'Add to Deck' }).click();
  
  // Verify added
  await expect(page.getByText('Card added')).toBeVisible();
});
```

### Test Data Management

```yaml
# test-data/fixtures/cards.yaml
cards:
  - id: "lightning-bolt-m10"
    name: "Lightning Bolt"
    scryfall_id: "e3285e6b-3e79-4d7c-bf96-d920f973b122"
    mana_cost: "{R}"
    type_line: "Instant"
    oracle_text: "Lightning Bolt deals 3 damage to any target."
    
  - id: "counterspell-tmp"
    name: "Counterspell"
    scryfall_id: "..."
```

---

## CI/CD Pipeline

### Continuous Integration

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  # ============================================
  # Rust checks
  # ============================================
  rust-check:
    name: Rust Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust toolchain
        uses: dtolnay/rust-action@stable
        with:
          components: rustfmt, clippy
          
      - name: Cache cargo
        uses: Swatinem/rust-cache@v2
        
      - name: Check formatting
        run: cargo fmt --all -- --check
        
      - name: Clippy lints
        run: cargo clippy --all-targets --all-features -- -D warnings
        
      - name: Run tests
        run: cargo test --all-features
        
      - name: Build release
        run: cargo build --release

  # ============================================
  # TypeScript checks
  # ============================================
  typescript-check:
    name: TypeScript Check
    runs-on: ubuntu-latest
    defaults:
      run:
        working-directory: ./client
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: client/package-lock.json
          
      - name: Install dependencies
        run: npm ci
        
      - name: Type check
        run: npm run type-check
        
      - name: Lint
        run: npm run lint
        
      - name: Run tests
        run: npm run test
        
      - name: Build
        run: npm run build

  # ============================================
  # Integration tests
  # ============================================
  integration:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: [rust-check, typescript-check]
    services:
      postgres:
        image: pgvector/pgvector:pg16
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: mtg_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7-alpine
        ports:
          - 6379:6379
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        
      - name: Run integration tests
        run: cargo test --test '*' --features integration
        env:
          DATABASE_URL: postgres://postgres:postgres@localhost:5432/mtg_test
          REDIS_URL: redis://localhost:6379

  # ============================================
  # E2E tests
  # ============================================
  e2e:
    name: E2E Tests
    runs-on: ubuntu-latest
    needs: [integration]
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          
      - name: Install Playwright
        run: npx playwright install --with-deps
        working-directory: ./client
        
      - name: Build and start server
        run: |
          cargo build --release
          ./target/release/mtg-server &
          cd client && npm run build && npm run start &
          sleep 10
          
      - name: Run E2E tests
        run: npm run test:e2e
        working-directory: ./client

  # ============================================
  # WASM build
  # ============================================
  wasm:
    name: WASM Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: wasm32-unknown-unknown
          
      - name: Install wasm-pack
        run: cargo install wasm-pack
        
      - name: Build WASM
        run: wasm-pack build crates/mtg-wasm --target web
        
      - name: Upload WASM artifact
        uses: actions/upload-artifact@v4
        with:
          name: wasm-pkg
          path: crates/mtg-wasm/pkg/
```

### Release Pipeline

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    name: Build ${{ matrix.target }}
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Build
        run: cargo build --release --target ${{ matrix.target }}
        
      - name: Package
        run: |
          mkdir -p dist
          cp target/${{ matrix.target }}/release/mtg-server dist/
          cp target/${{ matrix.target }}/release/mtg-cli dist/
          tar -czvf mtg-ai-suite-${{ matrix.target }}.tar.gz -C dist .
          
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: mtg-ai-suite-${{ matrix.target }}
          path: mtg-ai-suite-${{ matrix.target }}.tar.gz

  docker:
    name: Docker Image
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
        
      - name: Login to GHCR
        uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
          
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            ghcr.io/${{ github.repository }}:${{ github.ref_name }}
            ghcr.io/${{ github.repository }}:latest
          cache-from: type=gha
          cache-to: type=gha,mode=max

  release:
    name: Create Release
    runs-on: ubuntu-latest
    needs: [build, docker]
    steps:
      - name: Download artifacts
        uses: actions/download-artifact@v4
        
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            mtg-ai-suite-*/mtg-ai-suite-*.tar.gz
          generate_release_notes: true
```

---

## Code Style

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` with default settings
- All public items must have doc comments
- Prefer `thiserror` for error types
- Use `tracing` for logging

```rust
/// Performs fuzzy matching against the card database.
/// 
/// # Arguments
/// * `query` - The OCR result or user input to match
/// 
/// # Returns
/// The best matching card with confidence score, or None if no match found.
/// 
/// # Example
/// ```
/// let matcher = FuzzyMatcher::new(card_names);
/// let result = matcher.find("Lightening Bolt");
/// assert_eq!(result.unwrap().name, "Lightning Bolt");
/// ```
pub fn find(&self, query: &str) -> Option<MatchResult> {
    // Implementation
}
```

### TypeScript

- Use ESLint with `@typescript-eslint`
- Use Prettier for formatting
- Strict TypeScript (`strict: true`)
- Prefer `type` over `interface` for consistency
- Use barrel exports (`index.ts`)

```typescript
// types/card.ts
export type Card = {
  id: string;
  name: string;
  manaCost: string | null;
  typeLine: string;
  oracleText: string | null;
};

// types/index.ts
export * from './card';
export * from './deck';
export * from './game';
```

---

## Security Standards

### Authentication

- API keys are hashed with Argon2
- Keys never logged or exposed in errors
- Rate limiting per API key
- Key rotation supported

### Data Protection

- HTTPS required in production
- Sensitive config via environment variables
- No secrets in code or logs
- Database credentials rotated regularly

### Input Validation

- All inputs validated at API boundary
- File uploads scanned and size-limited
- SQL injection prevented via parameterized queries
- XSS prevented via output encoding

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| API response (p95) | < 100ms | Prometheus |
| Image processing | < 500ms | Tracing |
| Card recognition | < 2s | Tracing |
| WebSocket latency | < 50ms | Prometheus |
| Memory usage | < 512MB | Docker stats |
| Binary size | < 50MB | CI artifact |
| WASM size | < 2MB | CI artifact |
| Client bundle | < 200KB | Lighthouse |

---

## Monitoring & Observability

### Logging

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(image))]
pub async fn process_capture(
    capture_id: Uuid,
    image: &[u8],
) -> Result<Recognition, Error> {
    info!(capture_id = %capture_id, size = image.len(), "Processing capture");
    
    let result = recognize(image).await?;
    
    info!(
        capture_id = %capture_id,
        card = %result.card_name,
        confidence = result.confidence,
        "Recognition complete"
    );
    
    Ok(result)
}
```

### Metrics

Expose Prometheus metrics at `/metrics`:
- `http_requests_total`
- `http_request_duration_seconds`
- `recognition_duration_seconds`
- `recognition_confidence`
- `active_websocket_connections`

---

## Documentation Requirements

Every module must have:
1. **README.md** - Overview, setup, usage
2. **Doc comments** - All public APIs
3. **Examples** - In doc comments or `examples/` directory
4. **Changelog** - Notable changes per version

---

## AI Agent Instructions

When implementing features:

1. **Read this document first** - Understand the architecture
2. **Check project.yaml** - Follow the phase order
3. **Write tests first** - TDD approach
4. **Follow code style** - Run formatters and linters
5. **Update documentation** - Keep docs in sync
6. **Small commits** - One logical change per commit
7. **Run CI locally** - `cargo test && cd client && npm test`

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Example:
```
feat(recognition): add fuzzy matching for card names

Implements SymSpell-based fuzzy matching with configurable
edit distance. Handles common OCR errors like 0/O confusion.

Closes #123
```
