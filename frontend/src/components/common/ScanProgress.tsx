import React from 'react';
import { ScanStatus } from '../../api/scan';

interface ScanProgressProps {
  status: ScanStatus;
}

const ScanProgress: React.FC<ScanProgressProps> = ({ status }) => {
  if (!status.is_scanning && status.projects_found === undefined) {
    return null;
  }

  const isComplete = !status.is_scanning && status.projects_found !== undefined;

  return (
    <div
      className={`border-l-4 p-4 ${
        isComplete
          ? 'bg-green-100 border-green-500 text-green-700'
          : 'bg-blue-100 border-blue-500 text-blue-700'
      }`}
    >
      <p className="font-bold">
        {isComplete ? 'Scan complete!' : 'Scan in progress...'}
      </p>

      {status.projects_found !== undefined && (
        <div className="mt-2 space-y-1">
          <p>Projects found: {status.projects_found}</p>
          
          {status.projects_added !== undefined && status.projects_added > 0 && (
            <p className="text-green-600">
              ✓ Added: {status.projects_added} projects
            </p>
          )}
          
          {status.projects_updated !== undefined && status.projects_updated > 0 && (
            <p className="text-blue-600">
              ↻ Updated: {status.projects_updated} projects
            </p>
          )}
          
          {status.projects_removed !== undefined && status.projects_removed > 0 && (
            <p className="text-red-600">
              ✗ Removed: {status.projects_removed} projects
            </p>
          )}
        </div>
      )}

      {status.files_processed !== undefined && (
        <div className="mt-2 space-y-1">
          <p>Files processed: {status.files_processed}</p>
          
          {status.files_added !== undefined && status.files_added > 0 && (
            <p className="text-green-600">
              ✓ Added: {status.files_added} files
            </p>
          )}
          
          {status.files_updated !== undefined && status.files_updated > 0 && (
            <p className="text-blue-600">
              ↻ Updated: {status.files_updated} files
            </p>
          )}
          
          {status.files_removed !== undefined && status.files_removed > 0 && (
            <p className="text-red-600">
              ✗ Removed: {status.files_removed} files
            </p>
          )}
        </div>
      )}

      {status.errors && status.errors.length > 0 && (
        <div className="mt-3">
          <p className="font-semibold text-red-600">Errors:</p>
          <ul className="list-disc list-inside text-sm">
            {status.errors.slice(0, 5).map((error, idx) => (
              <li key={idx} className="text-red-600">
                {error}
              </li>
            ))}
            {status.errors.length > 5 && (
              <li className="text-red-600">
                ... and {status.errors.length - 5} more errors
              </li>
            )}
          </ul>
        </div>
      )}
    </div>
  );
};

export default ScanProgress;
