import type { Request, Response, NextFunction } from 'express';
import { Logger } from '../utils/logger';
import { AppError } from '../utils/errorHandler';

interface ErrorResponse {
  status: 'error';
  message: string;
  details?: string;
  stack?: string;
}

export const errorMiddleware = (
  error: Error | AppError, 
  req: Request, 
  res: Response<ErrorResponse>, 
  next: NextFunction
) => {
  // Log the error
  Logger.error('Unhandled Error', {
    method: req.method,
    path: req.path,
    body: req.body,
    error: error instanceof Error ? error.message : String(error)
  });

  // Determine status code
  const statusCode = error instanceof AppError 
    ? error.statusCode 
    : 500;

  // Prepare error response
  const errorResponse: ErrorResponse = {
    status: 'error',
    message: error instanceof AppError 
      ? error.message 
      : 'An unexpected error occurred',
  };

  // Add additional details in development
  if (process.env.NODE_ENV === 'development') {
    errorResponse.details = error instanceof Error 
      ? error.message 
      : String(error);
    
    if (error instanceof Error) {
      errorResponse.stack = error.stack;
    }
  }

  // Send error response
  res.status(statusCode).json(errorResponse);
}; 