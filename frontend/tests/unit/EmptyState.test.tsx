import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { EmptyState, NoProjectsFound, NoSearchResults, NoFilesFound } from '../../src/components/common/EmptyState';

describe('EmptyState Component', () => {
  it('renders title', () => {
    render(<EmptyState title="Test Title" />);
    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });

  it('renders description when provided', () => {
    render(<EmptyState title="Title" description="Test description" />);
    expect(screen.getByText('Test description')).toBeInTheDocument();
  });

  it('does not render description when not provided', () => {
    const { container } = render(<EmptyState title="Title" />);
    const descriptions = container.querySelectorAll('p.text-gray-600');
    expect(descriptions).toHaveLength(0);
  });

  it('renders action button when provided', () => {
    const action = { label: 'Click me', onClick: () => {} };
    render(<EmptyState title="Title" action={action} />);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });
});

describe('NoProjectsFound Component', () => {
  it('renders no projects message', () => {
    render(<NoProjectsFound />);
    expect(screen.getByText('No projects found')).toBeInTheDocument();
  });

  it('provides helpful description', () => {
    render(<NoProjectsFound />);
    expect(screen.getByText(/No 3D print projects/)).toBeInTheDocument();
  });
});

describe('NoSearchResults Component', () => {
  it('renders no results message', () => {
    render(<NoSearchResults query="test query" />);
    expect(screen.getByText('No results found')).toBeInTheDocument();
  });

  it('includes search query in description', () => {
    render(<NoSearchResults query="test query" />);
    expect(screen.getByText(/test query/)).toBeInTheDocument();
  });
});

describe('NoFilesFound Component', () => {
  it('renders no files message', () => {
    render(<NoFilesFound />);
    expect(screen.getByText('No files found')).toBeInTheDocument();
  });
});
