import React, { useState, MouseEvent, useEffect, useRef } from 'react';
import { ImagePreview } from '../../types/project';

interface SearchTileCarouselProps {
  images: ImagePreview[];
  projectName: string;
  autoAdvance?: boolean;
}

export const SearchTileCarousel: React.FC<SearchTileCarouselProps> = React.memo(({ images, projectName, autoAdvance = false }) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isHovered, setIsHovered] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [imageError, setImageError] = useState(false);
  const [imageLoaded, setImageLoaded] = useState(false);
  const timerRef = useRef<NodeJS.Timeout | null>(null);
  const pauseTimerRef = useRef<NodeJS.Timeout | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Reset error and loaded state when image changes
  useEffect(() => {
    setImageError(false);
    setImageLoaded(false);
  }, [currentIndex, images]);

  // Auto-advance logic
  useEffect(() => {
    if (!autoAdvance || !images || images.length <= 1 || isHovered || isPaused || imageError) {
      if (timerRef.current) {
        clearInterval(timerRef.current);
        timerRef.current = null;
      }
      return;
    }

    // Fixed interval of 4000ms (within 3-5s range)
    const interval = 4000;

    timerRef.current = setInterval(() => {
      setCurrentIndex((prev) => (prev === images.length - 1 ? 0 : prev + 1));
    }, interval);

    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, [autoAdvance, images, isHovered, isPaused, imageError]);

  // Cleanup pause timer
  useEffect(() => {
    return () => {
      if (pauseTimerRef.current) {
        clearTimeout(pauseTimerRef.current);
      }
    };
  }, []);

  const handleManualNavigation = () => {
    setIsPaused(true);
    if (pauseTimerRef.current) {
      clearTimeout(pauseTimerRef.current);
    }
    pauseTimerRef.current = setTimeout(() => {
      setIsPaused(false);
    }, 10000);
  };

  if (!images || images.length === 0) {
    return (
      <div className="w-full h-full bg-gray-200 flex items-center justify-center" aria-label={`No images for ${projectName}`}>
        <span className="text-gray-400 text-4xl" aria-hidden="true">📦</span>
      </div>
    );
  }

  const handlePrev = (e?: React.MouseEvent | React.KeyboardEvent) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }
    handleManualNavigation();
    setCurrentIndex((prev) => (prev === 0 ? images.length - 1 : prev - 1));
  };

  const handleNext = (e?: React.MouseEvent | React.KeyboardEvent) => {
    if (e) {
      e.preventDefault();
      e.stopPropagation();
    }
    handleManualNavigation();
    setCurrentIndex((prev) => (prev === images.length - 1 ? 0 : prev + 1));
  };

  const handleDotClick = (e: MouseEvent, index: number) => {
    e.preventDefault();
    e.stopPropagation();
    handleManualNavigation();
    setCurrentIndex(index);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (images.length <= 1) return;
    
    if (e.key === 'ArrowLeft') {
      handlePrev(e);
    } else if (e.key === 'ArrowRight') {
      handleNext(e);
    }
  };

  const currentImage = images[currentIndex];

  return (
    <div 
      ref={containerRef}
      className="relative w-full h-full overflow-hidden group bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      onKeyDown={handleKeyDown}
      tabIndex={0}
      role="region"
      aria-label={`Image carousel for ${projectName}`}
      style={{ contain: 'layout style paint' }}
    >
      {!imageLoaded && !imageError && (
        <div className="absolute inset-0 bg-gray-200 animate-pulse" aria-hidden="true" />
      )}
      
      {!imageError ? (
        <img
          src={`/api/files/images/${currentImage.id}`}
          alt={`${projectName} - Image ${currentIndex + 1} of ${images.length}`}
          className={`w-full h-full object-cover transition-opacity duration-300 ${imageLoaded ? 'opacity-100' : 'opacity-0'}`}
          loading="lazy"
          onLoad={() => setImageLoaded(true)}
          onError={() => setImageError(true)}
        />
      ) : (
        <div className="w-full h-full bg-gray-200 flex items-center justify-center text-gray-400">
          <span className="text-sm">Image unavailable</span>
        </div>
      )}
      
      {/* Badges */}
      <div className="absolute top-2 right-2 flex gap-1">
        {currentImage.image_source === 'stl_preview' && (
          <span className="bg-blue-500 text-white text-xs px-2 py-1 rounded shadow">STL</span>
        )}
        {currentImage.source_type === 'inherited' && (
          <span className="bg-purple-500 text-white text-xs px-2 py-1 rounded shadow">Inherited</span>
        )}
      </div>

      {/* Navigation Controls */}
      {images.length > 1 && (
        <>
          <button
            onClick={handlePrev}
            className={`absolute left-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white p-1 rounded-full transition-opacity ${isHovered ? 'opacity-100' : 'opacity-0'} focus:opacity-100`}
            aria-label="Previous image"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d="m15 18-6-6 6-6"/></svg>
          </button>
          
          <button
            onClick={handleNext}
            className={`absolute right-2 top-1/2 -translate-y-1/2 bg-black/50 hover:bg-black/70 text-white p-1 rounded-full transition-opacity ${isHovered ? 'opacity-100' : 'opacity-0'} focus:opacity-100`}
            aria-label="Next image"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true"><path d="m9 18 6-6-6-6"/></svg>
          </button>

          {/* Dots */}
          <div className="absolute bottom-2 left-1/2 -translate-x-1/2 flex gap-1" role="tablist">
            {images.map((_, idx) => (
              <button
                key={idx}
                onClick={(e) => handleDotClick(e, idx)}
                className={`w-2 h-2 rounded-full transition-colors ${idx === currentIndex ? 'bg-white' : 'bg-white/50 hover:bg-white/80'}`}
                aria-label={`Go to image ${idx + 1}`}
                aria-selected={idx === currentIndex}
                role="tab"
              />
            ))}
          </div>
        </>
      )}
    </div>
  );
});
