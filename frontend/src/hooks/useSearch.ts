import { useState, useEffect, useCallback } from 'react';
import { searchApi, type SearchParams } from '../api/search';
import type { Project } from '../types/project';

interface UseSearchResult {
  projects: Project[];
  loading: boolean;
  error: string | null;
  total: number;
  totalPages: number;
  search: (params: SearchParams) => Promise<void>;
  refresh: () => Promise<void>;
}

export const useSearch = (initialParams?: SearchParams): UseSearchResult => {
  const [projects, setProjects] = useState<Project[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [total, setTotal] = useState(0);
  const [totalPages, setTotalPages] = useState(0);
  const [currentParams, setCurrentParams] = useState<SearchParams>(initialParams || {});

  const search = useCallback(async (params: SearchParams) => {
    setLoading(true);
    setError(null);
    setCurrentParams(params);

    try {
      const result = await searchApi.search(params);
      setProjects(result.data);
      setTotal(result.meta.total);
      setTotalPages(result.meta.total_pages);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to search projects');
      setProjects([]);
      setTotal(0);
      setTotalPages(0);
    } finally {
      setLoading(false);
    }
  }, []);

  const refresh = useCallback(async () => {
    await search(currentParams);
  }, [search, currentParams]);

  useEffect(() => {
    if (initialParams) {
      search(initialParams);
    }
  }, []);

  return {
    projects,
    loading,
    error,
    total,
    totalPages,
    search,
    refresh,
  };
};
