import axios from './client';
import { Project, ProjectWithRelations, StlFile, ImageFile } from '../types/project';

export interface ProjectListResponse {
  projects: Project[];
}

export interface FilesResponse {
  stl_files: StlFile[];
  images: ImageFile[];
  total_images: number;
  page: number;
  per_page: number;
}

export const projectsAPI = {
  listRoot: async (): Promise<Project[]> => {
    const response = await axios.get<ProjectListResponse>('/api/projects');
    return response.data.projects;
  },

  getProject: async (id: number): Promise<ProjectWithRelations> => {
    const response = await axios.get<ProjectWithRelations>(`/api/projects/${id}`);
    return response.data;
  },

  getProjectChildren: async (id: number): Promise<Project[]> => {
    const response = await axios.get<ProjectListResponse>(`/api/projects/${id}/children`);
    return response.data.projects;
  },

  getProjectFiles: async (
    id: number,
    page: number = 1,
    perPage: number = 20
  ): Promise<FilesResponse> => {
    const response = await axios.get<FilesResponse>(
      `/api/projects/${id}/files`,
      { params: { page, per_page: perPage } }
    );
    return response.data;
  },

  getImageUrl: (hash: string): string => {
    return `${axios.defaults.baseURL}/api/images/${hash}`;
  },
};
