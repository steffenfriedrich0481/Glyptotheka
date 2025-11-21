// @vitest-environment jsdom
import { describe, it, expect, afterEach } from 'vitest';
import { render, screen, cleanup, fireEvent } from '@testing-library/react';
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
});
