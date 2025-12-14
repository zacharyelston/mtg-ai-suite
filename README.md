# MTG AI Suite

A comprehensive Magic: The Gathering AI-powered toolkit for card database management, rules processing, game state tracking, and intelligent play suggestions.

## 🎯 Project Goals

- **Card Database**: Comprehensive MTG card data with search, filtering, and semantic queries
- **Rules Engine**: Parse and apply MTG Comprehensive Rules for game state validation
- **Game State Tracker**: Real-time tracking of game state, cards in play, and game history
- **AI Play Suggestions**: Intelligent recommendations for optimal plays, deck building, and drafting
- **Card Recognition**: Image-based card identification using OCR and computer vision

## 🏗️ Project Structure

```
mtg-ai-suite/
├── backend/           # Python FastAPI backend
│   ├── api/          # REST API endpoints
│   ├── app/          # Application core
│   ├── models/       # Data models & schemas
│   ├── services/     # Business logic
│   └── utils/        # Utility functions
├── frontend/          # React/Next.js frontend
│   ├── components/   # UI components
│   ├── hooks/        # Custom React hooks
│   ├── pages/        # Page components
│   ├── public/       # Static assets
│   ├── src/          # Source files
│   └── utils/        # Frontend utilities
├── database/          # Database management
│   ├── migrations/   # Schema migrations
│   ├── schemas/      # Database schemas
│   └── seeds/        # Seed data
├── docs/              # Documentation
│   ├── api/          # API documentation
│   ├── development/  # Developer guides
│   └── user/         # User documentation
├── references/        # Literature review & external resources
├── scripts/           # Utility scripts
│   ├── data_import/  # Data import scripts
│   ├── deployment/   # Deployment scripts
│   └── setup/        # Setup scripts
└── tests/             # Test suites
    ├── e2e/          # End-to-end tests
    ├── integration/  # Integration tests
    └── unit/         # Unit tests
```

## 🛠️ Tech Stack

### Backend
- **Framework**: Python 3.11+ with FastAPI
- **Database**: PostgreSQL + pgvector for semantic search
- **Cache**: Redis
- **Task Queue**: Celery

### Frontend
- **Framework**: Next.js 14 with React 18
- **Styling**: TailwindCSS
- **Components**: shadcn/ui
- **State**: Zustand

### AI/ML
- **Embeddings**: sentence-transformers
- **LLM Integration**: OpenAI API / Local models
- **Card Recognition**: Azure Vision API / Tesseract OCR

### Data Sources
- **Primary**: [Scryfall API](https://scryfall.com/docs/api)
- **Secondary**: [MTGJSON](https://mtgjson.com/)

## 🚀 Getting Started

### Prerequisites
- Python 3.11+
- Node.js 18+
- PostgreSQL 15+
- Redis

### Installation

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/mtg-ai-suite.git
cd mtg-ai-suite

# Backend setup
cd backend
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt

# Frontend setup
cd ../frontend
npm install

# Database setup
# (Configure your PostgreSQL connection in .env)
```

### Running the Application

```bash
# Start backend
cd backend
uvicorn app.main:app --reload

# Start frontend (in another terminal)
cd frontend
npm run dev
```

## 📚 Documentation

### Wiki
- **[Wiki Home](https://github.com/zacharyelston/mtg-ai-suite/wiki)** - Project documentation hub
- **[Replit Agent Worklist](https://github.com/zacharyelston/mtg-ai-suite/wiki/Replit-Agent-Worklist)** - Implementation tasks by architecture layer
- **[Multi-Backend Architecture](https://github.com/zacharyelston/mtg-ai-suite/wiki/Multi-Backend-Architecture)** - Mobile + self-hosted backends design

### In-Repo Docs
- [ARCHITECTURE.md](ARCHITECTURE.md) - Core architecture and coding standards
- [REPLIT_SETUP.md](REPLIT_SETUP.md) - Replit environment setup guide
- [docs/PRD_FRONTEND.md](docs/PRD_FRONTEND.md) - Mobile PWA product requirements
- [docs/PRD_BACKEND.md](docs/PRD_BACKEND.md) - Backend API requirements

## 🔗 References

See [references/LITERATURE_REVIEW.md](references/LITERATURE_REVIEW.md) for a comprehensive review of existing MTG tools, APIs, and projects that inform this work.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This project is not affiliated with, endorsed, sponsored, or specifically approved by Wizards of the Coast LLC. Magic: The Gathering is a trademark of Wizards of the Coast. This project complies with the [Wizards of the Coast Fan Content Policy](https://company.wizards.com/fancontentpolicy).

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.
