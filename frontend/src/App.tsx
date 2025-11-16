import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import HomePage from './pages/HomePage';
import BrowsePage from './pages/BrowsePage';
import ProjectPage from './pages/ProjectPage';
import './index.css';

function App() {
  return (
    <Router>
      <NavigationProvider>
        <div className="app min-h-screen bg-gray-100">
          <main className="app-main">
            <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="/browse" element={<BrowsePage />} />
              <Route path="/project/:id" element={<ProjectPage />} />
            </Routes>
          </main>
        </div>
      </NavigationProvider>
    </Router>
  );
}

export default App;

