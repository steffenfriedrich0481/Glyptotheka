import React, { useState, useEffect } from 'react';
import { tagsApi } from '../../api/tags';
import { Tag } from '../../types/project';
import { TagInput } from '../common/TagInput';

interface TagManagerProps {
  projectId: number;
  initialTags?: Tag[];
  onTagsChange?: (tags: Tag[]) => void;
}

export const TagManager: React.FC<TagManagerProps> = ({ 
  projectId, 
  initialTags = [],
  onTagsChange 
}) => {
  const [tags, setTags] = useState<Tag[]>(initialTags);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setTags(initialTags);
  }, [initialTags]);

  const handleAddTag = async (tagName: string, color?: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await tagsApi.addToProject(projectId, { 
        tagName, 
        color 
      });
      setTags(response.tags);
      if (onTagsChange) {
        onTagsChange(response.tags);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to add tag');
      console.error('Failed to add tag:', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleRemoveTag = async (tagName: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await tagsApi.removeFromProject(projectId, tagName);
      setTags(response.tags);
      if (onTagsChange) {
        onTagsChange(response.tags);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to remove tag');
      console.error('Failed to remove tag:', err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div style={{ marginBottom: '20px' }}>
      <h3 style={{ fontSize: '16px', fontWeight: '600', marginBottom: '12px' }}>
        Tags
      </h3>
      
      {error && (
        <div
          style={{
            padding: '8px 12px',
            backgroundColor: '#fee',
            color: '#c00',
            borderRadius: '4px',
            marginBottom: '12px',
            fontSize: '14px',
          }}
        >
          {error}
        </div>
      )}
      
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', marginBottom: '12px' }}>
        {tags.length === 0 ? (
          <p style={{ fontSize: '14px', color: '#666', fontStyle: 'italic' }}>
            No tags yet. Add some below!
          </p>
        ) : (
          tags.map((tag) => (
            <div
              key={tag.id}
              style={{
                display: 'inline-flex',
                alignItems: 'center',
                gap: '6px',
                padding: '6px 10px',
                backgroundColor: tag.color || '#e0e0e0',
                color: tag.color ? getContrastColor(tag.color) : '#333',
                borderRadius: '16px',
                fontSize: '14px',
                fontWeight: '500',
              }}
            >
              <span>{tag.name}</span>
              <button
                onClick={() => handleRemoveTag(tag.name)}
                disabled={isLoading}
                style={{
                  background: 'none',
                  border: 'none',
                  color: 'inherit',
                  cursor: isLoading ? 'not-allowed' : 'pointer',
                  padding: '0',
                  fontSize: '16px',
                  lineHeight: '1',
                  opacity: isLoading ? 0.5 : 0.7,
                }}
                onMouseEnter={(e) => (e.currentTarget.style.opacity = '1')}
                onMouseLeave={(e) => (e.currentTarget.style.opacity = '0.7')}
                title="Remove tag"
              >
                ×
              </button>
            </div>
          ))
        )}
      </div>
      
      <TagInput 
        onTagAdd={handleAddTag} 
        disabled={isLoading}
        placeholder="Add a tag (press Enter)"
      />
    </div>
  );
};

// Helper function to determine text color based on background color
function getContrastColor(hexColor: string): string {
  // Remove # if present
  const hex = hexColor.replace('#', '');
  
  // Convert to RGB
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);
  
  // Calculate luminance
  const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
  
  // Return black or white based on luminance
  return luminance > 0.5 ? '#000' : '#fff';
}
