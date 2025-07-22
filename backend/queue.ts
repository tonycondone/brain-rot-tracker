import Bull from 'bull';
import { scrapeTikTok } from './scrapers/tiktokScraper';
import { fetchRecentItems, calcBrainRotScore } from './scrapers/robloxScraper';
import mongoose from 'mongoose';
import Redis from 'redis';
import { Item } from './db/models';
import { notifyNew } from './server';
import { Logger, LogLevel } from './utils/logger';
import { AppError, handleError } from './utils/errorHandler';

const connectDatabase = async () => {
  try {
    await mongoose.connect(process.env.MONGO_URI!);
    Logger.info('Connected to MongoDB for queue processing');
  } catch (error) {
    Logger.error('MongoDB connection failed', { error });
    throw new AppError('Database connection failed', 500);
  }
};

const connectRedis = async () => {
  try {
    const redisClient = Redis.createClient({ url: process.env.REDIS_URI });
    await redisClient.connect();
    Logger.info('Connected to Redis for queue processing');
    return redisClient;
  } catch (error) {
    Logger.error('Redis connection failed', { error });
    throw new AppError('Redis connection failed', 500);
  }
};

const scraperQueue = new Bull('scrape-queue', process.env.REDIS_URI!);

const processScrapeJob = async () => {
  try {
    const hashtags = ['skibiditoilet','ohio','sigma','gyatt','rizz','sussybaka','drip'];
    
    for (const tag of hashtags) {
      try {
        const videos = await scrapeTikTok(tag);
        videos.forEach(async (v) => {
          const desc = v.description;
          if (!desc.toLowerCase().includes('roblox')) return;
          const itemMatch = desc.match(/roblox item (\d+)/i);
          if (!itemMatch) return;
        });
      } catch (error) {
        Logger.warn(`TikTok scraping failed for tag: ${tag}`, { error });
      }
    }

    const items = await fetchRecentItems();
    for (const raw of items) {
      const score = calcBrainRotScore(raw.name, raw.description || '');
      if (score > 50) {
        try {
          const doc = await Item.findOneAndUpdate(
            { itemId: raw.id.toString() },
            {
              itemId: raw.id.toString(),
              name: raw.name,
              category: 'brain-rot',
              price: { current: raw.price || 0, history: [], original: raw.price || 0 },
              metrics: { trendingScore: score, tiktokMentions: 0, viewsLast24h: 0 },
              images: { thumbnail: raw.thumbnailUrl, fullSize: '', inGame: [] },
              scrapedFrom: ['roblox'],
              lastUpdated: new Date(),
            },
            { upsert: true, new: true }
          );
          notifyNew(doc);
          Logger.info('New brain rot item processed', { itemId: doc.itemId, score });
        } catch (error) {
          Logger.error('Failed to process item', { 
            itemId: raw.id, 
            error: handleError(error as Error) 
          });
        }
      }
    }
  } catch (error) {
    Logger.error('Scrape job failed', { error: handleError(error as Error) });
  }
};

const initializeQueue = async () => {
  try {
    await connectDatabase();
    await connectRedis();

    scraperQueue.process(processScrapeJob);
    scraperQueue.add({}, { 
      repeat: { cron: '*/15 * * * *' },
      attempts: 3,
      backoff: {
        type: 'exponential',
        delay: 1000
      }
    });

    Logger.info('Scraper queue initialized successfully');
  } catch (error) {
    Logger.error('Queue initialization failed', { error });
    process.exit(1);
  }
};

initializeQueue();
