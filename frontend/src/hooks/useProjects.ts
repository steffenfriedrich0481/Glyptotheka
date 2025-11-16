import { useState, useEffect } from 'react';
import { projectsAPI } from '../api/projects';
import { Project, ProjectWithRelations } from '../types/project';

export const useProjects = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const loadRootProjects = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await projectsAPI.listRoot();
      setProjects(data);
    } catch (err) {
      setError(err as Error);
    } finally {
      setLoading(false);
    }
  };

  const loadProject = async (id: number): Promise<ProjectWithRelations | null> => {
    setLoading(true);
    setError(null);
    try {
      const data = await projectsAPI.getProject(id);
      return data;
    } catch (err) {
      setError(err as Error);
      return null;
    } finally {
      setLoading(false);
    }
  };

  return {
    projects,
    loading,
    error,
    loadRootProjects,
    loadProject,
  };
};
