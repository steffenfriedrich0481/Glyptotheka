import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import { SearchProvider } from './store/searchContext';
import HomePage from './pages/HomePage';
import BrowsePage from './pages/BrowsePage';
import ProjectPage from './pages/ProjectPage';
import { SearchPage } from './pages/SearchPage';
import { SearchBar } from './components/common/SearchBar';
import './index.css';

function App() {
  return (
    <Router>
      <NavigationProvider>
        <SearchProvider>
          <div className="app min-h-screen bg-gray-100">
            <header className="app-header bg-white shadow-md p-4 mb-4">
              <div className="max-w-7xl mx-auto flex items-center justify-between gap-4">
                <Link to="/" className="text-xl font-bold text-gray-800 hover:text-blue-600">
                  3D Print Library
                </Link>
                <div className="flex-1 max-w-2xl">
                  <SearchBar />
                </div>
                <nav className="flex gap-4">
                  <Link to="/" className="text-gray-600 hover:text-blue-600">Home</Link>
                  <Link to="/browse" className="text-gray-600 hover:text-blue-600">Browse</Link>
                </nav>
              </div>
            </header>
            <main className="app-main">
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
    </Router>
  );
}

export default App;

