import React from 'react';

export const SkeletonTile: React.FC = () => {
  return (
    <div className="card overflow-hidden">
      <div className="aspect-square bg-gray-200 dark:bg-theme-lighter animate-pulse-subtle"></div>
      <div className="p-4 space-y-3">
        <div className="h-5 bg-gray-200 dark:bg-theme-lighter rounded animate-pulse-subtle" style={{ width: '75%' }}></div>
        <div className="space-y-2">
          <div className="h-4 bg-gray-200 dark:bg-theme-lighter rounded animate-pulse-subtle" style={{ width: '50%' }}></div>
          <div className="h-4 bg-gray-200 dark:bg-theme-lighter rounded animate-pulse-subtle" style={{ width: '40%' }}></div>
        </div>
      </div>
    </div>
  );
};
