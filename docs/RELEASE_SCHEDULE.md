# Release Schedule

This document outlines the phased release plan for MTG AI Suite, including infrastructure components and feature milestones.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              Docker Compose Stack                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐    │
│  │   Frontend   │  │   Backend    │  │   Vector DB  │  │   Postgres   │    │
│  │   (Next.js)  │  │   (Rust)     │  │   (Qdrant)   │  │  (pgvector)  │    │
│  │   Port 3000  │  │   Port 8080  │  │   Port 6333  │  │   Port 5432  │    │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘    │
│         │                 │                 │                 │             │
│         └─────────────────┴─────────────────┴─────────────────┘             │
│                                    │                                         │
│                            Internal Network                                  │
│                                                                              │
│  ┌──────────────┐  ┌──────────────┐                                         │
│  │    Redis     │  │   Flyway     │                                         │
│  │   Port 6379  │  │  (migrations)│                                         │
│  └──────────────┘  └──────────────┘                                         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Release Phases

### Phase 0: Foundation (Current)
**Target: December 2024**

| Component | Status | Description |
|-----------|--------|-------------|
| CI/CD Pipeline | ✅ Done | GitHub Actions for Rust + TypeScript |
| Database Schema | ✅ Done | Flyway migrations with initial schema |
| Scryfall Loader | ✅ Done | Python script to load card data |
| Documentation | ✅ Done | Architecture, PRDs, Wiki |

**Containers:**
- PostgreSQL + pgvector
- Redis
- Flyway (migrations)

---

### Phase 1: Core Backend
**Target: January 2025**

| Feature | Layer | Task IDs | Description |
|---------|-------|----------|-------------|
| Health Endpoint | API | A1 | `/health` with timestamp |
| Card Search | DB+SERVER+API | D3, S1, A2-A4 | Fuzzy search, autocomplete |
| Config Management | CONFIG | C1-C3 | Environment variables |

**New Containers:** None

**Deliverables:**
- Working Rust backend with card search
- SQLite for development, PostgreSQL for production
- API documentation

---

### Phase 2: Vector Search
**Target: February 2025**

| Feature | Layer | Task IDs | Description |
|---------|-------|----------|-------------|
| Vector DB Container | CONFIG | V1 | Qdrant container in docker-compose |
| Embedding Service | SERVER | V2 | Generate embeddings for cards |
| Semantic Search API | API | V3-V4 | `/api/cards/semantic` endpoint |
| Rules Embeddings | DB | V5 | MTG rules vector search |

**New Containers:**
```yaml
qdrant:
  image: qdrant/qdrant:latest
  ports:
    - "6333:6333"
  volumes:
    - qdrant_data:/qdrant/storage
```

**Vector Collections:**

| Collection | Content | Embedding Model | Use Case |
|------------|---------|-----------------|----------|
| `cards` | Card name + oracle text + type | OpenAI ada-002 / local | Semantic card search |
| `rules` | MTG Comprehensive Rules sections | OpenAI ada-002 / local | Rules Q&A |
| `decks` | Deck card composition | OpenAI ada-002 / local | Similar deck finder |

**Deliverables:**
- Qdrant container running alongside backend
- Semantic card search ("find burn spells that go face")
- Rules lookup ("what happens when a creature dies")

---

### Phase 3: Card Recognition
**Target: March 2025**

| Feature | Layer | Task IDs | Description |
|---------|-------|----------|-------------|
| Image Upload | API | A5 | `POST /api/captures` |
| OCR Pipeline | SERVER | S2 | Text extraction from images |
| Recognition Result | API | A6 | `GET /api/captures/:id` |
| Vector Fallback | SERVER | S3 | Use embeddings for fuzzy OCR matches |

**New Containers:** None (OCR runs in backend)

**Recognition Flow:**
```
Image → OCR → Text → Fuzzy Match → Card
                 ↓
            (if low confidence)
                 ↓
         Vector Search → Similar Cards
```

**Deliverables:**
- Upload card image, get card identification
- Confidence scores
- Vector-based fallback for unclear OCR

---

### Phase 4: Frontend MVP
**Target: April 2025**

| Feature | Layer | Task IDs | Description |
|---------|-------|----------|-------------|
| Next.js App | FRONT | F1 | PWA with TypeScript + Tailwind |
| Card Browser | FRONT | F2-F3 | Search, grid, detail modal |
| Camera Capture | FRONT | F4-F5 | WebRTC camera, recognition UI |

**New Containers:**
```yaml
frontend:
  build: ./client
  ports:
    - "3000:3000"
  depends_on:
    - backend
```

**Deliverables:**
- Mobile-responsive PWA
- Card search with autocomplete
- Camera-based card recognition
- Offline support (cached cards)

---

### Phase 5: AI Integration
**Target: May 2025**

| Feature | Layer | Description |
|---------|-------|-------------|
| LLM Service | SERVER | OpenAI / Ollama integration |
| Play Suggestions | API | `POST /api/ai/suggest` |
| Deck Analysis | API | `POST /api/ai/analyze-deck` |
| Chat Interface | FRONT | AI assistant UI |

**New Containers (optional):**
```yaml
ollama:
  image: ollama/ollama:latest
  ports:
    - "11434:11434"
  volumes:
    - ollama_data:/root/.ollama
```

**Deliverables:**
- AI-powered play suggestions
- Deck strength analysis
- Natural language rules queries

---

### Phase 6: Multi-Backend
**Target: June 2025**

| Feature | Layer | Description |
|---------|-------|-------------|
| Server Discovery | FRONT | Connect to multiple backends |
| API Key Management | FRONT | Per-server authentication |
| Feature Routing | FRONT | Route requests to appropriate server |
| Data Sync | FRONT | Sync decks/collection across servers |

**Deliverables:**
- Connect to home server + cloud server
- Route AI requests to cloud, data to local
- Offline-first with background sync

---

## Container Summary

| Phase | New Containers | Total Containers |
|-------|----------------|------------------|
| 0 | PostgreSQL, Redis, Flyway | 3 |
| 1 | - | 3 |
| 2 | **Qdrant** | 4 |
| 3 | - | 4 |
| 4 | **Frontend** | 5 |
| 5 | Ollama (optional) | 5-6 |
| 6 | - | 5-6 |

---

## Docker Compose (Full Stack)

```yaml
version: '3.8'

services:
  # ============================================
  # Frontend
  # ============================================
  frontend:
    build: ./client
    ports:
      - "3000:3000"
    environment:
      - NEXT_PUBLIC_API_URL=http://backend:8080
    depends_on:
      - backend

  # ============================================
  # Backend
  # ============================================
  backend:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://postgres:postgres@db:5432/mtg
      - REDIS_URL=redis://redis:6379
      - QDRANT_URL=http://qdrant:6333
      - RUST_LOG=info
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
      qdrant:
        condition: service_started

  # ============================================
  # Vector Database
  # ============================================
  qdrant:
    image: qdrant/qdrant:latest
    ports:
      - "6333:6333"
    volumes:
      - qdrant_data:/qdrant/storage

  # ============================================
  # PostgreSQL + pgvector
  # ============================================
  db:
    image: pgvector/pgvector:pg16
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: mtg
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  # ============================================
  # Redis
  # ============================================
  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  # ============================================
  # Database Migrations
  # ============================================
  flyway:
    image: flyway/flyway:10
    command: migrate
    volumes:
      - ./database/migrations:/flyway/sql
      - ./database/flyway.conf:/flyway/conf/flyway.conf
    depends_on:
      db:
        condition: service_healthy

  # ============================================
  # Local LLM (Optional)
  # ============================================
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
    profiles:
      - ai  # Only start with: docker compose --profile ai up

volumes:
  postgres_data:
  redis_data:
  qdrant_data:
  ollama_data:
```

---

## Why Qdrant Over pgvector?

| Aspect | pgvector | Qdrant |
|--------|----------|--------|
| **Setup** | Extension in PostgreSQL | Separate container |
| **Performance** | Good for <1M vectors | Optimized for large scale |
| **Filtering** | SQL WHERE clauses | Native payload filtering |
| **Updates** | Full table scan | Incremental indexing |
| **Memory** | Shares with PostgreSQL | Dedicated resources |
| **API** | SQL | REST + gRPC |

**Recommendation**: Use **Qdrant** as a dedicated vector container because:
1. Cleaner separation of concerns
2. Better performance for semantic search
3. Easy to scale independently
4. Rich filtering capabilities (by color, CMC, type)
5. Simple REST API for the Rust backend

---

## Milestones

| Milestone | Phase | Target | Key Deliverable |
|-----------|-------|--------|-----------------|
| **M1** | 1 | Jan 2025 | Card search API working |
| **M2** | 2 | Feb 2025 | Semantic search with Qdrant |
| **M3** | 3 | Mar 2025 | Card recognition from images |
| **M4** | 4 | Apr 2025 | Mobile PWA released |
| **M5** | 5 | May 2025 | AI play suggestions |
| **M6** | 6 | Jun 2025 | Multi-backend support |

---

## Getting Started

```bash
# Phase 0-1: Basic stack
docker compose up db redis flyway backend

# Phase 2+: With vector search
docker compose up db redis flyway qdrant backend

# Phase 4+: Full stack
docker compose up

# With local LLM
docker compose --profile ai up
```
