import React, { useState, useEffect } from 'react';
import { configAPI, UpdateConfigRequest } from '../api/config';
import { scanAPI } from '../api/scan';
import LoadingSpinner from '../components/common/LoadingSpinner';

const HomePage: React.FC = () => {
  const [rootPath, setRootPath] = useState('');
  const [loading, setLoading] = useState(false);
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [scanStatus, setScanStatus] = useState<any>(null);

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

  const handleStartScan = async () => {
    setLoading(true);
    setError(null);
    try {
      await scanAPI.startScan();
      setScanning(true);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to start scan');
    } finally {
      setLoading(false);
    }
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
          <button
            className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            onClick={handleStartScan}
            disabled={loading || scanning || !rootPath}
          >
            {scanning ? 'Scanning...' : 'Start Scan'}
          </button>
        </div>
      </div>

      {scanning && scanStatus && (
        <div className="bg-blue-100 border-l-4 border-blue-500 text-blue-700 p-4">
          <p className="font-bold">Scan in progress...</p>
          {scanStatus.projects_found !== undefined && (
            <p>Projects found: {scanStatus.projects_found}</p>
          )}
          {scanStatus.files_processed !== undefined && (
            <p>Files processed: {scanStatus.files_processed}</p>
          )}
        </div>
      )}

      {loading && <LoadingSpinner />}

      <div className="mt-8">
        <a
          href="/browse"
          className="text-blue-500 hover:text-blue-700 underline"
        >
          Browse Projects →
        </a>
      </div>
    </div>
  );
};

export default HomePage;
