import { Link } from 'react-router-dom';

export interface FolderInfo {
  name: string;
  path: string;
  project_count: number;
  has_images: boolean;
}

interface FolderTileProps {
  folder: FolderInfo;
}

export function FolderTile({ folder }: FolderTileProps) {
  return (
    <Link
      to={`/browse/${folder.path}`}
      className="bg-white dark:bg-theme-lighter rounded-lg shadow-md hover:shadow-lg transition-all cursor-pointer p-6 border border-gray-200 dark:border-theme block"
      aria-label={`Open folder ${folder.name}`}
    >
      <div className="flex items-center space-x-4">
        <div className="flex-shrink-0">
          <svg
            className="w-12 h-12 text-blue-500 dark:text-blue-400"
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
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-theme truncate">
            {folder.name}
          </h3>
          <div className="flex items-center space-x-4 mt-1 text-sm text-gray-600 dark:text-theme-muted">
            <span>
              {folder.project_count} {folder.project_count === 1 ? 'project' : 'projects'}
            </span>
            {folder.has_images && (
              <span className="flex items-center">
                <svg className="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                  <path
                    fillRule="evenodd"
                    d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z"
                    clipRule="evenodd"
                  />
                </svg>
                Has images
              </span>
            )}
          </div>
        </div>
      </div>
    </Link>
  );
}
