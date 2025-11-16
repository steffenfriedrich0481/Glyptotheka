import { apiClient } from './client';
import type { Project } from '../types/project';

export interface SearchParams {
  q?: string;
  tags?: string[];
  page?: number;
  per_page?: number;
}

export interface SearchMeta {
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

export interface SearchResponse {
  data: Project[];
  meta: SearchMeta;
}

export const searchApi = {
  async search(params: SearchParams): Promise<SearchResponse> {
    const queryParams = new URLSearchParams();
    
    if (params.q) {
      queryParams.append('q', params.q);
    }
    
    if (params.tags && params.tags.length > 0) {
      queryParams.append('tags', params.tags.join(','));
    }
    
    if (params.page) {
      queryParams.append('page', params.page.toString());
    }
    
    if (params.per_page) {
      queryParams.append('per_page', params.per_page.toString());
    }
    
    const response = await apiClient.get(`/search?${queryParams.toString()}`);
    return response.data;
  },
};
