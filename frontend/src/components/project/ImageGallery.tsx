import React from 'react';
import { ImageFile } from '../../types/project';
import Pagination from '../common/Pagination';

interface Props {
  images: ImageFile[];
  total: number;
  page: number;
  perPage: number;
  onPageChange: (page: number) => void;
}

const ImageGallery: React.FC<Props> = ({ images, total, page, perPage, onPageChange }) => {
  const totalPages = Math.ceil(total / perPage);

  return (
    <div>
      <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
        {images.map((image) => (
          <div key={image.id} className="border rounded overflow-hidden">
            <div className="aspect-square bg-gray-200 flex items-center justify-center">
              <span className="text-gray-400">🖼️</span>
            </div>
            <div className="p-2">
              <p className="text-xs truncate">{image.filename}</p>
            </div>
          </div>
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
