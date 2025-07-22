// Global test setup
import dotenv from 'dotenv';

// Load environment variables for testing
dotenv.config({ path: '.env.test' });

// Optional: Add global mocks or test configurations
jest.setTimeout(10000); // Increase timeout for async tests 