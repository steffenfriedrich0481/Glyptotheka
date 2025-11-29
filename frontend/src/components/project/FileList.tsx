import React, { useState } from 'react';
import { StlFile, StlCategory, ImageFile } from '../../types/project';
import { downloadAPI } from '../../api/download';
import { downloadUtils } from '../../utils/download';
import { projectsAPI } from '../../api/projects';

interface Props {
  categories: StlCategory[];
  images?: ImageFile[];
}

const FileList: React.FC<Props> = ({ categories, images = [] }) => {
  const [downloading, setDownloading] = useState<number | null>(null);
  const [failedPreviews, setFailedPreviews] = useState<Set<number>>(new Set());
  const [previewModal, setPreviewModal] = useState<{ file: StlFile; previewHash: string } | null>(null);

  const handleDownloadFile = async (fileId: number, filename: string, fileType: 'stl' | 'image') => {
    try {
      setDownloading(fileId);
      const blob = await downloadAPI.downloadFile({ fileId, fileType });
      downloadUtils.triggerDownload(blob, filename);
    } catch (error) {
      console.error('Download failed:', error);
      alert('Failed to download file. Please try again.');
    } finally {
      setDownloading(null);
    }
  };

  const handlePreviewError = (fileId: number) => {
    setFailedPreviews(prev => new Set(prev).add(fileId));
  };

  const getPreviewHash = (previewPath: string | null): string | null => {
    if (!previewPath) return null;
    // Extract hash from cache path like: cache/previews/{hash}.png
    const match = previewPath.match(/([a-f0-9]{64})/);
    return match ? match[1] : null;
  };

  const renderStlFile = (file: StlFile) => {
    const previewHash = getPreviewHash(file.preview_path);
    const showPreview = previewHash && !failedPreviews.has(file.id);
    
    return (
      <li key={file.id} className="flex justify-between items-center p-2 hover:bg-gray-50 dark:hover:bg-gray-700">
        <div className="flex items-center gap-3 flex-1">
          {showPreview ? (
            <img 
              src={projectsAPI.getPreviewUrl(previewHash)}
              alt={file.filename}
              className="w-16 h-16 object-cover rounded cursor-pointer hover:opacity-80 transition-opacity"
              onError={() => handlePreviewError(file.id)}
              onClick={() => setPreviewModal({ file, previewHash })}
            />
          ) : (
            <div className="w-16 h-16 bg-gray-200 dark:bg-gray-700 rounded flex items-center justify-center">
              <span className="text-2xl">🧊</span>
            </div>
          )}
          <span className="truncate text-gray-900 dark:text-gray-100">{file.filename}</span>
        </div>
        <div className="flex items-center gap-4">
          <span className="text-sm text-gray-500 dark:text-gray-400">
            {(file.file_size / 1024 / 1024).toFixed(2)} MB
          </span>
          <button
            onClick={() => handleDownloadFile(file.id, file.filename, 'stl')}
            disabled={downloading === file.id}
            className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400 disabled:cursor-not-allowed text-sm"
          >
            {downloading === file.id ? 'Downloading...' : 'Download'}
          </button>
        </div>
      </li>
    );
  };

  return (
    <>
      <div className="bg-white dark:bg-gray-800 shadow rounded-lg p-4">
        {categories.map((category, idx) => (
          <div key={idx} className="mb-6 last:mb-0">
            <h3 className="font-bold text-lg mb-4 text-gray-900 dark:text-gray-100">
              {category.category ? `STL Files - ${category.category}` : 'STL Files'}
            </h3>
            <ul className="space-y-2">
              {category.files.map(renderStlFile)}
            </ul>
          </div>
        ))}

        {images.length > 0 && (
          <>
            <h3 className="font-bold text-lg mb-4 mt-6 text-gray-900 dark:text-gray-100">Image Files</h3>
            <ul className="space-y-2">
              {images.map((image) => (
                <li key={image.id} className="flex justify-between items-center p-2 hover:bg-gray-50 dark:hover:bg-gray-700">
                  <span className="truncate text-gray-900 dark:text-gray-100">{image.filename}</span>
                  <div className="flex items-center gap-4">
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      {(image.file_size / 1024 / 1024).toFixed(2)} MB
                    </span>
                    <button
                      onClick={() => handleDownloadFile(image.id, image.filename, 'image')}
                      disabled={downloading === image.id}
                      className="px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400 disabled:cursor-not-allowed text-sm"
                    >
                      {downloading === image.id ? 'Downloading...' : 'Download'}
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          </>
        )}
      </div>

      {/* STL Preview Modal */}
      {previewModal && (
        <div 
          className="fixed inset-0 bg-black bg-opacity-75 flex items-center justify-center z-50 p-4"
          onClick={() => setPreviewModal(null)}
        >
          <div 
            className="relative max-w-5xl max-h-[90vh] bg-white dark:bg-gray-800 rounded-lg shadow-2xl overflow-hidden"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Close button */}
            <button
              onClick={() => setPreviewModal(null)}
              className="absolute top-4 right-4 z-10 bg-gray-900 bg-opacity-50 hover:bg-opacity-75 text-white rounded-full w-10 h-10 flex items-center justify-center text-2xl transition-all"
            >
              ×
            </button>

            {/* Preview Image */}
            <div className="flex flex-col items-center p-6">
              <img 
                src={projectsAPI.getPreviewUrl(previewModal.previewHash)}
                alt={previewModal.file.filename}
                className="max-w-full max-h-[75vh] object-contain rounded"
              />
              <div className="mt-4 text-center">
                <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-2">
                  {previewModal.file.filename}
                </h3>
                <div className="flex items-center justify-center gap-4">
                  <span className="text-sm text-gray-500 dark:text-gray-400">
                    {(previewModal.file.file_size / 1024 / 1024).toFixed(2)} MB
                  </span>
                  <button
                    onClick={() => handleDownloadFile(previewModal.file.id, previewModal.file.filename, 'stl')}
                    disabled={downloading === previewModal.file.id}
                    className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400 disabled:cursor-not-allowed"
                  >
                    {downloading === previewModal.file.id ? 'Downloading...' : 'Download STL'}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default FileList;
