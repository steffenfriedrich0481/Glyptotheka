import React, { useEffect, useRef } from 'react';
import { ScanStatus } from '../../api/scan';

interface ScanLogsPanelProps {
  status: ScanStatus | null;
  isScanning: boolean;
  onClose: () => void;
}

export const ScanLogsPanel: React.FC<ScanLogsPanelProps> = ({ status, isScanning, onClose }) => {
  const logsEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Auto-scroll to bottom when new logs appear
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [status]);

  if (!status && !isScanning) {
    return null;
  }

  return (
    <div className="fixed bottom-0 left-0 right-0 bg-theme-lighter border-t-2 border-theme shadow-2xl z-40 transition-all">
      {/* Header */}
      <div className="flex items-center justify-between px-4 py-2 bg-theme border-b border-theme-lighter">
        <div className="flex items-center gap-3">
          <h3 className="text-sm font-semibold text-theme">Scan Logs</h3>
          {isScanning && (
            <div className="flex items-center gap-2 text-xs text-theme-muted">
              <svg className="animate-spin h-4 w-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <span>Scanning...</span>
            </div>
          )}
        </div>
        <button
          onClick={onClose}
          className="text-theme-muted hover:text-theme transition-colors"
          aria-label="Close scan logs"
        >
          <svg className="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      {/* Logs Content */}
      <div className="h-64 overflow-y-auto px-4 py-3 bg-theme font-mono text-xs">
        {status && (
          <div className="space-y-1">
            {/* Stats */}
            <div className="text-theme-muted mb-2">
              {status.projects_found !== undefined && (
                <div className="text-green-600 dark:text-green-400">
                  ✓ Projects found: {status.projects_found}
                  {status.projects_added !== undefined && ` (+${status.projects_added} added`}
                  {status.projects_updated !== undefined && `, ${status.projects_updated} updated`}
                  {status.projects_removed !== undefined && `, ${status.projects_removed} removed)`}
                </div>
              )}
              {status.files_processed !== undefined && (
                <div className="text-blue-600 dark:text-blue-400">
                  ✓ Files processed: {status.files_processed}
                  {status.files_added !== undefined && ` (+${status.files_added} added`}
                  {status.files_updated !== undefined && `, ${status.files_updated} updated`}
                  {status.files_removed !== undefined && `, ${status.files_removed} removed)`}
                </div>
              )}
            </div>

            {/* Errors */}
            {status.errors && status.errors.length > 0 && (
              <div className="mt-3">
                <div className="text-red-600 dark:text-red-400 font-semibold mb-1">
                  ⚠ Errors ({status.errors.length}):
                </div>
                {status.errors.map((error, index) => (
                  <div key={index} className="text-red-500 dark:text-red-400 pl-4 py-0.5">
                    • {error}
                  </div>
                ))}
              </div>
            )}

            {/* Completion message */}
            {!isScanning && status.projects_found !== undefined && (
              <div className="text-green-600 dark:text-green-400 font-semibold mt-3">
                ✓ Scan completed successfully
              </div>
            )}
          </div>
        )}

        {/* Empty state while scanning */}
        {isScanning && !status && (
          <div className="text-theme-muted">
            Initializing scan...
          </div>
        )}

        {/* Auto-scroll anchor */}
        <div ref={logsEndRef} />
      </div>
    </div>
  );
};
