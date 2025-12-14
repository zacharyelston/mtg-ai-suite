import httpx
from typing import Optional
from app.config import get_settings


class ScryfallService:
    def __init__(self):
        self.settings = get_settings()
        self.base_url = self.settings.scryfall_api_base
        self.headers = {
            "User-Agent": "MTGAISuite/0.1.0",
            "Accept": "application/json",
        }

    async def search_cards(
        self,
        query: str,
        unique: str = "cards",
        order: str = "name",
        page: int = 1,
    ) -> dict:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/cards/search",
                params={
                    "q": query,
                    "unique": unique,
                    "order": order,
                    "page": page,
                },
                headers=self.headers,
            )
            response.raise_for_status()
            return response.json()

    async def get_card_by_id(self, scryfall_id: str) -> dict:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/cards/{scryfall_id}",
                headers=self.headers,
            )
            response.raise_for_status()
            return response.json()

    async def get_card_by_name(
        self, name: str, exact: bool = False, set_code: Optional[str] = None
    ) -> dict:
        params = {"exact" if exact else "fuzzy": name}
        if set_code:
            params["set"] = set_code

        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/cards/named",
                params=params,
                headers=self.headers,
            )
            response.raise_for_status()
            return response.json()

    async def get_random_card(self, query: Optional[str] = None) -> dict:
        params = {}
        if query:
            params["q"] = query

        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/cards/random",
                params=params,
                headers=self.headers,
            )
            response.raise_for_status()
            return response.json()

    async def autocomplete(self, query: str) -> list[str]:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/cards/autocomplete",
                params={"q": query},
                headers=self.headers,
            )
            response.raise_for_status()
            data = response.json()
            return data.get("data", [])

    async def get_sets(self) -> list[dict]:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/sets",
                headers=self.headers,
            )
            response.raise_for_status()
            data = response.json()
            return data.get("data", [])

    async def get_bulk_data(self) -> list[dict]:
        async with httpx.AsyncClient() as client:
            response = await client.get(
                f"{self.base_url}/bulk-data",
                headers=self.headers,
            )
            response.raise_for_status()
            data = response.json()
            return data.get("data", [])
