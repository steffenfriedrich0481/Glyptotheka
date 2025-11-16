import axios from './client';

export interface AppConfig {
  id: number;
  root_path: string | null;
  last_scan_at: number | null;
  stl_thumb_path: string | null;
  cache_max_size_mb: number;
  images_per_page: number;
  created_at: number;
  updated_at: number;
}

export interface UpdateConfigRequest {
  root_path?: string;
  stl_thumb_path?: string;
  cache_max_size_mb?: number;
  images_per_page?: number;
}

export const configAPI = {
  getConfig: async (): Promise<AppConfig> => {
    const response = await axios.get('/api/config');
    return response.data;
  },

  updateConfig: async (config: UpdateConfigRequest): Promise<AppConfig> => {
    const response = await axios.post('/api/config', config);
    return response.data;
  },
};
