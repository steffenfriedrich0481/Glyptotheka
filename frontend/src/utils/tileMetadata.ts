import { TileMetadata } from '../types/tile';
import { ProjectWithChildren } from '../types/project';
import { formatBytes } from './formatBytes';

export function calculateTileMetadata(project: ProjectWithChildren): TileMetadata {
  const isFolder = !project.is_leaf;
  const fileCount = project.stl_count + project.image_count;
  
  // For folders, we only know the immediate children's files
  // Total size would require recursive calculation on backend
  const totalSize = 0; // Backend doesn't currently provide total size
  
  return {
    fileCount,
    totalSize,
    formattedSize: formatBytes(totalSize),
    isFolder,
  };
}
