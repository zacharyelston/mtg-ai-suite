# Feature Set & Issue Triage

This document consolidates all features by version and provides a checklist for triaging new issues.

## Current Project Structure

```
develop/
├── crates/          # Rust backend (v0.1.0+)
│   ├── mtg-server/  # API server
│   ├── mtg-core/    # Core library
│   ├── mtg-cli/     # CLI tool
│   └── mtg-wasm/    # WebAssembly module
├── frontend/        # Next.js PWA (v0.4.0+)
├── database/        # Flyway migrations
├── scripts/         # Python utilities
└── docs/            # Documentation
```

---

## Feature Set by Version

### v0.1.0 - Core Backend API (Jan 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| Health endpoint with timestamp | API | 🔲 | #7 |
| Card search (fuzzy) | API+SERVER+DB | 🔲 | #8 |
| Card autocomplete | API | 🔲 | #8 |
| Get card by ID | API | 🔲 | #8 |
| Environment config | CONFIG | 🔲 | - |
| Database migrations | DB | ✅ | - |
| Scryfall data loader | DB | ✅ | #9 |

### v0.2.0 - Vector Search (Feb 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| Qdrant container | CONFIG | 🔲 | - |
| Card embeddings | SERVER | 🔲 | - |
| Semantic card search | API | 🔲 | - |
| Rules embeddings | DB | 🔲 | - |
| Rules Q&A search | API | 🔲 | - |

### v0.3.0 - Card Recognition (Mar 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| Image upload endpoint | API | 🔲 | #12 |
| OCR text extraction | SERVER | 🔲 | #12 |
| Recognition result endpoint | API | 🔲 | #12 |
| Vector fallback for OCR | SERVER | 🔲 | - |

### v0.4.0 - Frontend MVP (Apr 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| Next.js PWA shell | FRONT | 🔲 | - |
| Card search page | FRONT | 🔲 | #10 |
| Card detail modal | FRONT | 🔲 | - |
| Camera capture | FRONT | 🔲 | #11 |
| Recognition result display | FRONT | 🔲 | - |
| Offline support | FRONT | 🔲 | - |

### v1.0.0 - General Availability (Apr 2025)

All features from v0.1.0 - v0.4.0 complete and tested.

**Release Criteria:**
- [ ] Backend builds and passes all tests
- [ ] Frontend builds and passes all tests
- [ ] Card search returns results from database
- [ ] Semantic search returns relevant cards
- [ ] Card recognition identifies cards from images
- [ ] PWA installs on mobile devices
- [ ] Offline mode works for cached data
- [ ] Docker Compose deploys full stack
- [ ] API documentation is complete
- [ ] No critical or high-severity bugs open

### v1.1.0 - AI Integration (May 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| LLM service (OpenAI/Ollama) | SERVER | 🔲 | - |
| Play suggestions API | API | 🔲 | - |
| Deck analysis API | API | 🔲 | - |
| AI chat interface | FRONT | 🔲 | - |

### v1.2.0 - Multi-Backend (Jun 2025)

| Feature | Layer | Status | Issue |
|---------|-------|--------|-------|
| Server discovery | FRONT | 🔲 | - |
| API key management | FRONT | 🔲 | - |
| Feature routing | FRONT | 🔲 | - |
| Cross-server data sync | FRONT | 🔲 | - |

---

## Issue Triage Checklist

When a new issue comes in, use this checklist to categorize and prioritize it.

### 1. Identify the Layer

```
[ ] CONFIG  - Environment, secrets, feature toggles
[ ] DB      - Database, migrations, data models
[ ] SERVER  - Business logic, services
[ ] API     - HTTP routes, request/response
[ ] FRONT   - UI components, user interactions
```

### 2. Map to Version

```
[ ] v0.1.0 - Core backend (card search, health)
[ ] v0.2.0 - Vector search (semantic, embeddings)
[ ] v0.3.0 - Recognition (OCR, image upload)
[ ] v0.4.0 - Frontend (PWA, camera)
[ ] v1.0.0 - GA release criteria
[ ] v1.1.0 - AI features
[ ] v1.2.0 - Multi-backend
[ ] Backlog - Future consideration
```

### 3. Determine Priority

| Priority | Criteria |
|----------|----------|
| **P0 - Critical** | Blocks release, security issue, data loss |
| **P1 - High** | Core feature for current version |
| **P2 - Medium** | Important but not blocking |
| **P3 - Low** | Nice to have, future version |

### 4. Check Dependencies

Before starting work, verify:

```
[ ] Required infrastructure exists (DB, Qdrant, etc.)
[ ] Dependent features are complete
[ ] No conflicting PRs in progress
[ ] Tests can be written for this feature
```

### 5. Apply Labels

| Label | When to Use |
|-------|-------------|
| `bug` | Something is broken |
| `feature` | New functionality |
| `enhancement` | Improvement to existing |
| `documentation` | Docs only |
| `backend` | Rust crates |
| `frontend` | Next.js app |
| `infrastructure` | Docker, CI, config |
| `v0.1.0` - `v1.2.0` | Target version |
| `blocked` | Waiting on dependency |

---

## Decision Tree: What to Work On

```
START
  │
  ▼
Is there a P0/Critical issue?
  │
  ├─ YES → Work on it immediately
  │
  └─ NO
      │
      ▼
    What is the current target version?
      │
      ├─ v0.1.0 → Focus on: Health, Card Search, Config
      │
      ├─ v0.2.0 → Focus on: Qdrant, Embeddings, Semantic Search
      │
      ├─ v0.3.0 → Focus on: OCR, Image Upload, Recognition
      │
      ├─ v0.4.0 → Focus on: PWA, Camera, Offline
      │
      └─ v1.0.0 → Focus on: Testing, Docs, Bug Fixes
          │
          ▼
        Are all features for this version complete?
          │
          ├─ NO → Pick the highest priority incomplete feature
          │
          └─ YES → Tag release, move to next version
```

---

## Quick Reference

### Documentation Links

| Document | Purpose |
|----------|---------|
| [Release Schedule](RELEASE_SCHEDULE.md) | Version timeline, containers |
| [Architecture](../ARCHITECTURE.md) | Code standards, structure |
| [Multi-Backend](ARCHITECTURE_MULTI_BACKEND.md) | Server federation design |
| [PRD Frontend](PRD_FRONTEND.md) | Frontend requirements |
| [PRD Backend](PRD_BACKEND.md) | Backend requirements |
| [Replit Worklist](../REPLIT_WORKLIST.md) | Task IDs by layer |

### Wiki Links

- [Home](https://github.com/zacharyelston/mtg-ai-suite/wiki)
- [Replit Agent Worklist](https://github.com/zacharyelston/mtg-ai-suite/wiki/Replit-Agent-Worklist)
- [Multi-Backend Architecture](https://github.com/zacharyelston/mtg-ai-suite/wiki/Multi-Backend-Architecture)
- [Release Schedule](https://github.com/zacharyelston/mtg-ai-suite/wiki/Release-Schedule)

### GitHub Issues

- [Open Issues](https://github.com/zacharyelston/mtg-ai-suite/issues)
- [v0.1.0 Milestone](https://github.com/zacharyelston/mtg-ai-suite/milestone/1) (create if needed)
