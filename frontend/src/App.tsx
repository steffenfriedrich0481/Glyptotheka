import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import { SearchProvider } from './store/searchContext';
import { ToastProvider } from './components/common/Toast';
import { ErrorBoundary } from './components/ErrorBoundary';
import HomePage from './pages/HomePage';
import BrowsePage from './pages/BrowsePage';
import ProjectPage from './pages/ProjectPage';
import { SearchPage } from './pages/SearchPage';
import { SearchBar } from './components/common/SearchBar';
import './index.css';

function App() {
  return (
    <ErrorBoundary>
      <Router>
        <ToastProvider>
          <NavigationProvider>
            <SearchProvider>
              <div className="app min-h-screen bg-gray-100">
                <header className="app-header bg-white shadow-md p-3 sm:p-4 mb-4">
                  <div className="max-w-7xl mx-auto">
                    {/* Mobile Layout */}
                    <div className="flex flex-col gap-3 sm:hidden">
                      <div className="flex items-center justify-between">
                        <Link to="/" className="text-lg font-bold text-gray-800 hover:text-blue-600">
                          3D Print Library
                        </Link>
                        <nav className="flex gap-3">
                          <Link to="/" className="text-sm text-gray-600 hover:text-blue-600">Home</Link>
                          <Link to="/browse" className="text-sm text-gray-600 hover:text-blue-600">Browse</Link>
                        </nav>
                      </div>
                      <SearchBar />
                    </div>
                    {/* Desktop/Tablet Layout */}
                    <div className="hidden sm:flex items-center justify-between gap-4">
                      <Link to="/" className="text-xl font-bold text-gray-800 hover:text-blue-600 whitespace-nowrap">
                        3D Print Library
                      </Link>
                      <div className="flex-1 max-w-2xl">
                        <SearchBar />
                      </div>
                      <nav className="flex gap-4 whitespace-nowrap">
                        <Link to="/" className="text-gray-600 hover:text-blue-600">Home</Link>
                        <Link to="/browse" className="text-gray-600 hover:text-blue-600">Browse</Link>
                      </nav>
                    </div>
                  </div>
                </header>
                <main className="app-main px-3 sm:px-4">
                  <Routes>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/browse" element={<BrowsePage />} />
                    <Route path="/project/:id" element={<ProjectPage />} />
                    <Route path="/projects/:id" element={<ProjectPage />} />
                    <Route path="/search" element={<SearchPage />} />
                  </Routes>
                </main>
              </div>
            </SearchProvider>
          </NavigationProvider>
        </ToastProvider>
      </Router>
    </ErrorBoundary>
  );
}

export default App;

