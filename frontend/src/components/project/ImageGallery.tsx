import React, { useState, useRef, useEffect } from 'react';
import { ImageFile } from '../../types/project';
import Pagination from '../common/Pagination';

interface Props {
  images: ImageFile[];
  total: number;
  page: number;
  perPage: number;
  onPageChange: (page: number) => void;
}

const LazyImage: React.FC<{ image: ImageFile }> = ({ image }) => {
  const [isLoaded, setIsLoaded] = useState(false);
  const [isInView, setIsInView] = useState(false);
  const imgRef = useRef<HTMLImageElement>(null);

  useEffect(() => {
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setIsInView(true);
          observer.disconnect();
        }
      },
      { rootMargin: '50px' }
    );

    if (imgRef.current) {
      observer.observe(imgRef.current);
    }

    return () => observer.disconnect();
  }, []);

  const imageUrl = `/api/files/images/${image.id}`;

  return (
    <div className="border rounded overflow-hidden">
      <div className="aspect-square bg-gray-200 flex items-center justify-center relative">
        {isInView ? (
          <>
            {!isLoaded && (
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-500"></div>
              </div>
            )}
            <img
              ref={imgRef}
              src={imageUrl}
              alt={image.filename}
              className={`w-full h-full object-cover transition-opacity duration-300 ${
                isLoaded ? 'opacity-100' : 'opacity-0'
              }`}
              onLoad={() => setIsLoaded(true)}
              onError={(e) => {
                setIsLoaded(true);
                (e.target as HTMLImageElement).src = 'data:image/svg+xml,%3Csvg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"%3E%3Ctext y="50" font-size="16" text-anchor="middle" x="50"%3E❌%3C/text%3E%3C/svg%3E';
              }}
              loading="lazy"
            />
          </>
        ) : (
          <span className="text-theme-muted">🖼️</span>
        )}
      </div>
      <div className="p-2">
        <p className="text-xs truncate" title={image.filename}>
          {image.filename}
        </p>
      </div>
    </div>
  );
};

const ImageGallery: React.FC<Props> = ({ images, total, page, perPage, onPageChange }) => {
  const totalPages = Math.ceil(total / perPage);

  return (
    <div>
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        {images.map((image) => (
          <LazyImage key={image.id} image={image} />
        ))}
      </div>
      {totalPages > 1 && (
        <Pagination
          currentPage={page}
          totalPages={totalPages}
          onPageChange={onPageChange}
        />
      )}
    </div>
  );
};

export default ImageGallery;
