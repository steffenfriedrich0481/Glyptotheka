import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { ScanButton } from '../scan/ScanButton';
import { ThemeSelector } from './ThemeSelector';

export const NavBar: React.FC = () => {
  const location = useLocation();

  return (
    <>
      {/* Skip to main content link for accessibility */}
      <a
        href="#main-content"
        className="sr-only focus:not-sr-only focus:absolute focus:top-4 focus:left-4 focus:z-[100] focus:px-4 focus:py-2 focus:bg-theme-primary focus:text-theme focus:rounded focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
      >
        Skip to main content
      </a>
      
      <nav className="bg-theme-lighter border-b border-theme sticky top-0 z-50 shadow-lg" role="navigation" aria-label="Main navigation">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            {/* Logo and Title */}
            <div className="flex items-center">
              <Link to="/" className="flex items-center space-x-3" aria-label="Glyptotheka home">
                <svg className="h-8 w-8 text-theme-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                </svg>
                <span className="text-xl font-bold text-theme">Glyptotheka</span>
              </Link>
            </div>

            {/* Navigation Links */}
            <div className="flex items-center space-x-8">
              <Link
                to="/browse"
                className={`text-sm font-medium transition-colors ${
                  location.pathname.startsWith('/browse')
                    ? 'text-theme'
                    : 'text-theme-muted hover:text-theme'
                }`}
                aria-current={location.pathname.startsWith('/browse') ? 'page' : undefined}
              >
                Browse
              </Link>
              
              <Link
                to="/search"
                className={`text-sm font-medium transition-colors ${
                  location.pathname === '/search' || location.pathname === '/'
                    ? 'text-theme'
                    : 'text-theme-muted hover:text-theme'
                }`}
                aria-current={location.pathname === '/search' || location.pathname === '/' ? 'page' : undefined}
              >
                Search
              </Link>

              {/* Theme Selector */}
              <ThemeSelector />

              {/* Scan Button */}
              <div className="relative">
                <ScanButton />
              </div>
            </div>
          </div>
        </div>
      </nav>
    </>
  );
};
