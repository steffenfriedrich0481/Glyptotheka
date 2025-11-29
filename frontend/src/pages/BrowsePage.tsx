import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { fetchFolderContents, fetchBreadcrumb, FolderContents, BreadcrumbItem as ApiBreadcrumbItem } from '../api/client';
import { Breadcrumb } from '../components/Breadcrumb';
import { FolderView } from '../pages/FolderView';

const BrowsePage: React.FC = () => {
  const [folderContents, setFolderContents] = useState<FolderContents | null>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<ApiBreadcrumbItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const params = useParams<{ '*': string }>();

  // T031: Extract current path from URL (supports /browse/* wildcard route)
  const getCurrentPath = (): string => {
    return params['*'] || '';
  };

  const currentPath = getCurrentPath();

  // T034: Load folder contents and breadcrumbs when path changes
  useEffect(() => {
    loadFolderData();
  }, [currentPath]);

  const loadFolderData = async () => {
    setLoading(true);
    setError(null);
    try {
      // T027: Fetch folder contents with automatic request cancellation
      const [contentsData, breadcrumbData] = await Promise.all([
        fetchFolderContents(currentPath),
        fetchBreadcrumb(currentPath),
      ]);
      
      setFolderContents(contentsData);
      setBreadcrumbs(breadcrumbData);
    } catch (err: any) {
      // Ignore cancellation errors from rapid navigation
      if (err.message !== 'New folder request initiated' && err.message !== 'New breadcrumb request initiated') {
        console.error('Failed to load folder contents:', err);
        setError(err.message || 'Failed to load folder contents');
      }
    } finally {
      setLoading(false);
    }
  };

  // T034: Browser back/forward history support (React Router handles this automatically)

  if (error) {
    return (
      <div className="max-w-7xl mx-auto p-6">
        <div className="text-center py-12">
          <p className="text-red-600 dark:text-red-400">{error}</p>
          <button
            onClick={loadFolderData}
            className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto p-6">
      <div className="mb-6">
        <h1
          id="main-content"
          className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-4"
          tabIndex={-1}
        >
          {currentPath ? 'Browse Folder' : 'Browse Projects'}
        </h1>
        {/* T032: Integrate Breadcrumb component */}
        <Breadcrumb items={breadcrumbs} currentPath={currentPath} />
      </div>

      {/* Show project details if this path IS a project */}
      {folderContents?.is_leaf_project && folderContents?.project_details ? (
        <div>
          <h2 className="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4">
            {folderContents.project_details.project.name}
          </h2>
          
          {/* STL Categories */}
          {folderContents.project_details.stl_categories.length > 0 && (
            <div className="mb-8">
              <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-200 mb-4">STL Files</h3>
              {folderContents.project_details.stl_categories.map((category, idx) => (
                <div key={idx} className="mb-6">
                  <h4 className="text-lg font-medium text-gray-700 dark:text-gray-300 mb-2">
                    {category.category || 'Uncategorized'}
                  </h4>
                  <ul className="space-y-2">
                    {category.files.map((file) => (
                      <li key={file.id} className="text-gray-600 dark:text-gray-400">
                        {file.filename} ({(file.file_size / 1024 / 1024).toFixed(2)} MB)
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
          )}

          {/* Images */}
          {folderContents.project_details.images.length > 0 && (
            <div className="mb-8">
              <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-200 mb-4">
                Images ({folderContents.project_details.total_images})
              </h3>
              <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {folderContents.project_details.images.slice(0, 12).map((image) => (
                  <div key={image.id} className="relative aspect-square rounded-lg overflow-hidden bg-gray-200 dark:bg-gray-700">
                    <img
                      src={`/api/files/images/${image.id}`}
                      alt={image.filename}
                      className="w-full h-full object-cover"
                    />
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      ) : (
        /* T033: Integrate FolderView component for normal folder browsing */
        <FolderView
          folders={folderContents?.folders || []}
          projects={folderContents?.projects || []}
          loading={loading}
          error={error}
        />
      )}
    </div>
  );
};

export default BrowsePage;
