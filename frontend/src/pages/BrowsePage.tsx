import React, { useState, useEffect } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { browseAPI, FolderContents, FolderInfo } from '../api/browse';
import ProjectGrid from '../components/project/ProjectGrid';
import Breadcrumb from '../components/common/Breadcrumb';
import { NoProjectsFound } from '../components/common/EmptyState';

interface BreadcrumbItem {
  id: number;
  name: string;
  path: string;
}

const BrowsePage: React.FC = () => {
  const [folderContents, setFolderContents] = useState<FolderContents | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();
  const location = useLocation();

  // Extract current path from URL
  const getCurrentPath = (): string => {
    const path = location.pathname.replace('/browse', '').replace(/^\//, '');
    return path;
  };

  const currentPath = getCurrentPath();

  useEffect(() => {
    loadFolderContents();
  }, [currentPath]);

  const loadFolderContents = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await browseAPI.getFolderContents(currentPath);
      setFolderContents(data);
    } catch (err) {
      console.error('Failed to load folder contents:', err);
      setError('Failed to load folder contents');
    } finally {
      setLoading(false);
    }
  };

  const handleFolderClick = (folder: FolderInfo) => {
    const newPath = folder.path;
    navigate(`/browse/${newPath}`);
  };

  const handleProjectClick = (projectId: number) => {
    navigate(`/project/${projectId}`);
  };

  const handleBreadcrumbClick = (_item: BreadcrumbItem, index: number) => {
    if (index === -1) {
      // Navigate to root
      navigate('/browse');
    } else {
      // Navigate to specific breadcrumb using its path
      const pathParts = currentPath.split('/').filter(p => p);
      const targetPath = pathParts.slice(0, index + 1).join('/');
      navigate(`/browse/${targetPath}`);
    }
  };

  // Convert path to breadcrumbs
  const getBreadcrumbs = (): BreadcrumbItem[] => {
    if (!currentPath) return [];
    
    const parts = currentPath.split('/').filter(p => p);
    return parts.map((part, index) => ({
      id: index,
      name: part,
      path: parts.slice(0, index + 1).join('/'),
    }));
  };

  // Combine folders and projects for display
  const combinedItems = React.useMemo(() => {
    if (!folderContents) return [];
    
    // Convert folders to pseudo-projects for grid display
    const folderItems = folderContents.folders.map((folder, idx) => ({
      id: -1 - idx, // Use negative IDs to distinguish folders
      name: folder.name,
      path: folder.path,
      is_leaf: false,
      has_stl_files: false,
      _isFolder: true as const,
      _folderData: folder,
    }));

    // Convert projects with preview images
    const projectItems = folderContents.projects.map(project => ({
      ...project,
      _isFolder: false as const,
    }));

    return [...folderItems, ...projectItems];
  }, [folderContents]);

  const breadcrumbs = getBreadcrumbs();

  if (error) {
    return (
      <div className="max-w-7xl mx-auto">
        <div className="text-center py-12">
          <p className="text-red-600 dark:text-red-400">{error}</p>
          <button
            onClick={loadFolderContents}
            className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Retry
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto">
      <div className="mb-6">
        <h1 id="main-content" className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-4" tabIndex={-1}>
          {currentPath ? 'Browse Folder' : 'Browse Projects'}
        </h1>
        <Breadcrumb items={breadcrumbs} onNavigate={handleBreadcrumbClick} />
      </div>

      {loading ? (
        <ProjectGrid projects={[]} onProjectClick={() => {}} loading={true} />
      ) : combinedItems.length === 0 ? (
        <NoProjectsFound />
      ) : (
        <ProjectGrid 
          projects={combinedItems as any} 
          onProjectClick={(id) => {
            // Find the item
            const item = combinedItems.find(i => i.id === id);
            if (!item) return;
            
            if (item._isFolder && '_folderData' in item) {
              handleFolderClick(item._folderData);
            } else {
              handleProjectClick(id);
            }
          }} 
        />
      )}
    </div>
  );
};

export default BrowsePage;
