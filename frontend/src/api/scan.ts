import axios from './client';

export interface ScanStatus {
  is_scanning: boolean;
  projects_found?: number;
  files_processed?: number;
  errors?: string[];
}

export const scanAPI = {
  startScan: async (): Promise<ScanStatus> => {
    const response = await axios.post('/api/scan');
    return response.data;
  },

  getScanStatus: async (): Promise<ScanStatus> => {
    const response = await axios.get('/api/scan/status');
    return response.data;
  },
};
