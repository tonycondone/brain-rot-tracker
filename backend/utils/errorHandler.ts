class AppError extends Error {
  statusCode: number;

  constructor(message: string, statusCode: number = 500) {
    super(message);
    this.statusCode = statusCode;
    this.name = 'AppError';
  }
}

export const handleError = (error: Error) => {
  console.error(`[${new Date().toISOString()}] Error: ${error.message}`);
  
  if (error instanceof AppError) {
    return {
      status: error.statusCode,
      message: error.message,
    };
  }

  return {
    status: 500,
    message: 'An unexpected error occurred',
  };
};

export { AppError }; 