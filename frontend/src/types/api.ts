// API Response types
export interface ApiResponse<T> {
  data: T;
}

export interface PaginatedResponse<T> {
  data: T[];
  meta: PaginationMeta;
}

export interface PaginationMeta {
  page: number;
  perPage: number;
  total: number;
  totalPages: number;
}

export interface ErrorResponse {
  error: string;
  message: string;
}

// Query parameters
export interface PaginationParams {
  page?: number;
  perPage?: number;
}

export interface SearchParams extends PaginationParams {
  q?: string;
  tags?: string[];
}
