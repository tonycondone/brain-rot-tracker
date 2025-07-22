import * as dotenv from 'dotenv';
import { z } from 'zod';

// Load environment variables
dotenv.config();

// Configuration schema validation
const ConfigSchema = z.object({
  MONGO_URI: z.string().url('Invalid MongoDB connection string'),
  REDIS_URI: z.string().url('Invalid Redis connection string'),
  NEXT_PUBLIC_WS_URL: z.string().url('Invalid WebSocket URL'),
  PORT: z.string().transform(Number).default('3001'),
  NODE_ENV: z.enum(['development', 'production', 'test']).default('development')
});

// Validate and parse configuration
const validateConfig = (): z.infer<typeof ConfigSchema> => {
  try {
    return ConfigSchema.parse({
      MONGO_URI: process.env.MONGO_URI,
      REDIS_URI: process.env.REDIS_URI,
      NEXT_PUBLIC_WS_URL: process.env.NEXT_PUBLIC_WS_URL,
      PORT: process.env.PORT,
      NODE_ENV: process.env.NODE_ENV
    });
  } catch (error) {
    console.error('Configuration validation failed:', error);
    throw error;
  }
};

// Singleton configuration instance
class Config {
  private static instance: ReturnType<typeof validateConfig>;

  private constructor() {}

  public static getInstance(): ReturnType<typeof validateConfig> {
    if (!Config.instance) {
      Config.instance = validateConfig();
    }
    return Config.instance;
  }
}

export const config = Config.getInstance(); 