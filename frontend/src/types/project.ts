// Project types
export interface Project {
  id: number;
  name: string;
  fullPath: string;
  parentId: number | null;
  isLeaf: boolean;
  description: string | null;
  createdAt: number;
  updatedAt: number;
}

export interface ProjectWithChildren extends Project {
  children: Project[];
  stlCount: number;
  imageCount: number;
}

export interface ProjectWithRelations extends Project {
  children: Project[];
  stl_count: number;
  image_count: number;
}

// File types
export interface StlFile {
  id: number;
  projectId: number;
  filename: string;
  filePath: string;
  fileSize: number;
  previewPath: string | null;
  previewGeneratedAt: number | null;
}

export interface ImageFile {
  id: number;
  projectId: number;
  filename: string;
  filePath: string;
  fileSize: number;
  sourceType: 'direct' | 'inherited';
  sourceProjectId: number | null;
  displayOrder: number;
}

// Tag types
export interface Tag {
  id: number;
  name: string;
  color: string | null;
  usageCount: number;
}

// Scan types
export interface ScanSession {
  id: number;
  rootPath: string;
  status: 'running' | 'completed' | 'failed' | 'cancelled';
  startedAt: number;
  completedAt: number | null;
  projectsFound: number;
  filesProcessed: number;
  errorsCount: number;
}
