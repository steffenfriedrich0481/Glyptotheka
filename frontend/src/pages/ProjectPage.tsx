import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { projectsAPI } from '../api/projects';
import { ProjectWithRelations } from '../types/project';
import LoadingSpinner from '../components/common/LoadingSpinner';

const ProjectPage: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [project, setProject] = useState<ProjectWithRelations | null>(null);
  const [loading, setLoading] = useState(true);
  const navigate = useNavigate();

  useEffect(() => {
    if (id) {
      loadProject(parseInt(id));
    }
  }, [id]);

  const loadProject = async (projectId: number) => {
    try {
      const data = await projectsAPI.getProject(projectId);
      setProject(data);
    } catch (err) {
      console.error('Failed to load project:', err);
    } finally {
      setLoading(false);
    }
  };

  if (loading) return <LoadingSpinner />;
  if (!project) return <div>Project not found</div>;

  return (
    <div className="container mx-auto p-8">
      <button
        onClick={() => navigate(-1)}
        className="text-blue-500 hover:text-blue-700 mb-4"
      >
        ← Back
      </button>

      <h1 className="text-3xl font-bold mb-4">{project.name}</h1>
      <p className="text-gray-600 mb-8">{project.fullPath}</p>

      {project.children.length > 0 && (
        <div className="mb-8">
          <h2 className="text-2xl font-bold mb-4">Sub-projects</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {project.children.map((child) => (
              <div
                key={child.id}
                className="bg-white shadow-md rounded p-4 cursor-pointer hover:shadow-lg"
                onClick={() => navigate(`/project/${child.id}`)}
              >
                <h3 className="font-bold">{child.name}</h3>
              </div>
            ))}
          </div>
        </div>
      )}

      <div>
        <h2 className="text-2xl font-bold mb-4">Files</h2>
        <p>STL files: {project.stl_count}</p>
        <p>Images: {project.image_count}</p>
      </div>
    </div>
  );
};

export default ProjectPage;
