import React from 'react';
import './Breadcrumb.css';

interface BreadcrumbItem {
  id: number;
  name: string;
  path: string;
}

interface BreadcrumbProps {
  items: BreadcrumbItem[];
  onNavigate: (item: BreadcrumbItem, index: number) => void;
}

const Breadcrumb: React.FC<BreadcrumbProps> = ({ items, onNavigate }) => {
  if (items.length === 0) {
    return null;
  }

  const handleKeyDown = (e: React.KeyboardEvent, item: BreadcrumbItem, index: number) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      onNavigate(item, index);
    }
  };

  return (
    <nav className="breadcrumb" aria-label="Breadcrumb">
      <ol className="breadcrumb-list">
        <li className="breadcrumb-item">
          <button 
            className="breadcrumb-link"
            onClick={() => onNavigate({ id: 0, name: 'Home', path: '/' }, -1)}
            onKeyDown={(e) => handleKeyDown(e, { id: 0, name: 'Home', path: '/' }, -1)}
            aria-label="Navigate to Home"
          >
            🏠 Home
          </button>
        </li>
        {items.map((item, index) => (
          <li key={item.id} className="breadcrumb-item">
            <span className="breadcrumb-separator" aria-hidden="true">/</span>
            {index === items.length - 1 ? (
              <span className="breadcrumb-current" aria-current="page">{item.name}</span>
            ) : (
              <button 
                className="breadcrumb-link"
                onClick={() => onNavigate(item, index)}
                onKeyDown={(e) => handleKeyDown(e, item, index)}
                aria-label={`Navigate to ${item.name}`}
              >
                {item.name}
              </button>
            )}
          </li>
        ))}
      </ol>
    </nav>
  );
};

export default Breadcrumb;
