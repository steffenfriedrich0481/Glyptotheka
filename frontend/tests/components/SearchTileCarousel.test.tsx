// @vitest-environment jsdom
import { describe, it, expect, afterEach, vi } from 'vitest';
import { render, screen, cleanup, fireEvent, act } from '@testing-library/react';
import * as matchers from '@testing-library/jest-dom/matchers';
import { SearchTileCarousel } from '../../src/components/project/SearchTileCarousel';
import { ImagePreview } from '../../src/types/project';

expect.extend(matchers);

afterEach(() => {
  cleanup();
});

const mockImages: ImagePreview[] = [
  { id: 1, filename: 'img1.jpg', source_type: 'direct', image_source: 'original', priority: 100 },
  { id: 2, filename: 'img2.jpg', source_type: 'direct', image_source: 'original', priority: 100 },
  { id: 3, filename: 'img3.jpg', source_type: 'direct', image_source: 'original', priority: 100 },
];

describe('SearchTileCarousel', () => {
  it('renders placeholder when no images provided', () => {
    render(<SearchTileCarousel images={[]} projectName="Test Project" />);
    expect(screen.getByText('📦')).toBeInTheDocument();
  });

  it('renders first image when images provided', () => {
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" />);
    const img = screen.getByAltText('Test Project - Image 1');
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute('src', '/api/files/images/1');
  });

  it('shows navigation controls when multiple images', () => {
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" />);
    expect(screen.getByLabelText('Previous image')).toBeInTheDocument();
    expect(screen.getByLabelText('Next image')).toBeInTheDocument();
    expect(screen.getAllByLabelText(/Go to image/)).toHaveLength(3);
  });

  it('navigates to next image', () => {
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" />);
    const nextBtn = screen.getByLabelText('Next image');
    fireEvent.click(nextBtn);
    expect(screen.getByAltText('Test Project - Image 2')).toBeInTheDocument();
  });

  it('auto-advances to next image', () => {
    vi.useFakeTimers();
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" autoAdvance={true} />);
    
    expect(screen.getByAltText('Test Project - Image 1')).toBeInTheDocument();
    
    act(() => {
      vi.advanceTimersByTime(5000);
    });
    
    expect(screen.getByAltText('Test Project - Image 2')).toBeInTheDocument();
    vi.useRealTimers();
  });

  it('pauses on hover', () => {
    vi.useFakeTimers();
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" autoAdvance={true} />);
    
    const container = screen.getByAltText('Test Project - Image 1').parentElement!;
    fireEvent.mouseEnter(container);
    
    act(() => {
      vi.advanceTimersByTime(5000);
    });
    
    expect(screen.getByAltText('Test Project - Image 1')).toBeInTheDocument();
    vi.useRealTimers();
  });

  it('pauses after manual navigation', () => {
    vi.useFakeTimers();
    render(<SearchTileCarousel images={mockImages} projectName="Test Project" autoAdvance={true} />);
    
    const nextBtn = screen.getByLabelText('Next image');
    fireEvent.click(nextBtn);
    
    // Should be Image 2
    expect(screen.getByAltText('Test Project - Image 2')).toBeInTheDocument();
    
    // Advance by 5000ms (should be paused)
    act(() => {
      vi.advanceTimersByTime(5000);
    });
    
    // Should still be Image 2
    expect(screen.getByAltText('Test Project - Image 2')).toBeInTheDocument();
    
    // Advance to end pause (10000ms)
    act(() => {
      vi.advanceTimersByTime(10000);
    });
    
    // Advance to trigger interval (4000ms)
    act(() => {
      vi.advanceTimersByTime(5000);
    });
    
    // Should have advanced to Image 3
    expect(screen.getByAltText('Test Project - Image 3')).toBeInTheDocument();
    
    vi.useRealTimers();
  });
});
