// @vitest-environment jsdom
import { describe, it, expect, vi, afterEach } from 'vitest';
import { render, screen, fireEvent, cleanup } from '@testing-library/react';
import * as matchers from '@testing-library/jest-dom/matchers';
import ProjectTile from './ProjectTile';
import { ProjectWithChildren } from '../../types/project';

expect.extend(matchers);

afterEach(() => {
  cleanup();
});

describe('ProjectTile', () => {
  const mockProject: ProjectWithChildren = {
    id: 1,
    name: 'Test Project',
    full_path: '/test',
    parent_id: null,
    is_leaf: true,
    description: null,
    created_at: Date.now(),
    updated_at: Date.now(),
    children: [],
    stl_count: 5,
    image_count: 2,
    tags: [],
  };

  const mockFolder: ProjectWithChildren = {
    ...mockProject,
    id: 2,
    name: 'Test Folder',
    is_leaf: false,
    children: [mockProject],
  };

  it('renders project tile with name', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    expect(screen.getByText('Test Project')).toBeInTheDocument();
  });

  it('displays file count', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    expect(screen.getByText('5')).toBeInTheDocument();
  });

  it('shows "Project" badge for leaf projects', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', expect.stringContaining('Project'));
  });

  it('shows "Folder" badge for folders', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockFolder} onClick={onClick} />);
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', expect.stringContaining('Folder'));
  });

  it('displays child count for folders', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockFolder} onClick={onClick} />);
    expect(screen.getByText('1')).toBeInTheDocument();
  });

  it('calls onClick when clicked', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    fireEvent.click(screen.getByRole('button'));
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('calls onClick when Enter key is pressed', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    const tile = screen.getByRole('button');
    fireEvent.keyDown(tile, { key: 'Enter' });
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('calls onClick when Space key is pressed', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    const tile = screen.getByRole('button');
    fireEvent.keyDown(tile, { key: ' ' });
    expect(onClick).toHaveBeenCalledTimes(1);
  });

  it('has proper ARIA label', () => {
    const onClick = vi.fn();
    render(<ProjectTile project={mockProject} onClick={onClick} />);
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', 'Project: Test Project');
  });

  it('renders carousel when images are present', () => {
    const projectWithImages: any = {
      ...mockProject,
      images: [{ id: 1, filename: 'test.jpg', source_type: 'direct', image_source: 'original', priority: 100 }]
    };
    const onClick = vi.fn();
    render(<ProjectTile project={projectWithImages} onClick={onClick} />);
    expect(screen.getByAltText('Test Project - Image 1')).toBeInTheDocument();
  });
});
