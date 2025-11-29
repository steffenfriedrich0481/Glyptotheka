import axios, { CancelTokenSource } from 'axios';

// Use empty baseURL to use relative paths through Vite proxy
// In production, set VITE_API_BASE_URL to the backend URL
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '';

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 30000,
});

// T029: Request cancellation for rapid navigation
let currentFolderRequest: CancelTokenSource | null = null;
let currentBreadcrumbRequest: CancelTokenSource | null = null;

// Request interceptor
apiClient.interceptors.request.use(
  (config) => {
    // Add any auth tokens here if needed in the future
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor
apiClient.interceptors.response.use(
  (response) => {
    return response;
  },
  (error) => {
    // Handle errors globally
    if (error.response) {
      // Server responded with error status
      console.error('API Error:', error.response.data);
    } else if (error.request) {
      // Request made but no response
      console.error('Network Error:', error.message);
    } else {
      // Something else happened
      console.error('Error:', error.message);
    }
    return Promise.reject(error);
  }
);

// T026-T028: Folder navigation API calls
export interface BreadcrumbItem {
  name: string;
  path: string;
}

export interface FolderInfo {
  name: string;
  path: string;
  project_count: number;
  has_images: boolean;
}

export interface ProjectWithPreview {
  project: {
    id: number;
    name: string;
    full_path: string;
    parent_id: number | null;
    is_leaf: boolean;
    description: string | null;
    folder_level: number;
    created_at: string;
    updated_at: string;
  };
  preview_images: Array<{
    id: number;
    filename: string;
    source_type: string;
    image_source: string;
    priority: number;
    inherited_from: string | null;
  }>;
}

export interface FolderContents {
  folders: FolderInfo[];
  projects: ProjectWithPreview[];
  current_path: string;
  total_folders: number;
  total_projects: number;
  is_leaf_project: boolean; // T042: Indicates if current path is a leaf project
}

// T027: Fetch folder contents with cancellation support
export async function fetchFolderContents(
  path: string = '',
  page?: number,
  perPage?: number
): Promise<FolderContents> {
  // Cancel previous request if still pending
  if (currentFolderRequest) {
    currentFolderRequest.cancel('New folder request initiated');
  }

  // Create new cancel token
  currentFolderRequest = axios.CancelToken.source();

  const params = new URLSearchParams();
  if (page !== undefined) params.append('page', page.toString());
  if (perPage !== undefined) params.append('per_page', perPage.toString());

  const url = path ? `/api/browse/${path}` : '/api/browse';
  const queryString = params.toString();
  const fullUrl = queryString ? `${url}?${queryString}` : url;

  const response = await apiClient.get<FolderContents>(fullUrl, {
    cancelToken: currentFolderRequest.token,
  });

  return response.data;
}

// T028: Fetch breadcrumb trail with cancellation support
export async function fetchBreadcrumb(path: string = ''): Promise<BreadcrumbItem[]> {
  // Cancel previous request if still pending
  if (currentBreadcrumbRequest) {
    currentBreadcrumbRequest.cancel('New breadcrumb request initiated');
  }

  // Create new cancel token
  currentBreadcrumbRequest = axios.CancelToken.source();

  const url = path ? `/api/browse/breadcrumb/${path}` : '/api/browse/breadcrumb';

  const response = await apiClient.get<BreadcrumbItem[]>(url, {
    cancelToken: currentBreadcrumbRequest.token,
  });

  return response.data;
}

export default apiClient;
