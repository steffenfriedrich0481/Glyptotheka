import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { Skeleton, ProjectTileSkeleton, ProjectGridSkeleton } from '../../src/components/common/Skeleton';

describe('Skeleton Component', () => {
  it('renders basic skeleton', () => {
    const { container } = render(<Skeleton />);
    expect(container.firstChild).toHaveClass('animate-pulse');
    expect(container.firstChild).toHaveClass('bg-gray-300');
  });

  it('renders text variant', () => {
    const { container } = render(<Skeleton variant="text" />);
    expect(container.firstChild).toHaveClass('h-4');
    expect(container.firstChild).toHaveClass('rounded');
  });

  it('renders circular variant', () => {
    const { container } = render(<Skeleton variant="circular" />);
    expect(container.firstChild).toHaveClass('rounded-full');
  });

  it('renders tile variant', () => {
    const { container } = render(<Skeleton variant="tile" />);
    expect(container.firstChild).toHaveClass('aspect-square');
    expect(container.firstChild).toHaveClass('rounded-lg');
  });

  it('applies custom className', () => {
    const { container } = render(<Skeleton className="custom-class" />);
    expect(container.firstChild).toHaveClass('custom-class');
  });
});

describe('ProjectTileSkeleton Component', () => {
  it('renders project tile skeleton structure', () => {
    const { container } = render(<ProjectTileSkeleton />);
    expect(container.querySelector('.bg-white')).toBeInTheDocument();
    expect(container.querySelector('.rounded-lg')).toBeInTheDocument();
  });
});

describe('ProjectGridSkeleton Component', () => {
  it('renders default number of skeleton tiles', () => {
    const { container } = render(<ProjectGridSkeleton />);
    const tiles = container.querySelectorAll('.bg-white');
    expect(tiles).toHaveLength(8);
  });

  it('renders custom number of skeleton tiles', () => {
    const { container } = render(<ProjectGridSkeleton count={5} />);
    const tiles = container.querySelectorAll('.bg-white');
    expect(tiles).toHaveLength(5);
  });

  it('uses grid layout', () => {
    const { container } = render(<ProjectGridSkeleton />);
    const grid = container.firstChild;
    expect(grid).toHaveClass('grid');
  });
});
