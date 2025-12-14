from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

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


@app.get("/")
async def root():
    return {"message": "Welcome to MTG AI Suite API", "version": "0.1.0"}


@app.get("/health")
async def health_check():
    return {"status": "healthy"}
