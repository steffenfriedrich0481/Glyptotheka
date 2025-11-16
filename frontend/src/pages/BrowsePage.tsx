import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { projectsAPI } from '../api/projects';
import { Project } from '../types/project';
import LoadingSpinner from '../components/common/LoadingSpinner';

const BrowsePage: React.FC = () => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      const data = await projectsAPI.listRoot();
      setProjects(data);
    } catch (err) {
      console.error('Failed to load projects:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <LoadingSpinner />;

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-8">Browse Projects</h1>
      <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4">
        {projects.map((project) => (
          <div
            key={project.id}
            className="bg-white shadow-md rounded p-4 cursor-pointer hover:shadow-lg transition-shadow"
            onClick={() => navigate(`/project/${project.id}`)}
          >
            <h3 className="font-bold text-lg">{project.name}</h3>
            <p className="text-sm text-gray-600">{project.isLeaf ? 'Project' : 'Folder'}</p>
          </div>
        ))}
      </div>
      {projects.length === 0 && (
        <p className="text-center text-gray-500 mt-8">No projects found. Run a scan first.</p>
      )}
    </div>
  );
};

export default BrowsePage;
