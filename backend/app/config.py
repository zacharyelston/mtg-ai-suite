from pydantic_settings import BaseSettings
from functools import lru_cache


class Settings(BaseSettings):
    app_name: str = "MTG AI Suite"
    debug: bool = False
    
    # Database
    database_url: str = "postgresql+asyncpg://localhost/mtg_ai_suite"
    
    # Redis
    redis_url: str = "redis://localhost:6379"
    
    # External APIs
    scryfall_api_base: str = "https://api.scryfall.com"
    openai_api_key: str = ""
    azure_vision_key: str = ""
    azure_vision_endpoint: str = ""
    
    # Security
    secret_key: str = "change-me-in-production"
    algorithm: str = "HS256"
    access_token_expire_minutes: int = 30

    class Config:
        env_file = ".env"


@lru_cache()
def get_settings():
    return Settings()
