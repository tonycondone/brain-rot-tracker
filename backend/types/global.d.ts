/// <reference types="node" />
/// <reference types="express" />
/// <reference types="socket.io" />

declare namespace NodeJS {
  interface ProcessEnv {
    MONGO_URI: string;
    REDIS_URI: string;
    NEXT_PUBLIC_WS_URL: string;
    PORT?: string;
    NODE_ENV?: 'development' | 'production' | 'test';
    FRONTEND_URL?: string;
    JWT_SECRET?: string;
  }
}

// Extend Express Request type
declare module 'express-serve-static-core' {
  interface Request {
    user?: {
      id: string;
      role: 'admin' | 'user';
    };
    get(name: string): string | undefined;
  }
}

// Additional module declarations
declare module 'bull' {
  import { Job, Queue } from 'bull';
  export = Queue;
}

declare module 'node-fetch' {
  import fetch from 'node-fetch';
  export = fetch;
}

declare module 'crypto' {
  export function createHmac(algorithm: string, key: string): {
    update(data: string): {
      digest(encoding: string): string;
    };
  };
}

export {}; 