from fastapi import APIRouter, HTTPException, Query
from typing import Optional
from services.scryfall import ScryfallService

router = APIRouter(prefix="/cards", tags=["cards"])
scryfall = ScryfallService()


@router.get("/search")
async def search_cards(
    q: str = Query(..., description="Search query"),
    page: int = Query(1, ge=1, description="Page number"),
):
    try:
        results = await scryfall.search_cards(query=q, page=page)
        return results
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/autocomplete")
async def autocomplete(
    q: str = Query(..., min_length=2, description="Search query"),
):
    try:
        suggestions = await scryfall.autocomplete(query=q)
        return {"data": suggestions}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/random")
async def random_card(
    q: Optional[str] = Query(None, description="Optional filter query"),
):
    try:
        card = await scryfall.get_random_card(query=q)
        return card
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/named")
async def get_card_by_name(
    name: str = Query(..., description="Card name"),
    exact: bool = Query(False, description="Exact match"),
    set_code: Optional[str] = Query(None, description="Set code"),
):
    try:
        card = await scryfall.get_card_by_name(
            name=name, exact=exact, set_code=set_code
        )
        return card
    except Exception as e:
        raise HTTPException(status_code=404, detail=f"Card not found: {name}")


@router.get("/{card_id}")
async def get_card(card_id: str):
    try:
        card = await scryfall.get_card_by_id(scryfall_id=card_id)
        return card
    except Exception as e:
        raise HTTPException(status_code=404, detail=f"Card not found: {card_id}")
