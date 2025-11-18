import React, { useState, useEffect } from 'react';
import { configAPI, UpdateConfigRequest } from '../api/config';
import LoadingSpinner from '../components/common/LoadingSpinner';
import { Link } from 'react-router-dom';

const HomePage: React.FC = () => {
  const [rootPath, setRootPath] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [lastScanAt, setLastScanAt] = useState<number | null>(null);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const config = await configAPI.getConfig();
      if (config.root_path) {
        setRootPath(config.root_path);
      }
      if (config.last_scan_at) {
        setLastScanAt(config.last_scan_at);
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

  return (
    <div className="max-w-3xl mx-auto">
      <div className="mb-8">
        <h1 className="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
          3D Print Model Library
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          Configure your library and browse your 3D print collection
        </p>
      </div>

      <div className="card p-6 mb-6">
        <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
          Library Configuration
        </h2>
        
        <div className="mb-4">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Root Folder Path
          </label>
          <input
            className="input-field"
            type="text"
            placeholder="/projects"
            value={rootPath}
            onChange={(e) => setRootPath(e.target.value)}
          />
          <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
            Enter the path to your 3D model collection. When running in Docker, use <code className="px-1 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">/projects</code> which is mounted from your <code className="px-1 py-0.5 bg-gray-200 dark:bg-gray-700 rounded">PROJECTS_PATH</code> environment variable.
          </p>
        </div>

        {error && (
          <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
            {error}
          </div>
        )}

        <div className="flex items-center gap-4">
          <button
            className="btn-primary"
            onClick={handleSaveConfig}
            disabled={loading || !rootPath}
          >
            Save Configuration
          </button>
          
          {lastScanAt && (
            <p className="text-sm text-gray-600 dark:text-gray-400">
              Last scanned: {new Date(lastScanAt * 1000).toLocaleString()}
            </p>
          )}
        </div>
      </div>

      {loading && <LoadingSpinner />}

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <Link
          to="/browse"
          className="card p-6 hover:shadow-xl transition-shadow block"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center">
              <svg className="w-6 h-6 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">Browse Projects</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Explore your collection</p>
            </div>
          </div>
        </Link>

        <Link
          to="/search"
          className="card p-6 hover:shadow-xl transition-shadow block"
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 bg-primary-100 dark:bg-primary-900 rounded-lg flex items-center justify-center">
              <svg className="w-6 h-6 text-primary-600 dark:text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
            </div>
            <div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-gray-100">Search Projects</h3>
              <p className="text-sm text-gray-600 dark:text-gray-400">Find specific models</p>
            </div>
          </div>
        </Link>
      </div>
    </div>
  );
};

export default HomePage;
