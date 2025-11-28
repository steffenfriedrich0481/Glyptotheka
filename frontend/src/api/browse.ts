import { apiClient } from './client';

export interface FolderInfo {
  name: string;
  path: string;
  project_count: number;
  has_images: boolean;
}

export interface ProjectWithPreview {
  id: number;
  name: string;
  path: string;
  is_leaf: boolean;
  has_stl_files: boolean;
  preview_images: Array<{
    path: string;
    thumbnail_path?: string;
  }>;
}

export interface FolderContents {
  folders: FolderInfo[];
  projects: ProjectWithPreview[];
  current_path: string;
  total_folders: number;
  total_projects: number;
}

export interface BreadcrumbItem {
  name: string;
  path: string;
}

export const browseAPI = {
  /**
   * Fetch folder contents at a given path
   */
  async getFolderContents(
    path: string = '',
    page?: number,
    perPage?: number
  ): Promise<FolderContents> {
    const params = new URLSearchParams();
    if (page) params.append('page', page.toString());
    if (perPage) params.append('per_page', perPage.toString());

    const queryString = params.toString();
    const url = path
      ? `/api/browse/${path}${queryString ? `?${queryString}` : ''}`
      : `/api/browse${queryString ? `?${queryString}` : ''}`;

    const response = await apiClient.get<FolderContents>(url);
    return response.data;
  },

  /**
   * Fetch breadcrumb trail for a given path
   */
  async getBreadcrumb(path: string = ''): Promise<BreadcrumbItem[]> {
    const url = path
      ? `/api/browse/breadcrumb/${path}`
      : `/api/browse/breadcrumb`;

    const response = await apiClient.get<BreadcrumbItem[]>(url);
    return response.data;
  },
};
