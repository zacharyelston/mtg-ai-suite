# MTG AI Suite - Frontend Product Requirements Document

## Overview

The MTG AI Suite Frontend is a mobile-first Progressive Web App (PWA) that connects to a self-hosted backend server. It provides an intuitive interface for deck building, game tracking, and receiving AI-powered play suggestions with real-time push notifications.

---

## 1. Product Vision

### 1.1 Purpose
Deliver a beautiful, responsive mobile application that MTG players can use at the table during games, for deck building on the go, and for receiving timely notifications about their MTG activities.

### 1.2 Target Users
- **Primary**: MTG players who want AI assistance during games
- **Secondary**: Deck builders seeking optimization suggestions
- **Tertiary**: Collectors tracking their card inventory

### 1.3 Key Value Propositions
- Works offline with sync when connected
- Native-like mobile experience via PWA
- Real-time push notifications
- Connect to any self-hosted backend
- No app store required (installable PWA)

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     MTG AI Suite Frontend                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Application Shell                        │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐ │ │
│  │  │  Header  │  │Navigation│  │  Toast   │  │   Modal    │ │ │
│  │  │   Bar    │  │   Menu   │  │ Manager  │  │  Manager   │ │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                      Page Views                             │ │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────┐ │ │
│  │  │  Home   │ │  Cards  │ │  Decks  │ │  Game   │ │ More │ │ │
│  │  │ Screen  │ │ Browser │ │ Manager │ │ Tracker │ │      │ │ │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └──────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    State Management                         │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐ │ │
│  │  │  Zustand │  │  React   │  │  Offline │  │   Push     │ │ │
│  │  │  Store   │  │  Query   │  │  Storage │  │  Handler   │ │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Service Layer                            │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────┐ │ │
│  │  │   API    │  │WebSocket │  │  Service │  │   Push     │ │ │
│  │  │  Client  │  │  Client  │  │  Worker  │  │  Service   │ │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └────────────┘ │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  Self-Hosted    │
                    │    Backend      │
                    │  (User's Server)│
                    └─────────────────┘
```

---

## 3. Technical Stack

### 3.1 Core Technologies
| Technology | Purpose | Version |
|------------|---------|---------|
| Next.js | React framework with SSR/SSG | 14.x |
| React | UI library | 18.x |
| TypeScript | Type safety | 5.x |
| TailwindCSS | Styling | 3.x |
| Zustand | State management | 4.x |
| React Query | Server state & caching | 5.x |
| Workbox | Service worker & PWA | 7.x |

### 3.2 UI Components
| Library | Purpose |
|---------|---------|
| shadcn/ui | Base component library |
| Radix UI | Accessible primitives |
| Lucide React | Icon library |
| Framer Motion | Animations |
| React Hook Form | Form handling |
| Zod | Schema validation |

### 3.3 Mobile-Specific
| Feature | Implementation |
|---------|----------------|
| PWA | next-pwa + Workbox |
| Push Notifications | Web Push API |
| Offline Storage | IndexedDB via Dexie.js |
| Camera Access | MediaDevices API (`getUserMedia`) |
| Photo Upload | File Input API |
| Video Stream | MediaStream API |
| Screen Capture | Screen Capture API (`getDisplayMedia`) |
| Image Processing | Canvas API + Web Workers |
| Barcode Detection | BarcodeDetector API |
| Haptic Feedback | Vibration API |

### 3.4 Image Processing Libraries
| Library | Purpose |
|---------|---------|
| Tesseract.js | Client-side OCR for card text |
| TensorFlow.js | On-device ML for card recognition |
| OpenCV.js | Image preprocessing |
| Jimp | Image manipulation |
| Compressor.js | Image compression before upload |

---

## 4. Functional Requirements

### 4.1 Onboarding & Connection Setup

#### 4.1.1 First Launch Flow
```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Welcome      │     │  Enter Server   │     │   Enter API     │
│    Screen       │ ──▶ │      URL        │ ──▶ │      Key        │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Enable       │     │   Connection    │     │    Home         │
│    Push?        │ ◀── │   Successful    │ ◀── │   Verified      │
│                 │     │                 │     │                 │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

#### 4.1.2 Connection Configuration
```typescript
interface ServerConnection {
  id: string;
  name: string;           // "Home Server", "Tournament Server"
  url: string;            // https://mtg.myserver.com
  apiKey: string;         // Stored securely
  isActive: boolean;
  lastConnected: Date;
  pushEnabled: boolean;
  deviceToken?: string;
}
```

#### 4.1.3 Multi-Server Support
- Users can configure multiple backend servers
- Quick switch between servers
- Each server has its own API key
- Sync status indicator per server

### 4.2 Home Screen

#### 4.2.1 Layout
```
┌─────────────────────────────────────┐
│  MTG AI Suite          ⚙️  🔔       │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │  Quick Actions              │   │
│  │  [New Game] [Search Cards]  │   │
│  │  [My Decks] [AI Assistant]  │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │  Recent Decks               │   │
│  │  ┌─────┐ ┌─────┐ ┌─────┐   │   │
│  │  │Deck1│ │Deck2│ │Deck3│   │   │
│  │  └─────┘ └─────┘ └─────┘   │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │  Recent Games               │   │
│  │  Win vs. Opponent - 2h ago  │   │
│  │  Loss vs. Opponent - 1d ago │   │
│  └─────────────────────────────┘   │
│                                     │
├─────────────────────────────────────┤
│  🏠    🃏    📚    🎮    ⋯        │
└─────────────────────────────────────┘
```

### 4.3 Card Browser

#### 4.3.1 Features
- **Search**: Full-text and semantic search
- **Filters**: Colors, types, CMC, sets, rarity, legality
- **Sort**: Name, CMC, price, release date
- **View Modes**: Grid (images), List (compact), Table
- **Card Details**: Full card info, rulings, prices, legality

#### 4.3.2 Card Detail View
```
┌─────────────────────────────────────┐
│  ←  Lightning Bolt           ♡  📤 │
├─────────────────────────────────────┤
│                                     │
│         ┌───────────────┐          │
│         │               │          │
│         │  [Card Image] │          │
│         │               │          │
│         └───────────────┘          │
│                                     │
│  Lightning Bolt                {R} │
│  Instant                           │
│  ─────────────────────────────────  │
│  Lightning Bolt deals 3 damage to  │
│  any target.                       │
│  ─────────────────────────────────  │
│                                     │
│  ┌─────────┬─────────┬─────────┐   │
│  │ Prices  │ Rulings │ Legality│   │
│  └─────────┴─────────┴─────────┘   │
│                                     │
│  [Add to Deck ▼]  [Ask AI About]   │
│                                     │
└─────────────────────────────────────┘
```

#### 4.3.3 Offline Support
- Cache recently viewed cards
- Download full set data for offline
- Queue searches for when online

### 4.4 Deck Manager

#### 4.4.1 Deck List View
```
┌─────────────────────────────────────┐
│  My Decks                    [+ New]│
├─────────────────────────────────────┤
│  🔍 Search decks...                 │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🔴⚪ Boros Aggro            │   │
│  │ Standard • 60 cards • 75%   │   │
│  │ Last edited: 2 hours ago    │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🔵⚫ Dimir Control          │   │
│  │ Modern • 60 cards • 82%     │   │
│  │ Last edited: 1 day ago      │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🟢⚫🔴 Jund Midrange        │   │
│  │ Legacy • 60 cards • 91%     │   │
│  │ Last edited: 3 days ago     │   │
│  └─────────────────────────────┘   │
│                                     │
└─────────────────────────────────────┘
```

#### 4.4.2 Deck Editor
```
┌─────────────────────────────────────┐
│  ←  Boros Aggro              ⋮     │
├─────────────────────────────────────┤
│  ┌─────────┬─────────┬─────────┐   │
│  │  Cards  │ Analysis│   AI    │   │
│  └─────────┴─────────┴─────────┘   │
├─────────────────────────────────────┤
│  Creatures (24)              ▼     │
│  ├─ 4x Monastery Swiftspear        │
│  ├─ 4x Soul-Scar Mage              │
│  ├─ 4x Goblin Guide                │
│  └─ ...                            │
│                                     │
│  Instants (12)               ▼     │
│  ├─ 4x Lightning Bolt              │
│  ├─ 4x Boros Charm                 │
│  └─ ...                            │
│                                     │
│  Lands (20)                  ▼     │
│  └─ ...                            │
│                                     │
├─────────────────────────────────────┤
│  [+ Add Card]  [📷 Scan]  [📋 Import]│
└─────────────────────────────────────┘
```

#### 4.4.3 Deck Analysis Tab
```
┌─────────────────────────────────────┐
│  Mana Curve                        │
│  ┌─────────────────────────────┐   │
│  │    ██                       │   │
│  │    ██ ██                    │   │
│  │ ██ ██ ██ ██                 │   │
│  │ ██ ██ ██ ██ ██              │   │
│  │ 0  1  2  3  4  5  6  7+     │   │
│  └─────────────────────────────┘   │
│                                     │
│  Color Distribution                │
│  🔴 Red: 65%  ████████████░░░     │
│  ⚪ White: 35% ██████░░░░░░░░     │
│                                     │
│  Card Types                        │
│  Creatures: 24 (40%)               │
│  Instants: 12 (20%)                │
│  Sorceries: 4 (7%)                 │
│  Lands: 20 (33%)                   │
│                                     │
│  Archetype: Aggro (94% confidence) │
│                                     │
└─────────────────────────────────────┘
```

#### 4.4.4 AI Suggestions Tab
```
┌─────────────────────────────────────┐
│  AI Analysis                       │
├─────────────────────────────────────┤
│  ┌─────────────────────────────┐   │
│  │ 💡 Suggestions              │   │
│  │                             │   │
│  │ Consider adding:            │   │
│  │ • Eidolon of the Great Revel│   │
│  │   "Strong in aggro builds"  │   │
│  │   [+ Add]                   │   │
│  │                             │   │
│  │ Consider removing:          │   │
│  │ • Firebrand Archer          │   │
│  │   "Too slow for this curve" │   │
│  │   [- Remove]                │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🎯 Weaknesses               │   │
│  │ • Vulnerable to lifegain    │   │
│  │ • Limited card draw         │   │
│  │ • Weak to board wipes       │   │
│  └─────────────────────────────┘   │
│                                     │
│  [🔄 Refresh Analysis]             │
│                                     │
└─────────────────────────────────────┘
```

### 4.5 Game Tracker

#### 4.5.1 New Game Setup
```
┌─────────────────────────────────────┐
│  New Game                          │
├─────────────────────────────────────┤
│                                     │
│  Select Your Deck                  │
│  ┌─────────────────────────────┐   │
│  │ Boros Aggro              ▼ │   │
│  └─────────────────────────────┘   │
│                                     │
│  Format                            │
│  ┌─────────────────────────────┐   │
│  │ Standard                 ▼ │   │
│  └─────────────────────────────┘   │
│                                     │
│  Opponent (optional)               │
│  ┌─────────────────────────────┐   │
│  │ Enter name...               │   │
│  └─────────────────────────────┘   │
│                                     │
│  Starting Life                     │
│  ┌─────────────────────────────┐   │
│  │ 20                       ▼ │   │
│  └─────────────────────────────┘   │
│                                     │
│         [Start Game]               │
│                                     │
└─────────────────────────────────────┘
```

#### 4.5.2 Active Game View
```
┌─────────────────────────────────────┐
│  Game in Progress        ⏸️  ⋮    │
├─────────────────────────────────────┤
│                                     │
│  Opponent                          │
│  ┌─────────────────────────────┐   │
│  │         ❤️ 20               │   │
│  │      [-1] [-5] [+1]         │   │
│  └─────────────────────────────┘   │
│                                     │
│  ─────────── Turn 4 ───────────    │
│                                     │
│  You                               │
│  ┌─────────────────────────────┐   │
│  │         ❤️ 17               │   │
│  │      [-1] [-5] [+1]         │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🧠 AI Suggestions           │   │
│  │ "Attack with all creatures" │   │
│  │ "Hold Lightning Bolt"       │   │
│  │           [More ▼]          │   │
│  └─────────────────────────────┘   │
│                                     │
│  [📝 Log Action]  [🏆 End Game]    │
│                                     │
└─────────────────────────────────────┘
```

#### 4.5.3 Game Log
```
┌─────────────────────────────────────┐
│  Game Log                    [+ Add]│
├─────────────────────────────────────┤
│                                     │
│  Turn 4                            │
│  ├─ You: Cast Lightning Bolt       │
│  ├─ Opponent: -3 life (20→17)      │
│  └─ You: Attack for 5              │
│                                     │
│  Turn 3                            │
│  ├─ Opponent: Cast Counterspell    │
│  └─ You: Spell countered           │
│                                     │
│  Turn 2                            │
│  ├─ You: Cast Monastery Swiftspear │
│  └─ You: Attack for 1              │
│                                     │
│  Turn 1                            │
│  └─ You: Cast Soul-Scar Mage       │
│                                     │
└─────────────────────────────────────┘
```

### 4.6 AI Assistant

#### 4.6.1 Chat Interface
```
┌─────────────────────────────────────┐
│  AI Assistant                      │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🤖 How can I help you with │   │
│  │    Magic today?             │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 👤 What's the best way to   │   │
│  │    sideboard against        │   │
│  │    control decks?           │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🤖 Against control, you'll  │   │
│  │    want to bring in...      │   │
│  │    • Veil of Summer         │   │
│  │    • Chandra, Awakened      │   │
│  │    ...                      │   │
│  └─────────────────────────────┘   │
│                                     │
├─────────────────────────────────────┤
│  ┌─────────────────────────┐ [Send]│
│  │ Ask about MTG...         │      │
│  └─────────────────────────┘       │
└─────────────────────────────────────┘
```

#### 4.6.2 Quick Actions
- "Analyze my current board"
- "What should I play next?"
- "Explain this card interaction"
- "Help me sideboard"
- "Draft pick advice"

### 4.7 Image Capture & Card Recognition

#### 4.7.1 Capture Modes

| Mode | Description | Use Case |
|------|-------------|----------|
| **Single Card Scan** | Point camera at one card | Quick card lookup |
| **Batch Scan** | Continuous scanning mode | Collection intake |
| **Board State** | Wide shot of battlefield | AI board analysis |
| **Hand Photo** | Capture opening hand | Mulligan advice |
| **Deck Import** | Photo of card pile/list | Import physical deck |
| **Screen Capture** | Grab from MTG Arena/MTGO | Digital game tracking |
| **Binder Scan** | Scan binder pages | Collection cataloging |

#### 4.7.2 Single Card Scanner
```
┌─────────────────────────────────────┐
│  Scan Card                    ✕    │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │                             │   │
│  │                             │   │
│  │     [Camera Viewfinder]     │   │
│  │                             │   │
│  │    ┌─────────────────┐     │   │
│  │    │ Align card here │     │   │
│  │    └─────────────────┘     │   │
│  │                             │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🔍 Detected: Lightning Bolt │   │
│  │    Confidence: 98%          │   │
│  │    [View Card] [Add to Deck]│   │
│  └─────────────────────────────┘   │
│                                     │
│  [📷 Capture]  [🔦 Flash]  [⚙️]   │
│                                     │
└─────────────────────────────────────┘
```

#### 4.7.3 Batch Scan Mode
```
┌─────────────────────────────────────┐
│  Batch Scan (12 cards)        ✕    │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │     [Camera Viewfinder]     │   │
│  │                             │   │
│  │  ✓ Card detected - scanning │   │
│  └─────────────────────────────┘   │
│                                     │
│  Recent Scans:                     │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐  │
│  │ ✓   │ │ ✓   │ │ ✓   │ │ ✓   │  │
│  │Bolt │ │Guide│ │Charm│ │Mage │  │
│  └─────┘ └─────┘ └─────┘ └─────┘  │
│                                     │
│  [Pause]  [Review All]  [Done]     │
│                                     │
└─────────────────────────────────────┘
```

#### 4.7.4 Board State Capture
```
┌─────────────────────────────────────┐
│  Capture Board State          ✕    │
├─────────────────────────────────────┤
│                                     │
│  ┌─────────────────────────────┐   │
│  │                             │   │
│  │  [Wide Camera Viewfinder]   │   │
│  │                             │   │
│  │  Tip: Include all cards     │   │
│  │  in play for best analysis  │   │
│  │                             │   │
│  └─────────────────────────────┘   │
│                                     │
│  [📷 Capture Board]                │
│                                     │
│  After capture:                    │
│  • AI will identify all cards      │
│  • Analyze board state             │
│  • Suggest optimal plays           │
│                                     │
└─────────────────────────────────────┘
```

#### 4.7.5 Screen Capture (Desktop/Tablet)
```
┌─────────────────────────────────────┐
│  Screen Capture               ✕    │
├─────────────────────────────────────┤
│                                     │
│  Capture game state from:          │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🖥️ MTG Arena               │   │
│  │    Capture current game     │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 🖥️ MTG Online              │   │
│  │    Capture current game     │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 📋 Paste Screenshot         │   │
│  │    From clipboard           │   │
│  └─────────────────────────────┘   │
│                                     │
│  ┌─────────────────────────────┐   │
│  │ 📁 Upload Image             │   │
│  │    From device              │   │
│  └─────────────────────────────┘   │
│                                     │
└─────────────────────────────────────┘
```

#### 4.7.6 Recognition Pipeline
```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Capture │───▶│ Preprocess│───▶│   OCR/   │───▶│  Fuzzy   │
│  Image   │    │  Image   │    │   ML     │    │  Match   │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
                     │                │               │
                     ▼                ▼               ▼
              ┌──────────┐    ┌──────────┐    ┌──────────┐
              │ Crop &   │    │ Extract  │    │ Scryfall │
              │ Rotate   │    │ Card Name│    │ Lookup   │
              └──────────┘    └──────────┘    └──────────┘
```

#### 4.7.7 Processing Options

| Setting | Options | Default |
|---------|---------|---------|
| Processing Location | On-device / Server | On-device |
| OCR Engine | Tesseract.js / Azure Vision | Tesseract.js |
| Auto-capture | On / Off | On |
| Haptic Feedback | On / Off | On |
| Sound Effects | On / Off | On |
| Save Original Images | Yes / No | No |

#### 4.7.8 Supported Input Sources

| Source | API | Notes |
|--------|-----|-------|
| Rear Camera | `getUserMedia({ video: { facingMode: 'environment' } })` | Primary for card scanning |
| Front Camera | `getUserMedia({ video: { facingMode: 'user' } })` | Fallback |
| Screen Share | `getDisplayMedia()` | For MTG Arena/MTGO |
| File Upload | `<input type="file" accept="image/*">` | Gallery photos |
| Clipboard | `navigator.clipboard.read()` | Pasted screenshots |
| Drag & Drop | HTML5 Drag and Drop API | Desktop convenience |

#### 4.7.9 Image Quality Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| Resolution | 640x480 | 1280x720+ |
| Card Size in Frame | 20% of frame | 40-60% of frame |
| Lighting | Readable text | Even, no glare |
| Focus | Card name legible | Sharp edges |
| Angle | < 30° tilt | Flat/perpendicular |

### 4.8 Settings & Configuration

#### 4.8.1 Settings Screen
```
┌─────────────────────────────────────┐
│  Settings                          │
├─────────────────────────────────────┤
│                                     │
│  Server Connection                 │
│  ┌─────────────────────────────┐   │
│  │ 🟢 Home Server              │   │
│  │    mtg.myserver.com      ▶ │   │
│  └─────────────────────────────┘   │
│  [+ Add Server]                    │
│                                     │
│  ─────────────────────────────────  │
│                                     │
│  Notifications                     │
│  Push Notifications         [🔘]   │
│  Game Reminders            [🔘]   │
│  Deck Suggestions          [🔘]   │
│  Price Alerts              [🔘]   │
│                                     │
│  ─────────────────────────────────  │
│                                     │
│  Display                           │
│  Theme                    [Auto ▼] │
│  Card Size                [Med  ▼] │
│  Animations               [🔘]    │
│                                     │
│  ─────────────────────────────────  │
│                                     │
│  Data                              │
│  [Clear Cache]  [Export Data]      │
│                                     │
└─────────────────────────────────────┘
```

---

## 5. Push Notifications

### 5.1 Notification Types

| Type | Title | Body Example | Action |
|------|-------|--------------|--------|
| game_reminder | "Log your game?" | "You started a game 2 hours ago" | Open game tracker |
| deck_suggestion | "New suggestion" | "Try adding Ragavan to your deck" | Open deck |
| meta_update | "Meta Alert" | "Aggro decks rising in Standard" | Open analysis |
| price_alert | "Price Change" | "Lightning Bolt dropped 20%" | Open card |
| turn_reminder | "Your Turn" | "It's your turn in Game #123" | Open game |

### 5.2 Notification Preferences
```typescript
interface NotificationPreferences {
  enabled: boolean;
  gameReminders: boolean;
  deckSuggestions: boolean;
  metaUpdates: boolean;
  priceAlerts: boolean;
  turnReminders: boolean;
  quietHours: {
    enabled: boolean;
    start: string;  // "22:00"
    end: string;    // "08:00"
  };
}
```

### 5.3 Implementation
- Request permission on first launch (after onboarding)
- Store device token on backend via `/api/v1/push/register`
- Handle notification clicks to deep link into app
- Badge count for unread notifications

---

## 6. Offline Support

### 6.1 Offline-First Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                      Application                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐  │
│  │   UI Layer   │───▶│ State Store  │◀───│  API Client  │  │
│  └──────────────┘    └──────┬───────┘    └──────┬───────┘  │
│                             │                    │          │
│                             ▼                    ▼          │
│                    ┌──────────────┐    ┌──────────────┐    │
│                    │  IndexedDB   │    │   Network    │    │
│                    │  (Dexie.js)  │    │   Request    │    │
│                    └──────────────┘    └──────────────┘    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Cached Data
| Data Type | Storage | Sync Strategy |
|-----------|---------|---------------|
| Card database | IndexedDB | Background sync daily |
| User decks | IndexedDB | Sync on change + periodic |
| Game history | IndexedDB | Sync on change |
| Card images | Cache API | LRU, max 500MB |
| AI responses | Memory | No persistence |

### 6.3 Offline Actions Queue
```typescript
interface OfflineAction {
  id: string;
  type: 'create_deck' | 'update_deck' | 'log_game' | 'end_game';
  payload: any;
  createdAt: Date;
  retryCount: number;
}
```

### 6.4 Sync Indicators
- Connection status in header
- "Pending sync" badge on items
- "Last synced: X minutes ago"
- Manual sync button

---

## 7. PWA Configuration

### 7.1 Manifest
```json
{
  "name": "MTG AI Suite",
  "short_name": "MTG AI",
  "description": "AI-powered Magic: The Gathering companion",
  "start_url": "/",
  "display": "standalone",
  "orientation": "portrait",
  "background_color": "#1a1a2e",
  "theme_color": "#4a90d9",
  "icons": [
    {
      "src": "/icons/icon-192.png",
      "sizes": "192x192",
      "type": "image/png",
      "purpose": "any maskable"
    },
    {
      "src": "/icons/icon-512.png",
      "sizes": "512x512",
      "type": "image/png",
      "purpose": "any maskable"
    }
  ],
  "screenshots": [
    {
      "src": "/screenshots/home.png",
      "sizes": "1080x1920",
      "type": "image/png",
      "form_factor": "narrow"
    }
  ],
  "shortcuts": [
    {
      "name": "New Game",
      "url": "/game/new",
      "icon": "/icons/game.png"
    },
    {
      "name": "Search Cards",
      "url": "/cards",
      "icon": "/icons/search.png"
    }
  ]
}
```

### 7.2 Service Worker Strategy
- **App Shell**: Cache first
- **API Requests**: Network first, fallback to cache
- **Card Images**: Cache first, background update
- **Static Assets**: Cache first

### 7.3 Install Prompt
- Show custom install banner after 2nd visit
- Explain benefits of installing
- Track install conversion

---

## 8. Design System

### 8.1 Color Palette
```css
:root {
  /* MTG Mana Colors */
  --mana-white: #F8F6D8;
  --mana-blue: #0E68AB;
  --mana-black: #150B00;
  --mana-red: #D3202A;
  --mana-green: #00733E;
  --mana-colorless: #CAC5C0;
  
  /* App Colors */
  --primary: #4A90D9;
  --secondary: #6C5CE7;
  --background: #1A1A2E;
  --surface: #25253A;
  --text-primary: #FFFFFF;
  --text-secondary: #A0A0B0;
  --success: #00B894;
  --warning: #FDCB6E;
  --error: #E74C3C;
}
```

### 8.2 Typography
```css
:root {
  --font-display: 'Beleren', serif;  /* MTG-style for headers */
  --font-body: 'Inter', sans-serif;
  
  --text-xs: 0.75rem;
  --text-sm: 0.875rem;
  --text-base: 1rem;
  --text-lg: 1.125rem;
  --text-xl: 1.25rem;
  --text-2xl: 1.5rem;
  --text-3xl: 1.875rem;
}
```

### 8.3 Spacing
```css
:root {
  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  --space-6: 1.5rem;
  --space-8: 2rem;
  --space-12: 3rem;
}
```

### 8.4 Components
- Buttons (primary, secondary, ghost, icon)
- Cards (elevated, outlined)
- Inputs (text, select, checkbox, toggle)
- Navigation (bottom tabs, header)
- Modals (full screen on mobile)
- Toast notifications
- Loading states (skeleton, spinner)

---

## 9. Accessibility

### 9.1 Requirements
- WCAG 2.1 AA compliance
- Screen reader support
- Keyboard navigation
- Reduced motion support
- High contrast mode
- Minimum touch target: 44x44px

### 9.2 Implementation
- Semantic HTML
- ARIA labels
- Focus management
- Color contrast ratios ≥ 4.5:1
- `prefers-reduced-motion` media query

---

## 10. Performance Targets

| Metric | Target | Measurement |
|--------|--------|-------------|
| First Contentful Paint | < 1.5s | Lighthouse |
| Largest Contentful Paint | < 2.5s | Lighthouse |
| Time to Interactive | < 3.5s | Lighthouse |
| Cumulative Layout Shift | < 0.1 | Lighthouse |
| First Input Delay | < 100ms | Lighthouse |
| Bundle Size (initial) | < 200KB | Webpack |
| Lighthouse Score | > 90 | Lighthouse |

---

## 11. Security

### 11.1 API Key Storage
- Store in secure storage (Keychain/Keystore via Capacitor if native)
- For PWA: encrypted in IndexedDB
- Never expose in URLs or logs

### 11.2 Network Security
- HTTPS only
- Certificate pinning (optional)
- Request signing for sensitive operations

### 11.3 Data Protection
- Clear sensitive data on logout
- Secure wipe option
- No analytics without consent

---

## 12. Milestones

### Phase 1: Foundation (Weeks 1-2)
- [ ] Project setup with Next.js 14
- [ ] Design system implementation
- [ ] PWA configuration
- [ ] Server connection flow
- [ ] Basic navigation

### Phase 2: Core Features (Weeks 3-5)
- [ ] Card browser with search
- [ ] Deck manager CRUD
- [ ] Deck analysis views
- [ ] Offline storage setup

### Phase 3: Game Tracking (Weeks 6-7)
- [ ] Game tracker UI
- [ ] Life counter
- [ ] Game log
- [ ] WebSocket integration

### Phase 4: AI Integration (Week 8)
- [ ] AI chat interface
- [ ] Play suggestions UI
- [ ] Deck suggestions UI

### Phase 5: Push & Polish (Weeks 9-10)
- [ ] Push notification integration
- [ ] Offline sync refinement
- [ ] Performance optimization
- [ ] Accessibility audit
- [ ] Beta testing

---

## 13. Success Metrics

| Metric | Target |
|--------|--------|
| Lighthouse PWA Score | 100 |
| Install Rate | > 30% of returning users |
| Offline Usage | > 20% of sessions |
| Push Opt-in Rate | > 50% |
| Daily Active Users | Track growth |
| Session Duration | > 5 minutes |
| Crash-free Sessions | > 99.5% |
