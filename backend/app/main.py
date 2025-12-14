from datetime import datetime, timezone
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from api.cards import router as cards_router

app = FastAPI(
    title="MTG AI Suite API",
    description="Magic: The Gathering AI-powered toolkit API",
    version="0.1.0",
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(cards_router)


@app.get("/")
async def root():
    return {"message": "Welcome to MTG AI Suite API", "version": "0.1.0"}


@app.get("/health")
async def health_check():
    return {
        "status": "healthy",
        "version": "0.1.0",
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }
