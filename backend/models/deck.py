from pydantic import BaseModel
from typing import Optional
from datetime import datetime


class DeckCard(BaseModel):
    card_id: str
    quantity: int
    is_sideboard: bool = False
    is_commander: bool = False


class DeckBase(BaseModel):
    name: str
    format: str
    description: Optional[str] = None
    archetype: Optional[str] = None


class Deck(DeckBase):
    id: str
    cards: list[DeckCard] = []
    created_at: datetime
    updated_at: datetime
    
    class Config:
        from_attributes = True


class DeckCreate(DeckBase):
    cards: list[DeckCard] = []


class DeckAnalysis(BaseModel):
    deck_id: str
    archetype: str
    archetype_confidence: float
    color_distribution: dict[str, int]
    mana_curve: dict[int, int]
    card_type_distribution: dict[str, int]
    synergy_score: float
    suggestions: list[str] = []
