import React, { createContext, useContext, useState, useEffect } from 'react';

export type Theme = 'default' | 'ocean' | 'sunset' | 'forest' | 'purple-haze' | 'cyberpunk';

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export const themeColors: Record<Theme, { name: string; colors: Record<string, string> }> = {
  default: {
    name: 'Default Dark',
    colors: {
      primary: '#3b82f6',
      'primary-dark': '#2563eb',
      secondary: '#6b7280',
      background: '#111827',
      'background-lighter': '#1f2937',
      text: '#f9fafb',
      'text-muted': '#9ca3af',
      accent: '#10b981',
    },
  },
  ocean: {
    name: 'Ocean Breeze',
    colors: {
      primary: '#0ea5e9',
      'primary-dark': '#0284c7',
      secondary: '#06b6d4',
      background: '#0c1821',
      'background-lighter': '#1a2332',
      text: '#e0f2fe',
      'text-muted': '#7dd3fc',
      accent: '#22d3ee',
    },
  },
  sunset: {
    name: 'Sunset Glow',
    colors: {
      primary: '#f59e0b',
      'primary-dark': '#d97706',
      secondary: '#ef4444',
      background: '#1c0f0a',
      'background-lighter': '#2d1810',
      text: '#fef3c7',
      'text-muted': '#fbbf24',
      accent: '#fb923c',
    },
  },
  forest: {
    name: 'Forest Night',
    colors: {
      primary: '#10b981',
      'primary-dark': '#059669',
      secondary: '#34d399',
      background: '#0a1510',
      'background-lighter': '#132520',
      text: '#d1fae5',
      'text-muted': '#6ee7b7',
      accent: '#5eead4',
    },
  },
  'purple-haze': {
    name: 'Purple Haze',
    colors: {
      primary: '#a855f7',
      'primary-dark': '#9333ea',
      secondary: '#c084fc',
      background: '#1a0f24',
      'background-lighter': '#2d1b3d',
      text: '#f3e8ff',
      'text-muted': '#d8b4fe',
      accent: '#e879f9',
    },
  },
  cyberpunk: {
    name: 'Cyberpunk',
    colors: {
      primary: '#ff00ff',
      'primary-dark': '#cc00cc',
      secondary: '#00ffff',
      background: '#0a0014',
      'background-lighter': '#1a0028',
      text: '#f0f0ff',
      'text-muted': '#b19cd9',
      accent: '#00ff41',
    },
  },
};

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [theme, setThemeState] = useState<Theme>(() => {
    const saved = localStorage.getItem('glyptotheka-theme');
    return (saved as Theme) || 'default';
  });

  useEffect(() => {
    localStorage.setItem('glyptotheka-theme', theme);
    
    // Apply CSS variables to root
    const colors = themeColors[theme].colors;
    const root = document.documentElement;
    
    Object.entries(colors).forEach(([key, value]) => {
      root.style.setProperty(`--color-${key}`, value);
    });
  }, [theme]);

  const setTheme = (newTheme: Theme) => {
    setThemeState(newTheme);
  };

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
};
