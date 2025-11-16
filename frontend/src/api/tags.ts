import { apiClient } from './client';

export interface Tag {
  id: number;
  name: string;
  color?: string;
  created_at: number;
  usage_count: number;
}

export interface TagsResponse {
  data: Tag[];
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
};
