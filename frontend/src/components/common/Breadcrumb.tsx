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

  return (
    <nav className="breadcrumb" aria-label="Breadcrumb">
      <ol className="breadcrumb-list">
        <li className="breadcrumb-item">
          <button 
            className="breadcrumb-link"
            onClick={() => onNavigate({ id: 0, name: 'Home', path: '/' }, -1)}
          >
            🏠 Home
          </button>
        </li>
        {items.map((item, index) => (
          <li key={item.id} className="breadcrumb-item">
            <span className="breadcrumb-separator">/</span>
            {index === items.length - 1 ? (
              <span className="breadcrumb-current">{item.name}</span>
            ) : (
              <button 
                className="breadcrumb-link"
                onClick={() => onNavigate(item, index)}
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
