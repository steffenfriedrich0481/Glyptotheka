import React, { useState } from 'react';
import { StlFile, ImageFile } from '../../types/project';
import { downloadAPI } from '../../api/download';
import { downloadUtils } from '../../utils/download';

interface Props {
  files: StlFile[];
  images?: ImageFile[];
}

const FileList: React.FC<Props> = ({ files, images = [] }) => {
  const [downloading, setDownloading] = useState<number | null>(null);

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

  return (
    <div className="bg-white shadow rounded-lg p-4">
      <h3 className="font-bold text-lg mb-4">STL Files</h3>
      <ul className="space-y-2">
        {files.map((file) => (
          <li key={file.id} className="flex justify-between items-center p-2 hover:bg-gray-50">
            <span className="truncate">{file.filename}</span>
            <div className="flex items-center gap-4">
              <span className="text-sm text-gray-500">
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
        ))}
      </ul>

      {images.length > 0 && (
        <>
          <h3 className="font-bold text-lg mb-4 mt-6">Image Files</h3>
          <ul className="space-y-2">
            {images.map((image) => (
              <li key={image.id} className="flex justify-between items-center p-2 hover:bg-gray-50">
                <span className="truncate">{image.filename}</span>
                <div className="flex items-center gap-4">
                  <span className="text-sm text-gray-500">
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
  );
};

export default FileList;
