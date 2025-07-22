import { Request, Response, NextFunction } from 'express';
import { AppError } from '../utils/errorHandler';
import { Logger } from '../utils/logger';

interface RateLimitStore {
  [key: string]: {
    count: number;
    resetTime: number;
  };
}

export class RateLimiter {
  private store: RateLimitStore = {};
  private windowMs: number;
  private maxRequests: number;

  constructor(windowMs = 15 * 60 * 1000, maxRequests = 100) {
    this.windowMs = windowMs;
    this.maxRequests = maxRequests;
  }

  middleware = (req: Request, res: Response, next: NextFunction): void => {
    const ip = req.ip || req.connection.remoteAddress || 'unknown';
    const now = Date.now();

    // Clean up expired entries
    this.cleanupStore(now);

    // Initialize or update entry for this IP
    if (!this.store[ip]) {
      this.store[ip] = { count: 1, resetTime: now + this.windowMs };
      return next();
    }

    const entry = this.store[ip];

    // Check if within rate limit
    if (entry.count >= this.maxRequests) {
      Logger.warn('Rate limit exceeded', { ip });
      return next(new AppError('Too many requests, please try again later', 429));
    }

    // Increment request count
    entry.count++;
    next();
  }

  private cleanupStore(now: number): void {
    Object.keys(this.store).forEach(ip => {
      if (this.store[ip].resetTime < now) {
        delete this.store[ip];
      }
    });
  }
}

export const createRateLimiter = (
  windowMs = 15 * 60 * 1000, 
  maxRequests = 100
) => {
  const limiter = new RateLimiter(windowMs, maxRequests);
  return limiter.middleware;
}; 