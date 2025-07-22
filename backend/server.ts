import express from 'express';
import http from 'http';
import { Server } from 'socket.io';
import { Item } from './db/models';
import mongoose from 'mongoose';
import Redis from 'redis';
import cors from 'cors';
import helmet from 'helmet';
import { handleError, AppError } from './utils/errorHandler';
import { Logger } from './utils/logger';
import { errorMiddleware } from './middleware/errorMiddleware';
import { createRateLimiter } from './middleware/rateLimitMiddleware';
import { requireAuth } from './middleware/authMiddleware';

const app = express();
const server = http.createServer(app);
export const io = new Server(server, { cors: { origin: '*' } });

// Middleware setup
app.use(cors({
  origin: process.env.FRONTEND_URL || 'http://localhost:3000',
  credentials: true
}));
app.use(helmet());
app.use(express.json());
app.use(createRateLimiter());

const connectDatabase = async () => {
  try {
    await mongoose.connect(process.env.MONGO_URI!);
    Logger.info('Connected to MongoDB');
  } catch (error) {
    Logger.error('MongoDB connection failed', { error });
    throw new AppError('Database connection failed', 500);
  }
};

const connectRedis = async () => {
  try {
    const redisClient = Redis.createClient({ url: process.env.REDIS_URI });
    await redisClient.connect();
    Logger.info('Connected to Redis');
    return redisClient;
  } catch (error) {
    Logger.error('Redis connection failed', { error });
    throw new AppError('Redis connection failed', 500);
  }
};

io.on('connection', (socket) => {
  socket.on('subscribe', () => {});
});

export async function notifyNew(item: any) {
  io.emit('new-item', item);
}

// Protected item retrieval route
app.get('/api/items', requireAuth, async (_, res) => {
  try {
    const items = await Item.find()
      .sort({ 'metrics.trendingScore': -1 })
      .limit(50)
      .exec();
    res.json(items);
  } catch (error) {
    const errorResponse = handleError(error as Error);
    res.status(errorResponse.status).json(errorResponse);
  }
});

// Error handling middleware (should be last)
app.use(errorMiddleware);

const startServer = async () => {
  try {
    await connectDatabase();
    await connectRedis();
    
    const PORT = process.env.PORT || 3001;
    server.listen(PORT, () => {
      Logger.info(`Server listening on port ${PORT}`);
    });
  } catch (error) {
    Logger.error('Server startup failed', { error });
    process.exit(1);
  }
};

startServer();
