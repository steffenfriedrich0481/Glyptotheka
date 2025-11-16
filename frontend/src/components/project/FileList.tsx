import React from 'react';
import { StlFile } from '../../types/project';

interface Props {
  files: StlFile[];
}

const FileList: React.FC<Props> = ({ files }) => {
  return (
    <div className="bg-white shadow rounded-lg p-4">
      <h3 className="font-bold text-lg mb-4">STL Files</h3>
      <ul className="space-y-2">
        {files.map((file) => (
          <li key={file.id} className="flex justify-between items-center p-2 hover:bg-gray-50">
            <span className="truncate">{file.filename}</span>
            <span className="text-sm text-gray-500">
              {(file.fileSize / 1024 / 1024).toFixed(2)} MB
            </span>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default FileList;
