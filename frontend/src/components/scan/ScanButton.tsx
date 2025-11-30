import React, { useState, useEffect } from 'react';
import { scanAPI } from '../../api/scan';

interface ScanButtonProps {
  className?: string;
}

export const ScanButton: React.FC<ScanButtonProps> = ({ className = '' }) => {
  const [isScanning, setIsScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState<any>(null);

  useEffect(() => {
    if (isScanning) {
      const interval = setInterval(async () => {
        try {
          const status = await scanAPI.getScanStatus();
          setProgress(status);
          if (!status.is_scanning) {
            setIsScanning(false);
            clearInterval(interval);
          }
        } catch (err) {
          console.error('Failed to get scan status:', err);
        }
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [isScanning]);

  const handleScan = async () => {
    setError(null);
    try {
      await scanAPI.startScan(false);
      setIsScanning(true);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to start scan');
    }
  };

  return (
    <div className={className}>
      <button
        onClick={handleScan}
        disabled={isScanning}
        className="btn-primary flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
        title={isScanning ? 'Scanning in progress...' : 'Rescan library for new projects'}
      >
        {isScanning ? (
          <>
            <svg className="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>Scanning...</span>
          </>
        ) : (
          <>
            <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
            <span>Rescan</span>
          </>
        )}
      </button>
      {error && (
        <div className="absolute right-0 top-12 bg-red-100 border border-red-400 text-red-700 px-4 py-2 rounded shadow-lg z-50 max-w-xs">
          {error}
        </div>
      )}
      {progress && isScanning && (
        <div className="absolute right-0 top-12 bg-white dark:bg-theme-lighter border border-gray-300 dark:border-theme px-4 py-2 rounded shadow-lg z-50">
          <div className="text-sm">
            <div>Projects: {progress.projects_found || 0}</div>
            <div>Files: {progress.files_processed || 0}</div>
          </div>
        </div>
      )}
    </div>
  );
};
