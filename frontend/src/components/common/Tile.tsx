import React from 'react';
import './Tile.css';

interface TileProps {
  title: string;
  imageSrc?: string | null;
  subtitle?: string;
  onClick?: () => void;
  isFolder?: boolean;
  className?: string;
}

const Tile: React.FC<TileProps> = ({ 
  title, 
  imageSrc, 
  subtitle, 
  onClick, 
  isFolder = false,
  className = '',
}) => {
  return (
    <div 
      className={`tile ${className} ${onClick ? 'clickable' : ''}`}
      onClick={onClick}
      role={onClick ? 'button' : undefined}
      tabIndex={onClick ? 0 : undefined}
      onKeyDown={(e) => {
        if (onClick && (e.key === 'Enter' || e.key === ' ')) {
          e.preventDefault();
          onClick();
        }
      }}
    >
      <div className="tile-image">
        {imageSrc ? (
          <img src={imageSrc} alt={title} loading="lazy" />
        ) : (
          <div className="tile-placeholder">
            {isFolder ? '📁' : '📦'}
          </div>
        )}
      </div>
      <div className="tile-content">
        <h3 className="tile-title">{title}</h3>
        {subtitle && <p className="tile-subtitle">{subtitle}</p>}
      </div>
    </div>
  );
};

export default Tile;
