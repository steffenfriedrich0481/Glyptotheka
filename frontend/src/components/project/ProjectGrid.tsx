import React, { useRef, useEffect } from 'react';
import { ProjectWithChildren } from '../../types/project';
import ProjectTile from './ProjectTile';
import { SkeletonTile } from './SkeletonTile';
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
      <div className="project-grid">
        {[...Array(8)].map((_, i) => (
          <SkeletonTile key={i} />
        ))}
      </div>
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

export default ProjectGrid;
