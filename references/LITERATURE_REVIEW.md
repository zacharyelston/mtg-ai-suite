# MTG AI Suite - Literature Review & Reference Resources

## Overview
This document catalogs existing tools, APIs, databases, and open-source projects related to Magic: The Gathering that can serve as references for our MTG AI Suite development.

---

## 1. Card Data Sources & APIs

### Scryfall API
- **URL**: https://scryfall.com/docs/api
- **Type**: REST API
- **Description**: The most comprehensive and up-to-date MTG card database API. Provides card data, images, pricing, and search functionality.
- **Key Features**:
  - Endpoint: `https://api.scryfall.com`
  - HTTPS only with TLS 1.2+
  - UTF-8 encoding
  - Bulk data downloads available
  - Card images with various crops (art_crop, normal, large, etc.)
- **Usage Guidelines**:
  - Requires User-Agent and Accept headers
  - Free for non-commercial use under WotC Fan Content Policy
  - Cannot paywall access to Scryfall data
  - Must not cover/crop copyright or artist names on images
- **Rate Limits**: Be a good citizen, avoid hammering the API
- **Relevance**: Primary data source for card information

### MTGJSON
- **URL**: https://mtgjson.com/
- **Type**: Static data files + GraphQL API
- **Description**: Open-source project cataloging all MTG data in portable formats (JSON, CSV, SQLite).
- **Key Features**:
  - Daily updates
  - Card pricing data from partners
  - GraphQL API (beta, Patreon only)
  - TypeScript type definitions
  - Direct download of bulk files
- **Data Models**: Cards, Sets, Decks, Prices
- **Relevance**: Alternative/complementary data source, good for offline use

---

## 2. Rules Engines

### Forge (Card-Forge/forge)
- **URL**: https://github.com/Card-Forge/forge
- **Language**: Java
- **Stars**: 1000+
- **Description**: The most complete open-source MTG rules engine. Full game implementation with AI opponents.
- **Key Features**:
  - Cross-platform (Windows, Mac, Linux, Android)
  - Extensible architecture
  - Single-player and online multiplayer
  - Adventure mode, Quest modes
  - AI opponents with various difficulty levels
- **Relevance**: Reference for rules implementation, AI opponent logic

### mtg-python-engine (wanqizhu/mtg-python-engine)
- **URL**: https://github.com/wanqizhu/mtg-python-engine
- **Language**: Python
- **Description**: Python implementation aiming to replicate the Comprehensive Rules.
- **Key Features**:
  - Card parsing from MTGJSON
  - Ability system (Activated, Triggered, Static)
  - Spell resolution with targeting
  - Combat system
  - Comprehensive Rules implementation tracking
- **Architecture**:
  - Cards inherit from `card.Card` class
  - Abilities stored in card classes
  - Parser for card text files
- **Relevance**: Python-based reference for rules engine, good starting point

### mtghub-engine
- **URL**: http://mtghub-ru.github.io/mtghub-engine/
- **Description**: Open-source MTG rules engine with DSL for custom cards.
- **Key Features**:
  - Coverage of old and new mechanics
  - Simple integration/embedding
  - Custom card/mechanic support via DSL
- **Relevance**: DSL approach for card definitions

---

## 3. AI & Deck Building

### MTG AI Deck Builder (georgejieh/mtg_ai_deck_builder)
- **URL**: https://github.com/georgejieh/mtg_ai_deck_builder
- **Language**: Python
- **Description**: AI model training for competitive Standard deck generation using unsupervised learning.
- **Key Features**:
  - Scryfall integration for Standard-legal cards
  - Deck archetype analysis (Aggro, Midrange, Control, Tempo, Combo)
  - Meta analysis with multiple approaches:
    - Statistical card distribution analysis
    - Pattern matching on card mechanics
    - Semantic similarity clustering
    - Community naming convention analysis
  - Machine learning-based semantic analysis
- **Dependencies**: pandas, numpy, requests, sentence-transformers, scikit-learn, inflect
- **Relevance**: Direct reference for AI deck building approach

### KrakenTheMeta
- **URL**: https://krakenthemeta.com/
- **Type**: Commercial web service
- **Description**: AI-driven MTG deck builder for competitive play.
- **Relevance**: Commercial reference for AI deck building UX

---

## 4. Game State Tracking & Overlays

### MTGATracker
- **URL**: https://github.com/mtgatracker/mtgatracker
- **Language**: Python (backend) + Electron (frontend)
- **Description**: Deck tracker for MTG Arena with in-game overlay.
- **Key Features**:
  - Real-time deck tracking
  - Match history and statistics
  - Win/loss tracking by deck and event
  - Log file parsing
  - WebSocket communication between backend and frontend
- **Architecture**: Python log parser + Electron app
- **Relevance**: Reference for game state tracking, log parsing

### MTGA Pro Tracker
- **URL**: https://mtgarena.pro/mtga-pro-tracker/
- **GitHub**: https://github.com/Razviar/mtgap
- **Description**: Support tool for MTG Arena with overlay and collection tracking.
- **Relevance**: Commercial-grade tracker reference

### MTG Arena Tool
- **URL**: https://mtgatool.com/
- **Description**: Deck tracker with up to 5 simultaneous overlays.
- **Key Features**:
  - Full Deck view
  - Library state tracking
  - Multiple overlay modes
- **Relevance**: UI/UX reference for overlays

---

## 5. Card Recognition & Computer Vision

### MTGScan (fortierq/mtgscan)
- **URL**: https://github.com/fortierq/mtgscan
- **Language**: Python
- **Description**: OCR-based Magic card recognition from images.
- **Key Features**:
  - OCR recognition (Azure Vision API)
  - Fuzzy search with SymSpell
  - MTGJSON dictionary lookup
  - Poetry/pip/Docker installation
- **Dependencies**: Azure Vision API (or other OCR)
- **Relevance**: Reference for card image recognition

### MTG Card Reader (TrifectaIII/MTG-Card-Reader)
- **URL**: https://github.com/TrifectaIII/MTG-Card-Reader
- **Description**: Webcam-based card identification.
- **Relevance**: Real-time card recognition reference

### OpenCV Approach
- **URL**: https://thoughtseize.io/2020/07/10/recognizing-magic-the-gathering-cards-with-cpp-and-opencv/
- **Language**: C++
- **Description**: Card recognition using OpenCV image processing.
- **Relevance**: Computer vision techniques reference

---

## 6. Official Resources

### Wizards of the Coast
- **Comprehensive Rules**: https://magic.wizards.com/en/rules
- **Fan Content Policy**: https://company.wizards.com/fancontentpolicy
- **Gatherer** (official card database): https://gatherer.wizards.com/

---

## Recommended Architecture Based on Review

### Data Layer
- **Primary**: Scryfall API for real-time card data
- **Secondary**: MTGJSON for bulk/offline data
- **Storage**: PostgreSQL for structured data, Vector DB for semantic search

### Rules Engine
- Reference Forge (Java) for completeness
- Build Python-based engine inspired by mtg-python-engine
- Consider DSL approach from mtghub-engine for extensibility

### AI Components
- Deck building: Follow georgejieh/mtg_ai_deck_builder patterns
- Use sentence-transformers for semantic card analysis
- Implement archetype classification

### Game State Tracking
- Log parsing approach from MTGATracker
- WebSocket for real-time updates

### Card Recognition
- Azure Vision API or Tesseract for OCR
- SymSpell for fuzzy matching
- MTGJSON for card dictionary

---

## License Considerations
- Most referenced projects use MIT or GPL licenses
- Scryfall data is free under WotC Fan Content Policy
- MTGJSON is open-source
- Must comply with WotC intellectual property guidelines
