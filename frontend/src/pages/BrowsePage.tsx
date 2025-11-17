import React, { useState, useEffect, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { projectsAPI } from '../api/projects';
import { ProjectWithChildren } from '../types/project';
import ProjectGrid from '../components/project/ProjectGrid';
import Breadcrumb from '../components/common/Breadcrumb';
import { NoProjectsFound } from '../components/common/EmptyState';

interface BreadcrumbItem {
  id: number;
  name: string;
  path: string;
}

const BrowsePage: React.FC = () => {
  const [projects, setProjects] = useState<ProjectWithChildren[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentFolderId, setCurrentFolderId] = useState<number | null>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<BreadcrumbItem[]>([]);
  const navigate = useNavigate();

  useEffect(() => {
    loadProjects();
  }, [currentFolderId]);

  const loadProjects = async () => {
    setLoading(true);
    try {
      if (currentFolderId === null) {
        // Load root projects
        const data = await projectsAPI.listRoot();
        setProjects(data as ProjectWithChildren[]);
      } else {
        // Load children of current folder
        const data = await projectsAPI.getProjectChildren(currentFolderId);
        setProjects(data as ProjectWithChildren[]);
      }
    } catch (err) {
      console.error('Failed to load projects:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleTileClick = async (projectId: number) => {
    const project = projects.find(p => p.id === projectId);
    if (!project) return;

    if (project.is_leaf) {
      // Navigate to project detail page
      navigate(`/project/${projectId}`);
    } else {
      // Navigate into folder
      setCurrentFolderId(projectId);
      setBreadcrumbs([...breadcrumbs, { id: projectId, name: project.name, path: '' }]);
    }
  };

  const handleBreadcrumbClick = (item: BreadcrumbItem, index: number) => {
    if (index === -1) {
      // Navigate to root
      setCurrentFolderId(null);
      setBreadcrumbs([]);
    } else {
      // Navigate to specific breadcrumb
      setCurrentFolderId(item.id);
      setBreadcrumbs(breadcrumbs.slice(0, index + 1));
    }
  };

  const visibleProjects = useMemo(() => {
    return projects;
  }, [projects]);

  return (
    <div className="max-w-7xl mx-auto">
      <div className="mb-6">
        <h1 id="main-content" className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-4" tabIndex={-1}>
          {currentFolderId === null ? 'Browse Projects' : 'Browse Folder'}
        </h1>
        <Breadcrumb items={breadcrumbs} onNavigate={handleBreadcrumbClick} />
      </div>

      {loading ? (
        <ProjectGrid projects={[]} onProjectClick={() => {}} loading={true} />
      ) : visibleProjects.length === 0 ? (
        <NoProjectsFound />
      ) : (
        <ProjectGrid projects={visibleProjects} onProjectClick={handleTileClick} />
      )}
    </div>
  );
};

export default BrowsePage;
