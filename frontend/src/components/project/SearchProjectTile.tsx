import React from 'react';
import { SearchResultProject } from '../../types/project';
import { SearchTileCarousel } from './SearchTileCarousel';

interface SearchProjectTileProps {
  project: SearchResultProject;
  onClick: (id: number) => void;
}

export const SearchProjectTile: React.FC<SearchProjectTileProps> = ({ project, onClick }) => {
  // Logic to determine if we should show a keyword badge
  // We compare the project name with the last segment of the full path
  // If they are different, the last segment is likely a keyword/variant
  // We also need to handle potential trailing slashes or empty segments
  const cleanPath = project.full_path.replace(/\/$/, '');
  const pathSegments = cleanPath.split('/');
  const lastSegment = pathSegments[pathSegments.length - 1];
  
  // Check if the last segment is different from the name (case-insensitive check might be safer but exact match is good for now)
  const showKeyword = lastSegment && lastSegment !== project.name;
  const keyword = lastSegment;

  return (
    <div 
      className="search-page__project group flex flex-col h-full" 
      onClick={() => onClick(project.id)}
      role="article"
      aria-label={`Project ${project.name}`}
    >
      {/* Image Carousel Container */}
      <div className="relative aspect-square w-full mb-3 overflow-hidden rounded-lg bg-gray-100 dark:bg-gray-800">
        <SearchTileCarousel 
          images={project.images} 
          projectName={project.name}
          autoAdvance={false}
        />
        
        {/* Keyword Badge */}
        {showKeyword && (
          <div className="absolute top-2 right-2 bg-black/70 text-white text-xs font-bold px-2 py-1 rounded backdrop-blur-sm z-10 shadow-sm">
            {keyword}
          </div>
        )}
      </div>

      {/* Project Info */}
      <div className="search-page__project-info flex-1 flex flex-col">
        <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-1 truncate" title={project.name}>
          {project.name}
        </h3>
        <p className="text-sm text-gray-500 dark:text-gray-400 truncate mb-2" title={project.full_path}>
          {project.full_path}
        </p>
        
        <div className="mt-auto flex items-center gap-3 text-xs text-gray-500 dark:text-gray-400">
           <span className="flex items-center gap-1">
             <span className="font-medium">{project.stl_count}</span> STLs
           </span>
           <span className="w-1 h-1 rounded-full bg-gray-300 dark:bg-gray-600"></span>
           <span className="flex items-center gap-1">
             <span className="font-medium">{project.image_count}</span> Images
           </span>
        </div>
      </div>
    </div>
  );
};
