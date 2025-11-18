import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import { SearchProvider } from './store/searchContext';
import { ToastProvider } from './components/common/Toast';
import { ErrorBoundary } from './components/ErrorBoundary';
import { NavBar } from './components/common/NavBar';
import BrowsePage from './pages/BrowsePage';
import ProjectPage from './pages/ProjectPage';
import { SearchPage } from './pages/SearchPage';
import './index.css';

function App() {
  return (
    <ErrorBoundary>
      <Router>
        <ToastProvider>
          <NavigationProvider>
            <SearchProvider>
              <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
                <NavBar />
                <main className="app">
                  <Routes>
                    <Route path="/" element={<SearchPage />} />
                    <Route path="/browse" element={<BrowsePage />} />
                    <Route path="/project/:id" element={<ProjectPage />} />
                    <Route path="/projects/:id" element={<ProjectPage />} />
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

