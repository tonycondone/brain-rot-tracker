import { describe, it, expect, jest } from '@jest/globals';
import { fetchRecentItems, calcBrainRotScore } from '../../scrapers/robloxScraper';
import fetch from 'node-fetch';

// Mock node-fetch
jest.mock('node-fetch');

describe('RobloxScraper', () => {
  describe('fetchRecentItems', () => {
    it('should fetch recent items successfully', async () => {
      const mockResponse = {
        json: jest.fn().mockResolvedValue({
          data: [
            { 
              id: '123', 
              name: 'Test Item', 
              description: 'A test description',
              price: 100,
              thumbnailUrl: 'https://example.com/thumbnail.png'
            }
          ]
        })
      };

      (fetch as jest.MockedFunction<typeof fetch>).mockResolvedValue(mockResponse as any);

      const items = await fetchRecentItems();
      expect(items).toHaveLength(1);
      expect(items[0].id).toBe('123');
    });
  });

  describe('calcBrainRotScore', () => {
    it('should calculate brain rot score correctly', () => {
      const testCases = [
        { name: 'Skibidi Toilet Item', desc: 'A cringe item', expectedScore: 30 },
        { name: 'Normal Item', desc: 'Nothing special', expectedScore: 0 },
        { name: 'Sigma Rizz', desc: 'Among us sus', expectedScore: 45 }
      ];

      testCases.forEach(({ name, desc, expectedScore }) => {
        const score = calcBrainRotScore(name, desc);
        expect(score).toBe(expectedScore);
      });
    });
  });
}); 