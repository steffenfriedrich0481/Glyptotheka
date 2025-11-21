import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/react';
import '@testing-library/jest-dom';
import { LoadingSpinner } from '../../src/components/common/LoadingSpinner';

describe('LoadingSpinner Component', () => {
  it('renders spinner', () => {
    const { container } = render(<LoadingSpinner />);
    expect(container.firstChild).toBeInTheDocument();
  });

  it('renders with custom size', () => {
    const { container } = render(<LoadingSpinner size="large" />);
    const spinnerContainer = container.querySelector('.loading-spinner-container.large');
    expect(spinnerContainer).toBeInTheDocument();
  });
});
