import React from 'react';

export const SkeletonTile: React.FC = () => {
  return (
    <div className="card animate-pulse p-4">
      <div className="aspect-square bg-gray-300 dark:bg-gray-700 rounded-lg mb-4"></div>
      <div className="h-4 bg-gray-300 dark:bg-gray-700 rounded w-3/4 mb-2"></div>
      <div className="h-3 bg-gray-300 dark:bg-gray-700 rounded w-1/2"></div>
    </div>
  );
};
