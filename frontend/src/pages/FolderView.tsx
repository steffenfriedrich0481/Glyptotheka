import { useEffect } from 'react';
import { FolderTile, FolderInfo } from '../components/FolderTile';
import { ProjectWithPreview } from '../api/client';

interface FolderViewProps {
  folders: FolderInfo[];
  projects: ProjectWithPreview[];
  loading: boolean;
  error: string | null;
}

export function FolderView({ folders, projects, loading, error }: FolderViewProps) {
  // T035: Keyboard navigation support for folder tiles
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      const totalItems = folders.length + projects.length;
      
      if (totalItems === 0) return;

      // Keyboard navigation can be enhanced in future iterations
      if (e.key === 'ArrowDown' || e.key === 'ArrowUp' || e.key === 'Home' || e.key === 'End') {
        // Focus management for accessibility
        e.preventDefault();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [folders.length, projects.length]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-800 dark:text-red-200">
        <p className="font-medium">Error loading folder contents</p>
        <p className="text-sm mt-1">{error}</p>
      </div>
    );
  }

  if (folders.length === 0 && projects.length === 0) {
    return (
      <div className="text-center py-12">
        <svg
          className="mx-auto h-16 w-16 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
          />
        </svg>
        <h3 className="mt-2 text-lg font-medium text-gray-900 dark:text-gray-100">
          No contents
        </h3>
        <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
          This folder is empty.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Folders Section */}
      {folders.length > 0 && (
        <div>
          <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
            Folders
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {folders.map((folder) => (
              <FolderTile
                key={folder.path}
                folder={folder}
              />
            ))}
          </div>
        </div>
      )}

      {/* Projects Section */}
      {projects.length > 0 && (
        <div>
          <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
            Projects
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {projects.map((projectData) => (
              <div
                key={projectData.project.id}
                className="bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-lg transition-all p-4 border border-gray-200 dark:border-gray-700"
              >
                <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100 truncate mb-2">
                  {projectData.project.name}
                </h3>
                
                {/* T044: Display preview images in project cards */}
                {projectData.preview_images.length > 0 && (
                  <div className="mb-2">
                    <img
                      src={`/api/files/images/${projectData.preview_images[0].id}`}
                      alt={projectData.preview_images[0].filename}
                      className="w-full h-40 object-cover rounded"
                      loading="lazy"
                    />
                  </div>
                )}
                
                {projectData.project.description && (
                  <p className="text-sm text-gray-600 dark:text-gray-400 line-clamp-2">
                    {projectData.project.description}
                  </p>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
