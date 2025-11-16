import React, { useState, useEffect, useRef } from 'react';
import { tagsApi } from '../../api/tags';
import { Tag } from '../../types/project';

interface TagInputProps {
  onTagAdd: (tagName: string, color?: string) => void;
  placeholder?: string;
  disabled?: boolean;
}

export const TagInput: React.FC<TagInputProps> = ({ 
  onTagAdd, 
  placeholder = "Add tag...",
  disabled = false 
}) => {
  const [input, setInput] = useState('');
  const [suggestions, setSuggestions] = useState<Tag[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(-1);
  const inputRef = useRef<HTMLInputElement>(null);
  const suggestionsRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const fetchSuggestions = async () => {
      if (input.length > 0) {
        const results = await tagsApi.autocomplete(input);
        setSuggestions(results);
        setShowSuggestions(true);
        setSelectedIndex(-1);
      } else {
        setSuggestions([]);
        setShowSuggestions(false);
      }
    };

    const timeoutId = setTimeout(fetchSuggestions, 200);
    return () => clearTimeout(timeoutId);
  }, [input]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        inputRef.current &&
        !inputRef.current.contains(event.target as Node) &&
        suggestionsRef.current &&
        !suggestionsRef.current.contains(event.target as Node)
      ) {
        setShowSuggestions(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleSubmit = (tagName: string, color?: string) => {
    if (tagName.trim()) {
      onTagAdd(tagName.trim(), color);
      setInput('');
      setSuggestions([]);
      setShowSuggestions(false);
      setSelectedIndex(-1);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      if (selectedIndex >= 0 && selectedIndex < suggestions.length) {
        const selected = suggestions[selectedIndex];
        handleSubmit(selected.name, selected.color || undefined);
      } else if (input.trim()) {
        handleSubmit(input);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => 
        prev < suggestions.length - 1 ? prev + 1 : prev
      );
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev > 0 ? prev - 1 : -1));
    } else if (e.key === 'Escape') {
      setShowSuggestions(false);
      setSelectedIndex(-1);
    }
  };

  return (
    <div style={{ position: 'relative', display: 'inline-block', width: '100%' }}>
      <input
        ref={inputRef}
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={placeholder}
        disabled={disabled}
        style={{
          width: '100%',
          padding: '8px 12px',
          border: '1px solid #ddd',
          borderRadius: '4px',
          fontSize: '14px',
        }}
      />
      
      {showSuggestions && suggestions.length > 0 && (
        <div
          ref={suggestionsRef}
          style={{
            position: 'absolute',
            top: '100%',
            left: 0,
            right: 0,
            backgroundColor: 'white',
            border: '1px solid #ddd',
            borderTop: 'none',
            borderRadius: '0 0 4px 4px',
            maxHeight: '200px',
            overflowY: 'auto',
            zIndex: 1000,
            boxShadow: '0 2px 4px rgba(0,0,0,0.1)',
          }}
        >
          {suggestions.map((tag, index) => (
            <div
              key={tag.id}
              onClick={() => handleSubmit(tag.name, tag.color || undefined)}
              style={{
                padding: '8px 12px',
                cursor: 'pointer',
                backgroundColor: index === selectedIndex ? '#f0f0f0' : 'white',
                display: 'flex',
                alignItems: 'center',
                gap: '8px',
              }}
              onMouseEnter={() => setSelectedIndex(index)}
            >
              {tag.color && (
                <span
                  style={{
                    width: '16px',
                    height: '16px',
                    backgroundColor: tag.color,
                    borderRadius: '3px',
                    display: 'inline-block',
                  }}
                />
              )}
              <span>{tag.name}</span>
              <span style={{ marginLeft: 'auto', fontSize: '12px', color: '#666' }}>
                {tag.usage_count}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
