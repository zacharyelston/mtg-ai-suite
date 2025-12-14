import pytest
from unittest.mock import AsyncMock, patch, MagicMock
import httpx


class TestScryfallService:
    @pytest.fixture
    def mock_settings(self):
        settings = MagicMock()
        settings.scryfall_api_base = "https://api.scryfall.com"
        return settings

    @pytest.fixture
    def scryfall_service(self, mock_settings):
        with patch('services.scryfall.get_settings', return_value=mock_settings):
            from services.scryfall import ScryfallService
            return ScryfallService()

    @pytest.mark.asyncio
    async def test_search_cards_success(self, scryfall_service):
        mock_response = MagicMock()
        mock_response.json.return_value = {
            "object": "list",
            "total_cards": 2,
            "has_more": False,
            "data": [
                {"name": "Lightning Bolt"},
                {"name": "Lightning Strike"},
            ],
        }
        mock_response.raise_for_status = MagicMock()

        with patch.object(httpx.AsyncClient, 'get', new_callable=AsyncMock) as mock_get:
            mock_get.return_value = mock_response
            
            async with httpx.AsyncClient() as client:
                result = await scryfall_service.search_cards("lightning")
            
            assert result["total_cards"] == 2
            assert len(result["data"]) == 2

    @pytest.mark.asyncio
    async def test_get_random_card_success(self, scryfall_service):
        mock_response = MagicMock()
        mock_response.json.return_value = {"name": "Sol Ring", "id": "random-id"}
        mock_response.raise_for_status = MagicMock()

        with patch.object(httpx.AsyncClient, 'get', new_callable=AsyncMock) as mock_get:
            mock_get.return_value = mock_response
            
            result = await scryfall_service.get_random_card()
            
            assert result["name"] == "Sol Ring"

    @pytest.mark.asyncio
    async def test_autocomplete_success(self, scryfall_service):
        mock_response = MagicMock()
        mock_response.json.return_value = {
            "data": ["Lightning Bolt", "Lightning Strike"]
        }
        mock_response.raise_for_status = MagicMock()

        with patch.object(httpx.AsyncClient, 'get', new_callable=AsyncMock) as mock_get:
            mock_get.return_value = mock_response
            
            result = await scryfall_service.autocomplete("light")
            
            assert "Lightning Bolt" in result
            assert len(result) == 2
