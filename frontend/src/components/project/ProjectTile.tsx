import React from 'react';
import { ProjectWithChildren } from '../../types/project';
import { calculateTileMetadata } from '../../utils/tileMetadata';
import './ProjectTile.css';

interface Props {
  project: ProjectWithChildren;
  onClick: () => void;
}

const ProjectTile: React.FC<Props> = ({ project, onClick }) => {
  const metadata = calculateTileMetadata(project);
  const isFolder = metadata.isFolder;

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
      aria-label={`${isFolder ? 'Folder' : 'Project'}: ${project.name}`}
    >
      {/* Preview Image/Icon */}
      <div className="project-tile__preview aspect-square bg-gradient-to-br from-gray-100 to-gray-200 dark:from-gray-700 dark:to-gray-800 flex items-center justify-center relative">
        {isFolder ? (
          <svg className="w-20 h-20 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
          </svg>
        ) : (
          <svg className="w-20 h-20 text-primary-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
          </svg>
        )}
        
        {/* Type Badge */}
        <div className={`absolute top-2 right-2 px-2 py-1 rounded-full text-xs font-medium ${
          isFolder 
            ? 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200' 
            : 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
        }`}>
          {isFolder ? 'Folder' : 'Project'}
        </div>
      </div>

      {/* Content */}
      <div className="p-4">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 truncate mb-2">
          {project.name}
        </h3>
        
        {/* Metadata */}
        <div className="flex items-center gap-3 text-sm text-gray-600 dark:text-gray-400">
          <div className="flex items-center gap-1">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z" />
            </svg>
            <span>{metadata.fileCount} files</span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProjectTile;
