import React from 'react';
import { Project } from '../../types/project';

interface Props {
  projects: Project[];
  onProjectClick: (id: number) => void;
}

const ProjectGrid: React.FC<Props> = ({ projects, onProjectClick }) => {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4">
      {projects.map((project) => (
        <div
          key={project.id}
          className="bg-white shadow-md rounded p-4 cursor-pointer hover:shadow-lg transition-shadow"
          onClick={() => onProjectClick(project.id)}
        >
          <h3 className="font-bold text-lg">{project.name}</h3>
          <p className="text-sm text-gray-600">
            {project.isLeaf ? 'Project' : 'Folder'}
          </p>
        </div>
      ))}
    </div>
  );
};

export default ProjectGrid;
