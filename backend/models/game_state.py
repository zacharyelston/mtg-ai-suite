from pydantic import BaseModel
from typing import Optional
from enum import Enum


class Zone(str, Enum):
    LIBRARY = "library"
    HAND = "hand"
    BATTLEFIELD = "battlefield"
    GRAVEYARD = "graveyard"
    EXILE = "exile"
    COMMAND = "command"
    STACK = "stack"


class Phase(str, Enum):
    UNTAP = "untap"
    UPKEEP = "upkeep"
    DRAW = "draw"
    MAIN_1 = "main_1"
    COMBAT_BEGIN = "combat_begin"
    COMBAT_ATTACKERS = "combat_attackers"
    COMBAT_BLOCKERS = "combat_blockers"
    COMBAT_DAMAGE = "combat_damage"
    COMBAT_END = "combat_end"
    MAIN_2 = "main_2"
    END = "end"
    CLEANUP = "cleanup"


class CardInstance(BaseModel):
    instance_id: str
    card_id: str
    zone: Zone
    tapped: bool = False
    counters: dict[str, int] = {}
    attached_to: Optional[str] = None
    controller_id: str
    owner_id: str


class Player(BaseModel):
    id: str
    name: str
    life: int = 20
    poison_counters: int = 0
    mana_pool: dict[str, int] = {}
    has_priority: bool = False


class GameState(BaseModel):
    game_id: str
    turn_number: int = 1
    active_player_id: str
    phase: Phase = Phase.MAIN_1
    players: list[Player] = []
    cards: list[CardInstance] = []
    stack: list[str] = []


class PlaySuggestion(BaseModel):
    action: str
    card_id: Optional[str] = None
    target_ids: list[str] = []
    reasoning: str
    confidence: float
    priority: int
