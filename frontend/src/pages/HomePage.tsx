import React, { useState, useEffect } from 'react';
import { configAPI, UpdateConfigRequest } from '../api/config';
import { scanAPI } from '../api/scan';
import LoadingSpinner from '../components/common/LoadingSpinner';
import ScanProgress from '../components/common/ScanProgress';
import ConfirmDialog from '../components/common/ConfirmDialog';

const HomePage: React.FC = () => {
  const [rootPath, setRootPath] = useState('');
  const [loading, setLoading] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [scanStatus, setScanStatus] = useState<any>(null);
  const [showRescanConfirm, setShowRescanConfirm] = useState(false);
  const [hasBeenScanned, setHasBeenScanned] = useState(false);

  useEffect(() => {
    loadConfig();
  }, []);

  useEffect(() => {
    if (scanning) {
      const interval = setInterval(async () => {
        const status = await scanAPI.getScanStatus();
        setScanStatus(status);
        if (!status.is_scanning) {
          setScanning(false);
          clearInterval(interval);
        }
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [scanning]);

  const loadConfig = async () => {
    try {
      const config = await configAPI.getConfig();
      if (config.root_path) {
        setRootPath(config.root_path);
      }
      if (config.last_scan_at) {
        setHasBeenScanned(true);
      }
    } catch (err) {
      console.error('Failed to load config:', err);
    }
  };

  const handleSaveConfig = async () => {
    setLoading(true);
    setError(null);
    try {
      const update: UpdateConfigRequest = { root_path: rootPath };
      await configAPI.updateConfig(update);
      alert('Configuration saved successfully!');
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to save configuration');
    } finally {
      setLoading(false);
    }
  };

  const handleStartScan = async (force: boolean = false) => {
    setLoading(true);
    setError(null);
    setScanStatus(null);
    try {
      await scanAPI.startScan(force);
      setScanning(true);
      setHasBeenScanned(true);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to start scan');
    } finally {
      setLoading(false);
    }
  };

  const handleRescanClick = () => {
    setShowRescanConfirm(true);
  };

  const handleConfirmRescan = () => {
    setShowRescanConfirm(false);
    handleStartScan(false); // Incremental rescan
  };

  return (
    <div className="container mx-auto p-8">
      <h1 className="text-3xl font-bold mb-8">3D Print Model Library</h1>

      <div className="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
        <div className="mb-4">
          <label className="block text-gray-700 text-sm font-bold mb-2">
            Root Folder Path
          </label>
          <input
            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            placeholder="/path/to/your/3d-models"
            value={rootPath}
            onChange={(e) => setRootPath(e.target.value)}
          />
        </div>

        {error && (
          <div className="mb-4 text-red-500 text-sm">{error}</div>
        )}

        <div className="flex items-center justify-between">
          <button
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline mr-2"
            onClick={handleSaveConfig}
            disabled={loading || !rootPath}
          >
            Save Configuration
          </button>
          
          <div className="space-x-2">
            <button
              className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              onClick={() => handleStartScan(false)}
              disabled={loading || scanning || !rootPath}
            >
              {scanning ? 'Scanning...' : hasBeenScanned ? 'Scan' : 'Initial Scan'}
            </button>
            
            {hasBeenScanned && (
              <button
                className="bg-orange-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                onClick={handleRescanClick}
                disabled={loading || scanning || !rootPath}
              >
                Rescan Library
              </button>
            )}
          </div>
        </div>
      </div>

      {scanStatus && <ScanProgress status={scanStatus} />}

      {loading && <LoadingSpinner />}

      <div className="mt-8">
        <a
          href="/browse"
          className="text-blue-500 hover:text-blue-700 underline"
        >
          Browse Projects →
        </a>
      </div>

      <ConfirmDialog
        isOpen={showRescanConfirm}
        title="Rescan Library"
        message="This will check for new, modified, and deleted files. Existing tags will be preserved. Continue?"
        confirmLabel="Rescan"
        cancelLabel="Cancel"
        onConfirm={handleConfirmRescan}
        onCancel={() => setShowRescanConfirm(false)}
      />
    </div>
  );
};

export default HomePage;
