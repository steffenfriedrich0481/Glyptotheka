import { apiClient } from './client';

export interface DownloadFileParams {
  fileId: number;
  fileType: 'stl' | 'image';
}

export const downloadAPI = {
  async downloadFile(params: DownloadFileParams): Promise<Blob> {
    const response = await apiClient.get(`/api/files/${params.fileId}`, {
      params: { type: params.fileType },
      responseType: 'blob',
    });
    return response.data;
  },

  async downloadProjectZip(projectId: number): Promise<Blob> {
    const response = await apiClient.get(`/api/projects/${projectId}/download`, {
      responseType: 'blob',
    });
    return response.data;
  },
};
