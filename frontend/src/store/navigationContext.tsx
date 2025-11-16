import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';

interface BreadcrumbItem {
  id: number;
  name: string;
  path: string;
}

interface NavigationContextType {
  breadcrumbs: BreadcrumbItem[];
  setBreadcrumbs: (breadcrumbs: BreadcrumbItem[]) => void;
  addBreadcrumb: (item: BreadcrumbItem) => void;
  navigateToLevel: (index: number) => void;
  currentPath: string | null;
  setCurrentPath: (path: string | null) => void;
}

const NavigationContext = createContext<NavigationContextType | undefined>(undefined);

export const NavigationProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [breadcrumbs, setBreadcrumbsState] = useState<BreadcrumbItem[]>([]);
  const [currentPath, setCurrentPath] = useState<string | null>(null);

  const setBreadcrumbs = useCallback((newBreadcrumbs: BreadcrumbItem[]) => {
    setBreadcrumbsState(newBreadcrumbs);
  }, []);

  const addBreadcrumb = useCallback((item: BreadcrumbItem) => {
    setBreadcrumbsState((prev) => [...prev, item]);
  }, []);

  const navigateToLevel = useCallback((index: number) => {
    setBreadcrumbsState((prev) => prev.slice(0, index + 1));
  }, []);

  return (
    <NavigationContext.Provider
      value={{
        breadcrumbs,
        setBreadcrumbs,
        addBreadcrumb,
        navigateToLevel,
        currentPath,
        setCurrentPath,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};

export const useNavigation = (): NavigationContextType => {
  const context = useContext(NavigationContext);
  if (!context) {
    throw new Error('useNavigation must be used within NavigationProvider');
  }
  return context;
};
