import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import Breadcrumb from './Breadcrumb';

describe('Breadcrumb', () => {
  const mockItems = [
    { id: 1, name: 'Folder 1', path: '/folder1' },
    { id: 2, name: 'Folder 2', path: '/folder1/folder2' },
  ];

  it('renders nothing when items array is empty', () => {
    const onNavigate = vi.fn();
    const { container } = render(<Breadcrumb items={[]} onNavigate={onNavigate} />);
    expect(container.firstChild).toBeNull();
  });

  it('renders Home link', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    expect(screen.getByText('🏠 Home')).toBeInTheDocument();
  });

  it('renders all breadcrumb items', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    expect(screen.getByText('Folder 1')).toBeInTheDocument();
    expect(screen.getByText('Folder 2')).toBeInTheDocument();
  });

  it('shows last item as current (not clickable)', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    const folder2 = screen.getByText('Folder 2');
    expect(folder2.tagName).toBe('SPAN');
    expect(folder2).toHaveAttribute('aria-current', 'page');
  });

  it('calls onNavigate when Home is clicked', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    fireEvent.click(screen.getByText('🏠 Home'));
    expect(onNavigate).toHaveBeenCalledWith({ id: 0, name: 'Home', path: '/' }, -1);
  });

  it('calls onNavigate when breadcrumb item is clicked', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    fireEvent.click(screen.getByText('Folder 1'));
    expect(onNavigate).toHaveBeenCalledWith(mockItems[0], 0);
  });

  it('responds to Enter key on breadcrumb links', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    const homeLink = screen.getByText('🏠 Home');
    fireEvent.keyDown(homeLink, { key: 'Enter' });
    expect(onNavigate).toHaveBeenCalledTimes(1);
  });

  it('has proper ARIA labels', () => {
    const onNavigate = vi.fn();
    render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    expect(screen.getByLabelText('Navigate to Home')).toBeInTheDocument();
    expect(screen.getByLabelText('Navigate to Folder 1')).toBeInTheDocument();
  });

  it('renders separators between items', () => {
    const onNavigate = vi.fn();
    const { container } = render(<Breadcrumb items={mockItems} onNavigate={onNavigate} />);
    const separators = container.querySelectorAll('.breadcrumb-separator');
    expect(separators.length).toBe(2); // One for each item after Home
  });
});
