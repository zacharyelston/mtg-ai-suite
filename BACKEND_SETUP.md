# MTG AI Suite Backend Setup

This document explains how to set up the Rust backend as a separate Replit app.

## Overview

The MTG AI Suite uses a separate backend server for handling:
- Camera capture submissions
- Card recognition processing
- API endpoints for the frontend

## Setting up the Backend Replit App

### 1. Create a New Replit App

1. Go to Replit and create a new app
2. Import from the same GitHub repository: `https://github.com/zacharyelston/mtg-ai-suite`
3. Choose "Rust" as the language if prompted

### 2. Configure the Replit App

Create a workflow to run the Rust server:

**Workflow name:** `Backend`
**Command:** `cargo run --release --bin mtg-server`
**Port:** 5000

### 3. Configure Deployment

For deployment, use these settings:
- **Build command:** `cargo build --release --bin mtg-server`
- **Run command:** `./target/release/mtg-server`
- **Deployment type:** Autoscale or VM

### 4. Environment Variables

Set these environment variables in the backend Replit app:

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | `5000` |
| `RUST_LOG` | Log level | `info` |
| `SECRET_KEY` | JWT signing key | (generate a secure key) |
| `LLM_API_KEY` | OpenAI/Anthropic API key | (optional) |
| `LLM_PROVIDER` | LLM provider | `openai` |

### 5. Test the Backend

Once running, test the health endpoint:
```
curl https://your-backend-app.replit.app/health
```

Expected response:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-..."
}
```

## API Endpoints

### Health Check
- `GET /health` - Returns server health status

### Captures API (v1)
- `GET /api/v1/captures` - List captures
- `POST /api/v1/captures` - Create new capture (submit image)
- `GET /api/v1/captures/:id` - Get specific capture
- `PATCH /api/v1/captures/:id` - Update capture

### Cards API (v1)
- `GET /api/v1/cards` - List cards
- `GET /api/v1/cards/autocomplete` - Card name autocomplete
- `GET /api/v1/cards/:id` - Get specific card

## Connecting Frontend to Backend

In the frontend Replit app, set this environment variable:

```
NEXT_PUBLIC_API_BASE_URL=https://your-backend-app.replit.app
```

This tells the frontend where to send API requests.

## CORS Configuration

The backend is configured to accept requests from any origin, so the frontend can communicate with it from any Replit domain.
