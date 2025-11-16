import { apiClient } from './client';
import { Tag } from '../types/project';

export interface TagsResponse {
  data: Tag[];
}

export interface CreateTagRequest {
  name: string;
  color?: string;
}

export interface AddTagToProjectRequest {
  tagName: string;
  color?: string;
}

export const tagsApi = {
  async list(params?: { q?: string; sortBy?: 'name' | 'usage' }): Promise<Tag[]> {
    const queryParams = new URLSearchParams();
    
    if (params?.q) {
      queryParams.append('q', params.q);
    }
    
    if (params?.sortBy) {
      queryParams.append('sortBy', params.sortBy);
    }
    
    const response = await apiClient.get<TagsResponse>(`/api/tags?${queryParams.toString()}`);
    return response.data.data;
  },

  async autocomplete(query: string): Promise<Tag[]> {
    const queryParams = new URLSearchParams({ q: query });
    const response = await apiClient.get<TagsResponse>(`/api/tags/autocomplete?${queryParams.toString()}`);
    return response.data.data;
  },

  async create(tag: CreateTagRequest): Promise<Tag> {
    const response = await apiClient.post<Tag>('/api/tags', tag);
    return response.data;
  },

  async addToProject(projectId: number, request: AddTagToProjectRequest): Promise<{ tags: Tag[] }> {
    const response = await apiClient.post<{ tags: Tag[] }>(`/api/projects/${projectId}/tags`, request);
    return response.data;
  },

  async removeFromProject(projectId: number, tagName: string): Promise<{ tags: Tag[] }> {
    const queryParams = new URLSearchParams({ tagName });
    const response = await apiClient.delete<{ tags: Tag[] }>(`/api/projects/${projectId}/tags?${queryParams.toString()}`);
    return response.data;
  },
};
