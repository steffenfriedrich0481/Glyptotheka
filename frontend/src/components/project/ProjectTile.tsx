import React from 'react';
import { ProjectWithChildren, SearchResultProject } from '../../types/project';
import { calculateTileMetadata } from '../../utils/tileMetadata';
import { SearchTileCarousel } from './SearchTileCarousel';
import './ProjectTile.css';

interface Props {
  project: ProjectWithChildren | SearchResultProject;
  onClick: () => void;
}

const ProjectTile: React.FC<Props> = ({ project, onClick }) => {
  const metadata = calculateTileMetadata(project);
  const isFolder = metadata.isFolder;
  const hasChildren = 'children' in project && project.children && project.children.length > 0;
  const childCount = 'children' in project && project.children ? project.children.length : 0;

  return (
    <div
      className="project-tile card cursor-pointer transition-all duration-200 overflow-hidden"
      onClick={onClick}
      role="button"
      tabIndex={0}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onClick();
        }
      }}
      aria-label={`${isFolder ? 'Folder' : 'Project'}: ${project.name}${hasChildren ? `, ${childCount} items` : ''}`}
    >
      {/* Preview Image/Icon */}
      <div className="project-tile__preview aspect-square bg-white flex items-center justify-center relative group-hover:bg-gray-50">
        {'images' in project && project.images && project.images.length > 0 ? (
          <SearchTileCarousel images={project.images} projectName={project.name} />
        ) : isFolder ? (
          <svg className="w-20 h-20 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        ) : (
          <svg className="w-20 h-20 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
          </svg>
        )}
      </div>

      {/* Content */}
      <div className="p-4 space-y-3">
        <h3 className="text-base font-semibold text-gray-900 dark:text-gray-100 line-clamp-2 leading-snug min-h-[2.5rem]" title={project.name}>
          {project.name}
        </h3>
        
        {/* Author & Stats */}
        <div className="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <div className="flex items-center gap-1.5 font-medium">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            <span className="truncate max-w-[120px]">Creator</span>
          </div>
          
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M12 21.35l-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54L12 21.35z"/>
              </svg>
              <span>{project.stl_count}</span>
            </div>
            <div className="flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
              <span>{childCount || project.image_count}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default React.memo(ProjectTile);
