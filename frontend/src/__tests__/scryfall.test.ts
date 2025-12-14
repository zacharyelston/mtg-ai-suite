import { searchCards, getRandomCard, autocomplete, getCardByName } from '@/lib/scryfall';

global.fetch = jest.fn();

describe('Scryfall API', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('searchCards', () => {
    it('should return search results for valid query', async () => {
      const mockResponse = {
        object: 'list',
        total_cards: 2,
        has_more: false,
        data: [
          { id: '1', name: 'Lightning Bolt' },
          { id: '2', name: 'Lightning Strike' },
        ],
      };

      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockResponse,
      });

      const result = await searchCards('lightning');
      
      expect(global.fetch).toHaveBeenCalledWith(
        'https://api.scryfall.com/cards/search?q=lightning&page=1'
      );
      expect(result.total_cards).toBe(2);
      expect(result.data).toHaveLength(2);
    });

    it('should return empty results for 404', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 404,
      });

      const result = await searchCards('xyznotacard');
      
      expect(result.total_cards).toBe(0);
      expect(result.data).toHaveLength(0);
    });

    it('should throw error for non-404 failures', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
        status: 500,
      });

      await expect(searchCards('test')).rejects.toThrow('Failed to search cards');
    });
  });

  describe('getRandomCard', () => {
    it('should return a random card', async () => {
      const mockCard = { id: 'random-id', name: 'Sol Ring' };

      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockCard,
      });

      const result = await getRandomCard();
      
      expect(global.fetch).toHaveBeenCalledWith(
        'https://api.scryfall.com/cards/random'
      );
      expect(result.name).toBe('Sol Ring');
    });

    it('should throw error on failure', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
      });

      await expect(getRandomCard()).rejects.toThrow('Failed to get random card');
    });
  });

  describe('autocomplete', () => {
    it('should return suggestions for query', async () => {
      const mockSuggestions = ['Lightning Bolt', 'Lightning Strike'];

      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({ data: mockSuggestions }),
      });

      const result = await autocomplete('light');
      
      expect(result).toEqual(mockSuggestions);
    });

    it('should return empty array for short queries', async () => {
      const result = await autocomplete('a');
      
      expect(global.fetch).not.toHaveBeenCalled();
      expect(result).toEqual([]);
    });

    it('should return empty array on failure', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
      });

      const result = await autocomplete('test');
      
      expect(result).toEqual([]);
    });
  });

  describe('getCardByName', () => {
    it('should return card by name', async () => {
      const mockCard = { id: '1', name: 'Black Lotus' };

      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => mockCard,
      });

      const result = await getCardByName('Black Lotus');
      
      expect(result?.name).toBe('Black Lotus');
    });

    it('should return null on failure', async () => {
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: false,
      });

      const result = await getCardByName('Not A Card');
      
      expect(result).toBeNull();
    });
  });
});
