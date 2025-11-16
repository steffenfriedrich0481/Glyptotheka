import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { NavigationProvider } from './store/navigationContext';
import './index.css';

function App() {
  return (
    <Router>
      <NavigationProvider>
        <div className="app">
          <header className="app-header">
            <h1>Glyptotheka - 3D Print Library</h1>
          </header>
          <main className="app-main">
            <Routes>
              <Route path="/" element={
                <div>
                  <p>Welcome to Glyptotheka! Setup in progress...</p>
                </div>
              } />
            </Routes>
          </main>
        </div>
      </NavigationProvider>
    </Router>
  );
}

export default App;

