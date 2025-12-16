import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
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
    expect(button).toHaveAttribute('title', 'Quick incremental rescan');
  });

  it('is clickable when not scanning', () => {
    render(<ScanButton />);
    const button = screen.getByRole('button');
    expect(button).not.toBeDisabled();
  });

  it('shows force rescan and clean rescan checkboxes', () => {
    render(<ScanButton />);
    expect(screen.getByText('Force full rescan')).toBeInTheDocument();
    expect(screen.getByText('Clean rescan (clear all data)')).toBeInTheDocument();
  });

  it('calls startScan with clean=true when clean rescan is checked', async () => {
    const { scanAPI } = await import('../../api/scan');
    
    render(<ScanButton />);
    
    // Check the clean rescan checkbox
    const cleanCheckbox = screen.getByLabelText(/Clean rescan/i);
    fireEvent.click(cleanCheckbox);
    
    // Click the scan button
    const button = screen.getByRole('button');
    fireEvent.click(button);
    
    // Verify startScan was called with clean=true
    expect(scanAPI.startScan).toHaveBeenCalledWith(true, true);
  });
});
