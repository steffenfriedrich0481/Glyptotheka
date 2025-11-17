import { TileMetadata } from '../types/tile';
import { ProjectWithChildren } from '../types/project';
import { formatBytes } from './formatBytes';

const metadataCache = new Map<number, TileMetadata>();

export function calculateTileMetadata(project: ProjectWithChildren): TileMetadata {
  // Check cache first
  const cached = metadataCache.get(project.id);
  if (cached) {
    return cached;
  }

  const isFolder = !project.is_leaf;
  const fileCount = project.stl_count + project.image_count;
  
  // For folders, we only know the immediate children's files
  // Total size would require recursive calculation on backend
  const totalSize = 0; // Backend doesn't currently provide total size
  
  const metadata: TileMetadata = {
    fileCount,
    totalSize,
    formattedSize: formatBytes(totalSize),
    isFolder,
  };

  // Cache the result
  metadataCache.set(project.id, metadata);
  
  return metadata;
}

// Clear cache when needed (e.g., after scan)
export function clearMetadataCache() {
  metadataCache.clear();
}
