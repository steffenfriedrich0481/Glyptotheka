import React, { useRef, useEffect, KeyboardEvent } from 'react';
import { Project } from '../../types/project';

interface Props {
  projects: Project[];
  onProjectClick: (id: number) => void;
}

const ProjectGrid: React.FC<Props> = ({ projects, onProjectClick }) => {
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Focus first item when projects load
    const firstItem = gridRef.current?.querySelector('[role="button"]') as HTMLElement;
    if (firstItem && projects.length > 0) {
      firstItem.tabIndex = 0;
    }
  }, [projects]);

  const handleKeyDown = (e: KeyboardEvent<HTMLDivElement>, index: number, projectId: number) => {
    const items = gridRef.current?.querySelectorAll('[role="button"]');
    if (!items) return;

    const cols = window.innerWidth >= 1024 ? 4 : window.innerWidth >= 768 ? 3 : 1;
    let nextIndex = index;

    switch (e.key) {
      case 'Enter':
      case ' ':
        e.preventDefault();
        onProjectClick(projectId);
        break;
      case 'ArrowRight':
        e.preventDefault();
        nextIndex = Math.min(index + 1, items.length - 1);
        break;
      case 'ArrowLeft':
        e.preventDefault();
        nextIndex = Math.max(index - 1, 0);
        break;
      case 'ArrowDown':
        e.preventDefault();
        nextIndex = Math.min(index + cols, items.length - 1);
        break;
      case 'ArrowUp':
        e.preventDefault();
        nextIndex = Math.max(index - cols, 0);
        break;
      case 'Home':
        e.preventDefault();
        nextIndex = 0;
        break;
      case 'End':
        e.preventDefault();
        nextIndex = items.length - 1;
        break;
      default:
        return;
    }

    // Update focus and tabindex
    if (nextIndex !== index) {
      (items[index] as HTMLElement).tabIndex = -1;
      (items[nextIndex] as HTMLElement).tabIndex = 0;
      (items[nextIndex] as HTMLElement).focus();
    }
  };

  return (
    <div 
      ref={gridRef}
      className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4"
      role="grid"
      aria-label="Projects grid"
    >
      {projects.map((project, index) => (
        <div
          key={project.id}
          role="button"
          tabIndex={index === 0 ? 0 : -1}
          className="bg-white shadow-md rounded p-4 cursor-pointer hover:shadow-lg transition-shadow focus:outline-none focus:ring-2 focus:ring-blue-500"
          onClick={() => onProjectClick(project.id)}
          onKeyDown={(e) => handleKeyDown(e, index, project.id)}
          aria-label={`${project.name}, ${project.is_leaf ? 'Project' : 'Folder'}`}
        >
          <h3 className="font-bold text-lg">{project.name}</h3>
          <p className="text-sm text-gray-600">
            {project.is_leaf ? 'Project' : 'Folder'}
          </p>
        </div>
      ))}
    </div>
  );
};

export default ProjectGrid;
