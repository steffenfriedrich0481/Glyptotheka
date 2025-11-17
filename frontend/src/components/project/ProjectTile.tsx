import React from 'react';
import { Project } from '../../types/project';

interface Props {
  project: Project;
  onClick: () => void;
}

const ProjectTile: React.FC<Props> = ({ project, onClick }) => {
  return (
    <div
      className="bg-white shadow-md rounded-lg p-6 cursor-pointer hover:shadow-xl transition-shadow"
      onClick={onClick}
    >
      <div className="flex flex-col items-center">
        <div className="w-full h-32 bg-gray-200 rounded mb-4 flex items-center justify-center">
          <span className="text-gray-400 text-4xl">📁</span>
        </div>
        <h3 className="text-lg font-semibold text-center">{project.name}</h3>
        <p className="text-sm text-gray-500 mt-1">
          {project.is_leaf ? 'Project' : 'Folder'}
        </p>
      </div>
    </div>
  );
};

export default ProjectTile;
