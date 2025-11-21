import React, { useState, useEffect } from 'react';
import { useSearchParams, useNavigate } from 'react-router-dom';
import { searchApi } from '../api/search';
import { tagsApi } from '../api/tags';
import { SearchBar } from '../components/common/SearchBar';
import { LoadingSpinner } from '../components/common/LoadingSpinner';
import { Pagination } from '../components/common/Pagination';
import type { SearchResultProject, Tag } from '../types/project';
import './SearchPage.css';

export const SearchPage: React.FC = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const navigate = useNavigate();
  
  const [projects, setProjects] = useState<SearchResultProject[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [totalPages, setTotalPages] = useState(0);
  const [total, setTotal] = useState(0);
  
  const [availableTags, setAvailableTags] = useState<Tag[]>([]);
  const [selectedTags, setSelectedTags] = useState<string[]>([]);
  
  const query = searchParams.get('q') || '';
  const page = parseInt(searchParams.get('page') || '1', 10);
  const tags = searchParams.get('tags')?.split(',').filter(Boolean) || [];

  // Load available tags
  useEffect(() => {
    tagsApi.list({ sortBy: 'usage' })
      .then(setAvailableTags)
      .catch(console.error);
  }, []);

  // Initialize selected tags from URL
  useEffect(() => {
    setSelectedTags(tags);
  }, []);

  // Search when params change
  useEffect(() => {
    const performSearch = async () => {
      setLoading(true);
      setError(null);
      
      try {
        const result = await searchApi.search({
          q: query || undefined,
          tags: selectedTags.length > 0 ? selectedTags : undefined,
          page,
          per_page: 20,
          leaf_only: true,
        });
        
        setProjects(result.data);
        setTotalPages(result.meta.total_pages);
        setTotal(result.meta.total);
      } catch (err) {
        setError('Failed to search projects');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    performSearch();
  }, [query, page, selectedTags]);

  const handlePageChange = (newPage: number) => {
    const params = new URLSearchParams(searchParams);
    params.set('page', newPage.toString());
    setSearchParams(params);
  };

  const handleTagToggle = (tagName: string) => {
    const newTags = selectedTags.includes(tagName)
      ? selectedTags.filter(t => t !== tagName)
      : [...selectedTags, tagName];
    
    setSelectedTags(newTags);
    
    const params = new URLSearchParams(searchParams);
    if (newTags.length > 0) {
      params.set('tags', newTags.join(','));
    } else {
      params.delete('tags');
    }
    params.delete('page'); // Reset to page 1
    setSearchParams(params);
  };

  const handleProjectClick = (projectId: number) => {
    navigate(`/projects/${projectId}`);
  };

  return (
    <div className="search-page">
      <div className="search-page__header">
        <h1>Search Projects</h1>
        <SearchBar initialQuery={query} />
      </div>

      {availableTags.length > 0 && (
        <div className="search-page__filters">
          <h3>Filter by Tags:</h3>
          <div className="search-page__tags">
            {availableTags.map(tag => (
              <button
                key={tag.id}
                className={`search-page__tag ${
                  selectedTags.includes(tag.name) ? 'search-page__tag--active' : ''
                }`}
                onClick={() => handleTagToggle(tag.name)}
                style={{
                  backgroundColor: selectedTags.includes(tag.name)
                    ? tag.color || '#4a90e2'
                    : '#f0f0f0',
                  color: selectedTags.includes(tag.name) ? 'white' : '#333',
                }}
              >
                {tag.name} ({tag.usage_count})
              </button>
            ))}
          </div>
        </div>
      )}

      <div className="search-page__content">
        {loading && <LoadingSpinner />}
        
        {error && (
          <div className="search-page__error">
            <p>{error}</p>
          </div>
        )}
        
        {!loading && !error && projects.length === 0 && (
          <div className="search-page__empty">
            <p>No projects found.</p>
            {(query || selectedTags.length > 0) && (
              <p>Try adjusting your search or filters.</p>
            )}
          </div>
        )}
        
        {!loading && !error && projects.length > 0 && (
          <>
            <div className="search-page__results-info">
              Found {total} project{total !== 1 ? 's' : ''}
            </div>
            <div className="search-page__grid">
              {projects.map(project => (
                <div
                  key={project.id}
                  className="search-page__project"
                  onClick={() => handleProjectClick(project.id)}
                >
                  <h3>{project.name}</h3>
                  <p className="search-page__project-path">{project.full_path}</p>
                </div>
              ))}
            </div>
            
            {totalPages > 1 && (
              <Pagination
                currentPage={page}
                totalPages={totalPages}
                onPageChange={handlePageChange}
              />
            )}
          </>
        )}
      </div>
    </div>
  );
};
