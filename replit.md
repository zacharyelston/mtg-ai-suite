# MTG AI Suite - Replit Project

## Overview
A Magic: The Gathering AI-powered toolkit for card database management, rules processing, game state tracking, and intelligent play suggestions.

## Project Structure
- `frontend/` - Next.js 14 with React 18, TailwindCSS frontend
- `crates/mtg-server/` - Rust Axum backend with Scryfall API integration
- `references/` - Literature review and documentation

## Tech Stack
- **Frontend**: Next.js 14, React 18, TailwindCSS, Zustand
- **Backend**: Rust (Axum, Tokio, Reqwest), Scryfall API integration
- **Planned**: PostgreSQL with pgvector, Redis

## Running the Application
- Frontend runs on port 5000 via the "Frontend" workflow
- Backend runs on port 8000 via the "Backend" workflow
- Frontend proxies API calls to backend via Next.js rewrites

## Architecture
```
Frontend (Next.js :5000) --> Proxy (/api/v1/*) --> Backend (Rust Axum :8000) --> Scryfall API
```

### Backend API Endpoints
- `GET /health` - Health check with version and timestamp
- `GET /api/v1/cards` - List/search cards
- `GET /api/v1/cards/search?q=query` - Search cards by query
- `GET /api/v1/cards/autocomplete?q=query` - Autocomplete card names
- `GET /api/v1/cards/random` - Get random card
- `GET /api/v1/cards/:id` - Get card by ID

## Configuration
- Frontend is configured to allow all dev origins for Replit proxy compatibility
- CORS is enabled for the backend API to accept all origins
- Next.js rewrites proxy /api/v1/* to localhost:8000/api/v1/*

## Development Notes
- Use `npm run dev` in frontend/ for development
- Use `cargo build -p mtg-server` to build Rust backend
- Run backend with: `PORT=8000 ./target/debug/mtg-server`

## Testing
- **Frontend**: Run `npm test` in frontend/ (Jest + React Testing Library)
- **Backend**: Run `cargo test -p mtg-server` for Rust tests

## Features
- **Card Search**: Search MTG cards using Scryfall API with autocomplete
- **Card Display**: Grid view of cards with images
- **Card Details**: Modal view with full card information and pricing
- **Random Card**: Get a random card from the database

## GitHub Issues
See [GitHub Issues](https://github.com/zacharyelston/mtg-ai-suite/issues) for tracked work items.

## Recent Changes
- **Migrated to Rust backend**: Replaced Python FastAPI with Rust Axum server
- Added ScryfallService for API integration (search, autocomplete, random, get by ID)
- Configured Next.js rewrites to proxy API calls to Rust backend
- Backend serves on port 8000, frontend on port 5000
- End-to-end card search flow through Rust backend working
