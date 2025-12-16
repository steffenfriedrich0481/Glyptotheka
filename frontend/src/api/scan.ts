import axios from './client';

export interface ScanStatus {
  is_scanning: boolean;
  projects_found?: number;
  projects_added?: number;
  projects_updated?: number;
  projects_removed?: number;
  files_processed?: number;
  files_added?: number;
  files_updated?: number;
  files_removed?: number;
  errors?: string[];
}

export interface StartScanRequest {
  force?: boolean;
  clean?: boolean;
}

export const scanAPI = {
  startScan: async (force?: boolean, clean?: boolean): Promise<ScanStatus> => {
    const response = await axios.post('/api/scan', { force, clean });
    return response.data;
  },

  getScanStatus: async (): Promise<ScanStatus> => {
    const response = await axios.get('/api/scan/status');
    return response.data;
  },
};
