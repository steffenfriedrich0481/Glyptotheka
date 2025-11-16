import { useState } from 'react';
import { tagsApi } from '../api/tags';
import { Tag } from '../types/project';

export const useTags = () => {
  const [tags, setTags] = useState<Tag[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTags = async (params?: { q?: string; sortBy?: 'name' | 'usage' }) => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await tagsApi.list(params);
      setTags(data);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to load tags';
      setError(message);
      console.error('Failed to load tags:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const searchTags = async (query: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await tagsApi.autocomplete(query);
      setTags(data);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to search tags';
      setError(message);
      console.error('Failed to search tags:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const createTag = async (name: string, color?: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const newTag = await tagsApi.create({ name, color });
      setTags((prev) => [...prev, newTag]);
      return newTag;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to create tag';
      setError(message);
      console.error('Failed to create tag:', err);
      return null;
    } finally {
      setIsLoading(false);
    }
  };

  const addTagToProject = async (projectId: number, tagName: string, color?: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await tagsApi.addToProject(projectId, { tagName, color });
      return response.tags;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to add tag to project';
      setError(message);
      console.error('Failed to add tag to project:', err);
      return null;
    } finally {
      setIsLoading(false);
    }
  };

  const removeTagFromProject = async (projectId: number, tagName: string) => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await tagsApi.removeFromProject(projectId, tagName);
      return response.tags;
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to remove tag from project';
      setError(message);
      console.error('Failed to remove tag from project:', err);
      return null;
    } finally {
      setIsLoading(false);
    }
  };

  return {
    tags,
    isLoading,
    error,
    loadTags,
    searchTags,
    createTag,
    addTagToProject,
    removeTagFromProject,
  };
};
