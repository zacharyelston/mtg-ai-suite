# MTG AI Suite - Replit Project

## Overview
A Magic: The Gathering AI-powered toolkit for card database management, rules processing, game state tracking, and intelligent play suggestions.

## Project Structure
- `frontend/` - Next.js 14 with React 18, TailwindCSS frontend
- `backend/` - Python FastAPI backend (development stage)
- `references/` - Literature review and documentation

## Tech Stack
- **Frontend**: Next.js 14, React 18, TailwindCSS, Zustand
- **Backend**: Python 3.11, FastAPI, uvicorn
- **Planned**: PostgreSQL with pgvector, Redis, Celery

## Running the Application
- Frontend runs on port 5000 via the "Frontend" workflow
- Backend (when needed) should run on port 8000 via uvicorn

## Configuration
- Frontend is configured to allow all dev origins for Replit proxy compatibility
- CORS is enabled for the backend API to accept all origins

## Development Notes
- Use `npm run dev` in frontend/ for development
- Use `uvicorn app.main:app --host localhost --port 8000` in backend/ for API development

## Testing
- **Frontend**: Run `npm test` in frontend/ (Jest + React Testing Library)
- **Backend**: Run `pytest` in backend/ (pytest + pytest-asyncio)

## Features
- **Card Search**: Search MTG cards using Scryfall API with autocomplete
- **Card Display**: Grid view of cards with images
- **Card Details**: Modal view with full card information and pricing
- **Random Card**: Get a random card from the database

## GitHub Issues
See [GitHub Issues](https://github.com/zacharyelston/mtg-ai-suite/issues) for tracked work items.

## Recent Changes
- Configured for Replit environment
- Updated Next.js to run on port 5000 with host 0.0.0.0
- Added allowedDevOrigins configuration for proxy support
- Added functional card search feature with Scryfall integration
- Added frontend unit tests (34 tests total)
- Added backend unit tests (5 tests total)
- Created develop branch review (DEVELOP_BRANCH_REVIEW.md)
- Created GitHub issues #6-#12 from review findings
- **Camera capture feature**: Added CameraCapture and CapturePreview components with WebRTC
- **Backend routes mounted**: Connected card routes to FastAPI main app
- **Health endpoint enhanced**: Added version and timestamp to health response
