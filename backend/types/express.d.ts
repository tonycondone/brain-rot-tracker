/// <reference types="node" />
/// <reference types="express" />

declare module 'express' {
  interface Request {
    user?: {
      id: string;
      role: 'admin' | 'user';
    };
  }

  interface Response {
    locals: {
      [key: string]: any;
    };
  }
}

export {}; 