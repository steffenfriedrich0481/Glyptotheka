import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/react';
import { afterEach } from 'vitest';

// Cleanup after each test to avoid multiple elements
afterEach(() => {
  cleanup();
});
