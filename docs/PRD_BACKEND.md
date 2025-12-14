# MTG AI Suite - Backend Product Requirements Document

## Overview

The MTG AI Suite Backend is a self-hosted server application that provides AI-powered Magic: The Gathering analysis, game tracking, and play suggestions. It's designed to be deployed by users on their own infrastructure with their own LLM API keys.

---

## 1. Product Vision

### 1.1 Purpose
Provide a powerful, self-hosted backend engine that MTG players can deploy to get personalized AI assistance for deck building, game analysis, and play recommendations.

### 1.2 Target Users
- **Primary**: MTG enthusiasts with technical ability to self-host
- **Secondary**: MTG communities/groups wanting shared infrastructure
- **Tertiary**: Tournament organizers needing analysis tools

### 1.3 Key Value Propositions
- Full data ownership and privacy
- Customizable LLM integration (OpenAI, Anthropic, local models)
- Multi-client support via API key management
- No subscription fees (bring your own LLM API key)

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        MTG AI Suite Backend                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │   REST API   │  │  WebSocket   │  │    Push Service      │  │
│  │   Gateway    │  │   Server     │  │  (Firebase/APNs)     │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬───────────┘  │
│         │                 │                      │              │
│  ┌──────┴─────────────────┴──────────────────────┴───────────┐  │
│  │                    Service Layer                          │  │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────────┐ │  │
│  │  │  Card   │ │  Deck   │ │  Game   │ │  AI Suggestion  │ │  │
│  │  │ Service │ │ Service │ │ Service │ │    Service      │ │  │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────────────┘ │  │
│  └───────────────────────────┬───────────────────────────────┘  │
│                              │                                   │
│  ┌───────────────────────────┴───────────────────────────────┐  │
│  │                    Data Layer                              │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌───────────┐ │  │
│  │  │PostgreSQL│  │  Redis   │  │  Vector  │  │   LLM     │ │  │
│  │  │  + Cards │  │  Cache   │  │    DB    │  │  Gateway  │ │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └───────────┘ │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    CLI Interface                           │  │
│  │  mtg-ai-suite apikey create | list | revoke | rotate      │  │
│  │  mtg-ai-suite server start | stop | status                │  │
│  │  mtg-ai-suite db migrate | seed | backup                  │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Functional Requirements

### 3.1 Authentication & API Key Management

#### 3.1.1 CLI Commands
```bash
# API Key Management
mtg-ai-suite apikey create --name "iPhone" --permissions read,write
mtg-ai-suite apikey create --name "iPad" --permissions read
mtg-ai-suite apikey list
mtg-ai-suite apikey revoke <key_id>
mtg-ai-suite apikey rotate <key_id>

# Server Management
mtg-ai-suite server start --port 8080
mtg-ai-suite server stop
mtg-ai-suite server status
mtg-ai-suite server config set LLM_PROVIDER openai
mtg-ai-suite server config set LLM_API_KEY sk-xxx

# Database Management
mtg-ai-suite db migrate
mtg-ai-suite db seed --source scryfall
mtg-ai-suite db backup --output ./backup.sql
mtg-ai-suite db restore --input ./backup.sql
```

#### 3.1.2 API Key Properties
| Field | Type | Description |
|-------|------|-------------|
| id | UUID | Unique identifier |
| name | string | Human-readable name (e.g., "John's iPhone") |
| key_hash | string | Hashed API key (never store plaintext) |
| permissions | string[] | Array of permissions |
| created_at | datetime | Creation timestamp |
| last_used_at | datetime | Last usage timestamp |
| expires_at | datetime | Optional expiration |
| is_active | boolean | Active status |
| device_token | string | Push notification token |

#### 3.1.3 Permissions Model
- `read` - Read card data, decks, game history
- `write` - Create/modify decks, log games
- `ai` - Access AI suggestions
- `push` - Receive push notifications
- `admin` - Manage other API keys

### 3.2 Card Database Service

#### 3.2.1 Data Sync
- Initial seed from Scryfall bulk data
- Daily incremental updates
- Manual sync trigger via CLI

#### 3.2.2 Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/v1/cards | Search/list cards |
| GET | /api/v1/cards/:id | Get card by ID |
| GET | /api/v1/cards/autocomplete | Card name autocomplete |
| GET | /api/v1/cards/random | Random card |
| GET | /api/v1/sets | List all sets |
| GET | /api/v1/sets/:code | Get set details |

#### 3.2.3 Search Capabilities
- Full-text search on card names and oracle text
- Semantic search via vector embeddings
- Filter by: colors, types, CMC, sets, rarity, legality
- Sort by: name, CMC, price, release date

### 3.3 Deck Service

#### 3.3.1 Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/v1/decks | List user's decks |
| POST | /api/v1/decks | Create deck |
| GET | /api/v1/decks/:id | Get deck |
| PUT | /api/v1/decks/:id | Update deck |
| DELETE | /api/v1/decks/:id | Delete deck |
| POST | /api/v1/decks/:id/analyze | AI deck analysis |
| POST | /api/v1/decks/:id/suggestions | Get card suggestions |
| POST | /api/v1/decks/import | Import from text/URL |
| GET | /api/v1/decks/:id/export | Export deck |

#### 3.3.2 Deck Analysis Features
- Mana curve visualization data
- Color distribution
- Card type breakdown
- Archetype classification
- Synergy scoring
- Weakness identification
- Suggested improvements

### 3.4 Game State Service

#### 3.4.1 Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/v1/games | Start new game |
| GET | /api/v1/games/:id | Get game state |
| PUT | /api/v1/games/:id | Update game state |
| POST | /api/v1/games/:id/action | Log game action |
| GET | /api/v1/games/:id/suggestions | Get play suggestions |
| GET | /api/v1/games/:id/history | Get game history |
| POST | /api/v1/games/:id/end | End game |

#### 3.4.2 WebSocket Events
```typescript
// Client -> Server
{ type: "game:join", gameId: string }
{ type: "game:action", action: GameAction }
{ type: "game:request_suggestion" }

// Server -> Client
{ type: "game:state", state: GameState }
{ type: "game:suggestion", suggestions: PlaySuggestion[] }
{ type: "game:notification", message: string }
```

### 3.5 AI Suggestion Service

#### 3.5.1 LLM Integration
- **Supported Providers**:
  - OpenAI (GPT-4, GPT-4-turbo)
  - Anthropic (Claude 3)
  - Local models (Ollama, llama.cpp)
  - Azure OpenAI

#### 3.5.2 Configuration
```yaml
# config.yaml
llm:
  provider: openai  # openai | anthropic | ollama | azure
  api_key: ${LLM_API_KEY}  # Environment variable
  model: gpt-4-turbo
  max_tokens: 2000
  temperature: 0.7
  
  # For local models
  ollama:
    base_url: http://localhost:11434
    model: llama2

  # For Azure
  azure:
    endpoint: https://your-resource.openai.azure.com
    deployment: gpt-4
    api_version: 2024-02-15-preview
```

#### 3.5.3 AI Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/v1/ai/analyze-board | Analyze current board state |
| POST | /api/v1/ai/suggest-play | Get play suggestions |
| POST | /api/v1/ai/evaluate-hand | Evaluate opening hand |
| POST | /api/v1/ai/draft-pick | Draft pick recommendation |
| POST | /api/v1/ai/deck-suggestions | Deck improvement suggestions |
| POST | /api/v1/ai/rules-question | Answer rules questions |

### 3.6 Push Notification Service

#### 3.6.1 Supported Platforms
- **iOS**: Apple Push Notification Service (APNs)
- **Android**: Firebase Cloud Messaging (FCM)
- **Web**: Web Push (VAPID)

#### 3.6.2 Notification Types
| Type | Description | Priority |
|------|-------------|----------|
| game_reminder | Reminder to log game result | Normal |
| deck_suggestion | New card suggestions for deck | Low |
| meta_update | Meta shift detected | Normal |
| price_alert | Card price change alert | High |
| turn_reminder | Your turn (multiplayer) | High |

#### 3.6.3 Configuration
```yaml
# config.yaml
push:
  enabled: true
  
  apns:
    key_id: ${APNS_KEY_ID}
    team_id: ${APNS_TEAM_ID}
    key_path: ./certs/apns.p8
    bundle_id: com.mtgaisuite.app
    
  fcm:
    credentials_path: ./certs/firebase-adminsdk.json
    
  web_push:
    vapid_public_key: ${VAPID_PUBLIC_KEY}
    vapid_private_key: ${VAPID_PRIVATE_KEY}
    vapid_email: admin@example.com
```

#### 3.6.4 Endpoints
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/v1/push/register | Register device token |
| DELETE | /api/v1/push/unregister | Unregister device |
| PUT | /api/v1/push/preferences | Update notification preferences |
| GET | /api/v1/push/preferences | Get notification preferences |

---

## 4. Non-Functional Requirements

### 4.1 Performance
- API response time: < 200ms (p95) for non-AI endpoints
- AI suggestion response: < 5s (p95)
- WebSocket latency: < 100ms
- Support 100 concurrent connections per instance

### 4.2 Security
- All API keys hashed with bcrypt
- HTTPS required in production
- Rate limiting per API key
- Request signing for sensitive operations
- Audit logging for admin actions

### 4.3 Deployment Options
- **Docker**: Single container or docker-compose
- **Kubernetes**: Helm chart provided
- **Bare metal**: Direct installation guide
- **Cloud**: One-click deploy templates (Railway, Render, Fly.io)

### 4.4 Data Storage
- PostgreSQL 15+ for relational data
- Redis for caching and sessions
- pgvector for semantic search embeddings
- SQLite option for single-user deployments

---

## 5. CLI Interface Specification

### 5.1 Command Structure
```
mtg-ai-suite <command> <subcommand> [options] [arguments]
```

### 5.2 Global Options
```
--config, -c    Path to config file (default: ./config.yaml)
--verbose, -v   Verbose output
--quiet, -q     Suppress non-error output
--json          Output in JSON format
--help, -h      Show help
--version       Show version
```

### 5.3 Command Reference

#### apikey
```bash
mtg-ai-suite apikey create [options]
  --name, -n        Client name (required)
  --permissions, -p Comma-separated permissions (default: read,write,ai,push)
  --expires         Expiration duration (e.g., 30d, 1y, never)
  
mtg-ai-suite apikey list [options]
  --active-only     Show only active keys
  --format          Output format (table, json)
  
mtg-ai-suite apikey revoke <key_id>
  --force, -f       Skip confirmation

mtg-ai-suite apikey rotate <key_id>
  --notify          Send push notification to device about new key
```

#### server
```bash
mtg-ai-suite server start [options]
  --port, -p        Port to listen on (default: 8080)
  --host            Host to bind to (default: 0.0.0.0)
  --workers, -w     Number of worker processes (default: auto)
  --daemon, -d      Run as daemon

mtg-ai-suite server stop
mtg-ai-suite server status
mtg-ai-suite server logs [options]
  --follow, -f      Follow log output
  --lines, -n       Number of lines to show (default: 100)

mtg-ai-suite server config <action> [key] [value]
  # Actions: get, set, list, reset
```

#### db
```bash
mtg-ai-suite db migrate [options]
  --target          Target migration version
  --dry-run         Show SQL without executing

mtg-ai-suite db seed [options]
  --source          Data source (scryfall, mtgjson)
  --incremental     Only fetch new data

mtg-ai-suite db backup [options]
  --output, -o      Output file path
  --compress        Compress backup

mtg-ai-suite db restore [options]
  --input, -i       Input file path
  --force           Skip confirmation
```

---

## 6. API Authentication

### 6.1 Request Headers
```http
Authorization: Bearer <api_key>
X-Client-Version: 1.0.0
X-Device-ID: <unique_device_id>
```

### 6.2 Response Format
```json
{
  "success": true,
  "data": { ... },
  "meta": {
    "request_id": "uuid",
    "timestamp": "ISO8601",
    "rate_limit": {
      "remaining": 99,
      "reset_at": "ISO8601"
    }
  }
}
```

### 6.3 Error Response
```json
{
  "success": false,
  "error": {
    "code": "INVALID_API_KEY",
    "message": "The provided API key is invalid or expired",
    "details": { ... }
  },
  "meta": {
    "request_id": "uuid",
    "timestamp": "ISO8601"
  }
}
```

---

## 7. Configuration File

### 7.1 Full Configuration Example
```yaml
# config.yaml
server:
  host: 0.0.0.0
  port: 8080
  workers: auto
  cors_origins:
    - http://localhost:3000
    - https://your-domain.com

database:
  url: postgresql://user:pass@localhost:5432/mtg_ai_suite
  pool_size: 10
  
redis:
  url: redis://localhost:6379
  
llm:
  provider: openai
  api_key: ${LLM_API_KEY}
  model: gpt-4-turbo
  
push:
  enabled: true
  apns:
    key_id: ${APNS_KEY_ID}
    team_id: ${APNS_TEAM_ID}
    key_path: ./certs/apns.p8
    bundle_id: com.mtgaisuite.app
    
security:
  api_key_hash_rounds: 12
  rate_limit:
    requests_per_minute: 60
    ai_requests_per_minute: 10
    
logging:
  level: info
  format: json
  output: stdout
```

---

## 8. Deployment Guide

### 8.1 Docker Deployment
```bash
# Pull image
docker pull ghcr.io/zacharyelston/mtg-ai-suite:latest

# Run with environment variables
docker run -d \
  -p 8080:8080 \
  -e DATABASE_URL=postgresql://... \
  -e LLM_API_KEY=sk-... \
  -v ./data:/app/data \
  ghcr.io/zacharyelston/mtg-ai-suite:latest

# Or use docker-compose
docker-compose up -d
```

### 8.2 Docker Compose
```yaml
version: '3.8'
services:
  mtg-ai-suite:
    image: ghcr.io/zacharyelston/mtg-ai-suite:latest
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@db:5432/mtg
      - REDIS_URL=redis://redis:6379
      - LLM_API_KEY=${LLM_API_KEY}
    depends_on:
      - db
      - redis
    volumes:
      - ./config.yaml:/app/config.yaml
      - ./certs:/app/certs
      
  db:
    image: pgvector/pgvector:pg16
    environment:
      POSTGRES_DB: mtg
      POSTGRES_PASSWORD: postgres
    volumes:
      - pgdata:/var/lib/postgresql/data
      
  redis:
    image: redis:7-alpine
    
volumes:
  pgdata:
```

---

## 9. Milestones

### Phase 1: Core Infrastructure (Weeks 1-2)
- [ ] CLI framework setup
- [ ] API key management
- [ ] Database schema and migrations
- [ ] Basic REST API structure
- [ ] Docker containerization

### Phase 2: Card Database (Weeks 3-4)
- [ ] Scryfall data sync
- [ ] Card search endpoints
- [ ] Vector embeddings for semantic search
- [ ] Caching layer

### Phase 3: Deck & Game Services (Weeks 5-6)
- [ ] Deck CRUD operations
- [ ] Game state management
- [ ] WebSocket implementation
- [ ] Basic analytics

### Phase 4: AI Integration (Weeks 7-8)
- [ ] LLM provider abstraction
- [ ] Play suggestion engine
- [ ] Deck analysis
- [ ] Rules Q&A

### Phase 5: Push Notifications (Week 9)
- [ ] APNs integration
- [ ] FCM integration
- [ ] Notification preferences
- [ ] Device token management

### Phase 6: Polish & Documentation (Week 10)
- [ ] Comprehensive API documentation
- [ ] Deployment guides
- [ ] Performance optimization
- [ ] Security audit

---

## 10. Success Metrics

| Metric | Target |
|--------|--------|
| API uptime | 99.9% |
| Average response time | < 200ms |
| AI suggestion accuracy | > 80% user satisfaction |
| Setup time | < 15 minutes |
| Documentation coverage | 100% of endpoints |
