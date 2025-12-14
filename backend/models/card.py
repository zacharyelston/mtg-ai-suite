from pydantic import BaseModel
from typing import Optional
from datetime import date


class CardBase(BaseModel):
    name: str
    mana_cost: Optional[str] = None
    cmc: float = 0.0
    type_line: str
    oracle_text: Optional[str] = None
    power: Optional[str] = None
    toughness: Optional[str] = None
    colors: list[str] = []
    color_identity: list[str] = []
    keywords: list[str] = []
    set_code: str
    rarity: str
    

class Card(CardBase):
    id: str
    scryfall_id: str
    oracle_id: Optional[str] = None
    image_uri: Optional[str] = None
    art_crop_uri: Optional[str] = None
    released_at: Optional[date] = None
    legalities: dict[str, str] = {}
    prices: dict[str, Optional[str]] = {}

    class Config:
        from_attributes = True


class CardCreate(CardBase):
    scryfall_id: str


class CardSearch(BaseModel):
    query: str
    colors: Optional[list[str]] = None
    types: Optional[list[str]] = None
    sets: Optional[list[str]] = None
    rarity: Optional[list[str]] = None
    cmc_min: Optional[float] = None
    cmc_max: Optional[float] = None
    limit: int = 20
    offset: int = 0
