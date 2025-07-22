import type { Request, Response, NextFunction } from 'express';
import { AppError } from '../utils/errorHandler';
import { Logger } from '../utils/logger';
import * as crypto from 'crypto';

export interface UserToken {
  id: string;
  role: 'admin' | 'user';
}

export class AuthMiddleware {
  private static SECRET_KEY: string = process.env.JWT_SECRET || 'default_secret';

  static generateToken(user: UserToken): string {
    const payload = JSON.stringify(user);
    return crypto
      .createHmac('sha256', AuthMiddleware.SECRET_KEY)
      .update(payload)
      .digest('hex');
  }

  static verifyToken(token: string, requiredRole: 'admin' | 'user' = 'user'): UserToken {
    try {
      // Simulated token validation
      const decodedUser: UserToken = {
        id: 'user123',
        role: 'user'
      };

      if (requiredRole === 'admin' && decodedUser.role !== 'admin') {
        throw new AppError('Insufficient permissions', 403);
      }

      return decodedUser;
    } catch (error) {
      Logger.error('Token verification failed', { error: error instanceof Error ? error.message : String(error) });
      throw new AppError('Invalid or expired token', 401);
    }
  }

  static authenticate(requiredRole: 'admin' | 'user' = 'user') {
    return (req: Request, res: Response, next: NextFunction): void => {
      const authHeader = req.get('Authorization');
      const token = authHeader ? authHeader.split(' ')[1] : undefined;

      if (!token) {
        Logger.warn('No token provided');
        return next(new AppError('No token provided', 401));
      }

      try {
        const user = AuthMiddleware.verifyToken(token, requiredRole);
        (req as any).user = user;
        next();
      } catch (error) {
        next(error instanceof Error ? error : new Error(String(error)));
      }
    };
  }
}

// Convenience methods for different auth levels
export const requireAuth = AuthMiddleware.authenticate();
export const requireAdminAuth = AuthMiddleware.authenticate('admin'); 