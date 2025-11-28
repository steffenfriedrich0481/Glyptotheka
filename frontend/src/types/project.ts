// Project types
export interface Project {
  id: number;
  name: string;
  full_path: string;
  parent_id: number | null;
  is_leaf: boolean;
  description: string | null;
  created_at: number;
  updated_at: number;
}

export interface ImagePreview {
  id: number;
  filename: string;
  source_type: string;
  image_source: string;
  priority: number;
  inherited_from?: string | null;  // T041: Add inherited_from path
}

export interface SearchResultProject extends Project {
  stl_count: number;
  image_count: number;
  images: ImagePreview[];
}

export interface ProjectWithChildren extends Project {
  children: Project[];
  stl_count: number;
  image_count: number;
  tags: Tag[];
  preview_images?: ImagePreview[];  // T041: Add preview images for folder view
}

export interface ProjectWithRelations extends Project {
  children: Project[];
  stl_count: number;
  image_count: number;
  tags: Tag[];
  inherited_images: ImagePreview[];  // T037: Add inherited images
}

// File types
export interface StlFile {
  id: number;
  project_id: number;
  filename: string;
  file_path: string;
  file_size: number;
  preview_path: string | null;
  preview_generated_at: number | null;
}

export interface ImageFile {
  id: number;
  project_id: number;
  filename: string;
  file_path: string;
  file_size: number;
  source_type: 'direct' | 'inherited';
  source_project_id: number | null;
  display_order: number;
}

// Tag types
export interface Tag {
  id: number;
  name: string;
  color: string | null;
  usage_count: number;
  created_at: number;
}

// Scan types
export interface ScanSession {
  id: number;
  root_path: string;
  status: 'running' | 'completed' | 'failed' | 'cancelled';
  started_at: number;
  completed_at: number | null;
  projects_found: number;
  files_processed: number;
  errors_count: number;
}
