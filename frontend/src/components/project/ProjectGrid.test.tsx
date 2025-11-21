import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import ProjectGrid from './ProjectGrid';
import { ProjectWithChildren } from '../../types/project';

describe('ProjectGrid', () => {
  const mockProjects: ProjectWithChildren[] = [
    {
      id: 1,
      name: 'Project 1',
      full_path: '/project1',
      parent_id: null,
      is_leaf: true,
      description: null,
      created_at: Date.now(),
      updated_at: Date.now(),
      children: [],
      stl_count: 3,
      image_count: 1,
      tags: [],
    },
    {
      id: 2,
      name: 'Project 2',
      full_path: '/project2',
      parent_id: null,
      is_leaf: true,
      description: null,
      created_at: Date.now(),
      updated_at: Date.now(),
      children: [],
      stl_count: 5,
      image_count: 2,
      tags: [],
    },
  ];

  it('renders loading state with skeleton tiles', () => {
    const onProjectClick = vi.fn();
    render(<ProjectGrid projects={[]} onProjectClick={onProjectClick} loading={true} />);
    expect(screen.getByLabelText('Loading projects')).toBeInTheDocument();
  });

  it('renders empty state when no projects', () => {
    const onProjectClick = vi.fn();
    render(<ProjectGrid projects={[]} onProjectClick={onProjectClick} loading={false} />);
    expect(screen.getByText('No projects found')).toBeInTheDocument();
  });

  it('renders all projects', () => {
    const onProjectClick = vi.fn();
    render(<ProjectGrid projects={mockProjects} onProjectClick={onProjectClick} />);
    expect(screen.getByText('Project 1')).toBeInTheDocument();
    expect(screen.getByText('Project 2')).toBeInTheDocument();
  });

  it('has proper ARIA role for grid', () => {
    const onProjectClick = vi.fn();
    render(<ProjectGrid projects={mockProjects} onProjectClick={onProjectClick} />);
    expect(screen.getByRole('grid')).toBeInTheDocument();
  });

  it('displays correct file counts', () => {
    const onProjectClick = vi.fn();
    render(<ProjectGrid projects={mockProjects} onProjectClick={onProjectClick} />);
    
    // Project 1: 3 STLs, 1 Image
    expect(screen.getByText('3')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument();
    
    // Project 2: 5 STLs, 2 Images
    expect(screen.getByText('5')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
  });
});
