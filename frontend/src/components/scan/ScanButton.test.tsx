import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { ScanButton } from './ScanButton';

// Mock the scan API
vi.mock('../../api/scan', () => ({
  scanAPI: {
    startScan: vi.fn().mockResolvedValue({}),
    getScanStatus: vi.fn().mockResolvedValue({
      is_scanning: false,
      projects_found: 0,
      files_processed: 0,
    }),
  },
}));

describe('ScanButton', () => {
  it('renders Rescan button', () => {
    render(<ScanButton />);
    expect(screen.getByText('Rescan')).toBeInTheDocument();
  });

  it('has proper title attribute', () => {
    render(<ScanButton />);
    const button = screen.getByRole('button');
    expect(button).toHaveAttribute('title', 'Rescan library for new projects');
  });

  it('is clickable when not scanning', () => {
    render(<ScanButton />);
    const button = screen.getByRole('button');
    expect(button).not.toBeDisabled();
  });

  it('shows spinner icon when scanning', async () => {
    const { scanAPI } = await import('../../api/scan');
    (scanAPI.getScanStatus as any).mockResolvedValue({
      is_scanning: true,
      projects_found: 5,
      files_processed: 10,
    });
    
    render(<ScanButton />);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    
    // Check for scanning state (wait a bit for state update)
    await screen.findByText('Scanning...', {}, { timeout: 2000 });
  });
});
