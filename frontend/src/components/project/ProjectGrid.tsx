import React, { useRef, useEffect } from 'react';
import { ProjectWithChildren } from '../../types/project';
import ProjectTile from './ProjectTile';
import { SkeletonTile } from './SkeletonTile';
import { EmptyState } from '../common/EmptyState';
import './ProjectGrid.css';

interface Props {
  projects: ProjectWithChildren[];
  onProjectClick: (id: number) => void;
  loading?: boolean;
}

const ProjectGrid: React.FC<Props> = ({ projects, onProjectClick, loading = false }) => {
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Focus first item when projects load
    const firstItem = gridRef.current?.querySelector('[role="button"]') as HTMLElement;
    if (firstItem && projects.length > 0) {
      firstItem.tabIndex = 0;
    }
  }, [projects]);

  if (loading) {
    return (
      <div className="project-grid project-grid--loading" aria-busy="true" aria-label="Loading projects">
        {[...Array(8)].map((_, i) => (
          <SkeletonTile key={i} />
        ))}
      </div>
    );
  }

  if (projects.length === 0) {
    return (
      <EmptyState
        title="No projects found"
        description="This folder doesn't contain any projects yet. Click Rescan to refresh your library."
        icon={
          <svg className="w-24 h-24" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        }
      />
    );
  }

  return (
    <div 
      ref={gridRef}
      className="project-grid"
      role="grid"
      aria-label="Projects grid"
    >
      {projects.map((project) => (
        <ProjectTile
          key={project.id}
          project={project}
          onClick={() => onProjectClick(project.id)}
        />
      ))}
    </div>
  );
};

export default React.memo(ProjectGrid);
