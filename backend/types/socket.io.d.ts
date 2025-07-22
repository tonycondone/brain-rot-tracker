/// <reference types="socket.io" />

declare module 'socket.io' {
  interface Socket {
    user?: {
      id: string;
      role: 'admin' | 'user';
    };
  }

  interface Namespace {
    adapter: {
      [key: string]: any;
    };
  }
}

export {}; 