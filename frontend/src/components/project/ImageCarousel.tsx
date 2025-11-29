import React, { useState } from 'react';
import { ImagePreview } from '../../types/project';

interface Props {
  images: ImagePreview[];
}

const ImageCarousel: React.FC<Props> = ({ images }) => {
  const [currentIndex, setCurrentIndex] = useState(0);

  if (images.length === 0) return null;

  const goToPrevious = () => {
    setCurrentIndex((prevIndex) => 
      prevIndex === 0 ? images.length - 1 : prevIndex - 1
    );
  };

  const goToNext = () => {
    setCurrentIndex((prevIndex) => 
      prevIndex === images.length - 1 ? 0 : prevIndex + 1
    );
  };

  const goToSlide = (index: number) => {
    setCurrentIndex(index);
  };

  const currentImage = images[currentIndex];
  const imageUrl = `/api/files/images/${currentImage.id}`;

  return (
    <div className="relative w-full bg-gray-900 rounded-lg overflow-hidden shadow-xl mb-8">
      {/* Main image display */}
      <div className="relative aspect-video bg-gray-800">
        <img
          src={imageUrl}
          alt={currentImage.filename}
          className="w-full h-full object-contain"
          loading="eager"
        />
        
        {/* Image counter */}
        <div className="absolute top-4 right-4 bg-black bg-opacity-60 text-white px-3 py-1 rounded text-sm">
          {currentIndex + 1} / {images.length}
        </div>

        {/* Source indicator for inherited images */}
        {currentImage.source_type === 'inherited' && (
          <div className="absolute top-4 left-4 bg-blue-600 bg-opacity-90 text-white px-3 py-1 rounded text-sm flex items-center gap-2">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" />
            </svg>
            Inherited
          </div>
        )}
      </div>

      {/* Navigation arrows (only show if more than 1 image) */}
      {images.length > 1 && (
        <>
          <button
            onClick={goToPrevious}
            className="absolute left-4 top-1/2 -translate-y-1/2 bg-black bg-opacity-50 hover:bg-opacity-75 text-white p-3 rounded-full transition-all"
            aria-label="Previous image"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>

          <button
            onClick={goToNext}
            className="absolute right-4 top-1/2 -translate-y-1/2 bg-black bg-opacity-50 hover:bg-opacity-75 text-white p-3 rounded-full transition-all"
            aria-label="Next image"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </>
      )}

      {/* Thumbnail navigation (only show if more than 1 image) */}
      {images.length > 1 && (
        <div className="bg-gray-800 px-4 py-3">
          <div className="flex gap-2 overflow-x-auto">
            {images.map((image, index) => (
              <button
                key={image.id}
                onClick={() => goToSlide(index)}
                className={`flex-shrink-0 w-20 h-20 rounded overflow-hidden border-2 transition-all ${
                  index === currentIndex 
                    ? 'border-blue-500 scale-105' 
                    : 'border-gray-600 opacity-60 hover:opacity-100 hover:border-gray-400'
                }`}
              >
                <img
                  src={`/api/files/images/${image.id}`}
                  alt={`Thumbnail ${index + 1}`}
                  className="w-full h-full object-cover"
                  loading="lazy"
                />
              </button>
            ))}
          </div>
        </div>
      )}

      {/* Image info bar */}
      <div className="bg-gray-800 px-4 py-2 text-white text-sm">
        <p className="truncate" title={currentImage.filename}>
          📁 {currentImage.filename}
        </p>
      </div>
    </div>
  );
};

export default ImageCarousel;
