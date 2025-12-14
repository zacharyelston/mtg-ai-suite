# Multi-Backend Architecture

This document describes the architecture for MTG AI Suite's mobile-first frontend that connects to multiple self-hosted backends via API keys.

## Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Mobile PWA Frontend                                │
│                     (React Native / Next.js PWA)                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                      Connection Manager                                 │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                 │ │
│  │  │ Home Server  │  │ LGS Server   │  │ Cloud Server │                 │ │
│  │  │ 192.168.1.x  │  │ lgs.mtg.com  │  │ mtg.cloud.io │                 │ │
│  │  │ API Key: A   │  │ API Key: B   │  │ API Key: C   │                 │ │
│  │  │ ✓ Active     │  │ ○ Available  │  │ ○ Available  │                 │ │
│  │  └──────────────┘  └──────────────┘  └──────────────┘                 │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                         Feature Router                                  │ │
│  │                                                                         │ │
│  │  Card Search ──────────► Home Server (fastest, local data)             │ │
│  │  AI Suggestions ───────► Cloud Server (best LLM)                       │ │
│  │  Game Tracking ────────► Home Server (privacy)                         │ │
│  │  Price Data ───────────► LGS Server (store prices)                     │ │
│  │  Collection Sync ──────► All Servers (redundancy)                      │ │
│  │                                                                         │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
                    │                    │                    │
                    ▼                    ▼                    ▼
         ┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
         │   Home Server    │  │   LGS Server     │  │   Cloud Server   │
         │   (Self-Hosted)  │  │   (Store-Hosted) │  │   (SaaS)         │
         ├──────────────────┤  ├──────────────────┤  ├──────────────────┤
         │ • Full card DB   │  │ • Store prices   │  │ • Premium LLM    │
         │ • Local AI       │  │ • Event calendar │  │ • Fast inference │
         │ • Private data   │  │ • Trade offers   │  │ • Shared decks   │
         │ • Offline capable│  │ • Tournament reg │  │ • Meta analysis  │
         └──────────────────┘  └──────────────────┘  └──────────────────┘
```

## Key Concepts

### 1. Server Connections

Each backend server is configured independently with its own API key:

```typescript
interface ServerConnection {
  id: string;                    // Unique identifier
  name: string;                  // Display name ("Home Server")
  url: string;                   // Base URL (https://mtg.home.lan)
  apiKey: string;                // Server-specific API key
  capabilities: ServerCapability[]; // What this server provides
  priority: number;              // Preference order (1 = highest)
  isActive: boolean;             // Currently selected
  isOnline: boolean;             // Connection status
  lastSyncedAt: Date;            // Last successful sync
  settings: ServerSettings;      // Server-specific config
}

type ServerCapability =
  | 'cards'           // Card database
  | 'search'          // Search functionality
  | 'ai'              // AI/LLM features
  | 'recognition'     // Card recognition
  | 'decks'           // Deck storage
  | 'games'           // Game tracking
  | 'collection'      // Collection management
  | 'prices'          // Price data
  | 'events'          // Tournament/event data
  | 'social';         // Social features
```

### 2. Feature Routing

The frontend routes requests to the appropriate backend based on:

1. **Capability** - Which servers support the feature
2. **Priority** - User-defined preference order
3. **Availability** - Current online status
4. **Latency** - Response time (prefer faster)
5. **Cost** - Some features may have usage limits

```typescript
interface FeatureRoute {
  feature: string;
  primaryServer: string;      // Preferred server ID
  fallbackServers: string[];  // Backup servers
  cacheStrategy: 'none' | 'local' | 'sync-all';
  offlineMode: 'block' | 'cache' | 'queue';
}

// Example routing configuration
const defaultRoutes: FeatureRoute[] = [
  {
    feature: 'card-search',
    primaryServer: 'home',
    fallbackServers: ['cloud'],
    cacheStrategy: 'local',
    offlineMode: 'cache'
  },
  {
    feature: 'ai-suggestions',
    primaryServer: 'cloud',
    fallbackServers: ['home'],
    cacheStrategy: 'none',
    offlineMode: 'queue'
  },
  {
    feature: 'deck-storage',
    primaryServer: 'home',
    fallbackServers: [],
    cacheStrategy: 'sync-all',
    offlineMode: 'cache'
  }
];
```

### 3. API Key Management

API keys are stored securely on the device and never transmitted to other servers:

```typescript
interface ApiKeyStorage {
  // Secure storage (Keychain/Keystore)
  storeKey(serverId: string, apiKey: string): Promise<void>;
  getKey(serverId: string): Promise<string | null>;
  deleteKey(serverId: string): Promise<void>;

  // Key validation
  validateKey(serverId: string): Promise<KeyValidationResult>;

  // Key rotation
  rotateKey(serverId: string, newKey: string): Promise<void>;
}

interface KeyValidationResult {
  valid: boolean;
  expiresAt?: Date;
  permissions: string[];
  rateLimits: RateLimitInfo;
}
```

### 4. Data Synchronization

Data can be synced across multiple backends:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Sync Manager                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Local Cache (IndexedDB)                                        │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │ Decks    │ Collection │ Games    │ Settings │ Queue    │   │
│  └─────────────────────────────────────────────────────────┘   │
│                          │                                       │
│                          ▼                                       │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                   Sync Engine                            │   │
│  │  • Conflict resolution (last-write-wins / merge)        │   │
│  │  • Delta sync (only changed data)                       │   │
│  │  • Background sync (when online)                        │   │
│  │  • Priority sync (user-initiated)                       │   │
│  └─────────────────────────────────────────────────────────┘   │
│                          │                                       │
│          ┌───────────────┼───────────────┐                      │
│          ▼               ▼               ▼                      │
│     Home Server     LGS Server     Cloud Server                 │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Use Cases

### Use Case 1: Home Player

**Setup:**
- Single self-hosted server at home
- Full card database, local AI (Ollama)
- Complete privacy, no cloud dependency

**Configuration:**
```yaml
servers:
  - name: "Home Server"
    url: "http://192.168.1.100:8080"
    capabilities: [cards, search, ai, recognition, decks, games, collection]
    priority: 1
```

### Use Case 2: LGS Regular

**Setup:**
- Home server for personal data
- LGS server for store prices, events, trades

**Configuration:**
```yaml
servers:
  - name: "Home Server"
    url: "https://mtg.mydomain.com"
    capabilities: [cards, search, ai, decks, games, collection]
    priority: 1

  - name: "Card Kingdom LGS"
    url: "https://api.cardkingdom.com/mtg"
    capabilities: [prices, events, social]
    priority: 2
```

### Use Case 3: Competitive Player

**Setup:**
- Cloud server for best AI (GPT-4, Claude)
- Home server for private deck testing
- Multiple LGS servers for price comparison

**Configuration:**
```yaml
servers:
  - name: "MTG Cloud Pro"
    url: "https://api.mtgcloud.io"
    capabilities: [ai, search]
    priority: 1
    routes:
      ai-suggestions: primary

  - name: "Home Lab"
    url: "https://mtg.home.lan"
    capabilities: [cards, decks, games, collection, recognition]
    priority: 2
    routes:
      deck-storage: primary
      game-tracking: primary

  - name: "TCGPlayer"
    url: "https://api.tcgplayer.com"
    capabilities: [prices]
    priority: 3

  - name: "Card Kingdom"
    url: "https://api.cardkingdom.com"
    capabilities: [prices]
    priority: 4
```

### Use Case 4: Tournament Organizer

**Setup:**
- Central tournament server
- Players connect with limited API keys
- Real-time game state sync

**Configuration:**
```yaml
servers:
  - name: "GP Vegas 2024"
    url: "https://events.cfbevents.com/gpvegas"
    capabilities: [events, games, social]
    priority: 1
    permissions:
      - read:events
      - write:games
      - read:pairings
```

## API Design

### Server Discovery

```
GET /api/v1/server/info
Authorization: Bearer <api_key>

Response:
{
  "name": "Home Server",
  "version": "1.0.0",
  "capabilities": ["cards", "search", "ai", "decks"],
  "limits": {
    "requests_per_minute": 60,
    "ai_requests_per_day": 100
  },
  "features": {
    "ai_provider": "ollama",
    "ai_model": "llama3",
    "card_database": "scryfall",
    "last_updated": "2024-01-15T00:00:00Z"
  }
}
```

### Multi-Server Request

The frontend can query multiple servers and merge results:

```typescript
async function searchCards(query: string): Promise<Card[]> {
  const servers = getServersWithCapability('search');

  // Query all servers in parallel
  const results = await Promise.allSettled(
    servers.map(server =>
      fetchWithTimeout(
        `${server.url}/api/v1/cards/search?q=${query}`,
        { headers: { Authorization: `Bearer ${server.apiKey}` } },
        5000 // 5s timeout
      )
    )
  );

  // Merge and deduplicate results
  return mergeCardResults(results);
}
```

### Offline Queue

When offline, requests are queued for later:

```typescript
interface QueuedRequest {
  id: string;
  serverId: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  path: string;
  body?: unknown;
  createdAt: Date;
  retryCount: number;
  priority: 'high' | 'normal' | 'low';
}

// Queue is processed when back online
async function processQueue(): Promise<void> {
  const queue = await getQueuedRequests();

  for (const request of queue.sort(byPriority)) {
    try {
      await executeRequest(request);
      await removeFromQueue(request.id);
    } catch (error) {
      await incrementRetryCount(request.id);
    }
  }
}
```

## Security Considerations

### API Key Security

1. **Storage**: Keys stored in platform secure storage (iOS Keychain, Android Keystore)
2. **Transmission**: Always HTTPS, keys in Authorization header only
3. **Scope**: Keys can have limited permissions per server
4. **Rotation**: Support for key rotation without losing data
5. **Revocation**: Server can revoke keys, client handles gracefully

### Data Privacy

1. **Local-first**: All data cached locally, sync is optional
2. **Server isolation**: Data from one server never sent to another
3. **Encryption**: Local database encrypted at rest
4. **Audit log**: Track which server accessed what data

### Network Security

1. **Certificate pinning**: Optional for self-hosted servers
2. **Timeout handling**: Prevent hanging on unresponsive servers
3. **Rate limiting**: Respect server rate limits
4. **Retry backoff**: Exponential backoff on failures

## Implementation Phases

### Phase 1: Single Server (Current)
- [x] Basic server connection
- [x] API key authentication
- [x] Core API endpoints
- [ ] Offline caching

### Phase 2: Multi-Server Foundation
- [ ] Server connection manager UI
- [ ] Multiple server configuration
- [ ] Server capability discovery
- [ ] Basic routing (primary/fallback)

### Phase 3: Smart Routing
- [ ] Feature-based routing
- [ ] Latency-aware routing
- [ ] Cost-aware routing
- [ ] Automatic failover

### Phase 4: Advanced Sync
- [ ] Multi-server data sync
- [ ] Conflict resolution
- [ ] Selective sync
- [ ] Real-time sync (WebSocket)

### Phase 5: Federation
- [ ] Server-to-server communication
- [ ] Shared deck publishing
- [ ] Cross-server tournaments
- [ ] Decentralized identity

## Configuration Files

### Client Config (stored on device)

```json
{
  "servers": [
    {
      "id": "home-server",
      "name": "Home Server",
      "url": "https://mtg.home.lan",
      "isActive": true,
      "priority": 1,
      "capabilities": ["cards", "search", "ai", "decks", "games"],
      "settings": {
        "syncInterval": 300,
        "cacheSize": 100
      }
    }
  ],
  "routing": {
    "card-search": { "primary": "home-server" },
    "ai-suggestions": { "primary": "home-server" }
  },
  "sync": {
    "enabled": true,
    "wifiOnly": false,
    "interval": 300
  }
}
```

### Server Config (on backend)

```yaml
# config.yaml on self-hosted server
server:
  name: "Home Server"
  public_url: "https://mtg.home.lan"

capabilities:
  cards: true
  search: true
  ai:
    enabled: true
    provider: ollama
    model: llama3
  recognition:
    enabled: true
    model: local
  decks: true
  games: true
  collection: true
  prices: false  # No price data on this server

api_keys:
  - name: "Mobile App"
    permissions: ["read", "write"]
    rate_limit: 60  # requests per minute

  - name: "Read-Only Viewer"
    permissions: ["read"]
    rate_limit: 30
```

## Summary

This architecture enables:

1. **Privacy**: Keep sensitive data on your own server
2. **Flexibility**: Mix and match servers for different features
3. **Resilience**: Automatic failover when servers are unavailable
4. **Offline**: Full functionality without network
5. **Federation**: Future support for server-to-server features

The mobile app acts as a smart client that orchestrates multiple backends, giving users complete control over where their data lives and which services they use.
