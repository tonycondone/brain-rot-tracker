/// <reference types="jest" />

declare namespace jest {
  interface MockedFunction<T extends (...args: any[]) => any> {
    mock: {
      calls: any[][];
      results: Array<{
        type: 'return' | 'throw';
        value: any;
      }>;
    };
    mockImplementation(fn?: (...args: Parameters<T>) => ReturnType<T>): this;
    mockReturnValue(value: ReturnType<T>): this;
    mockResolvedValue(value: ReturnType<T>): this;
  }
} 