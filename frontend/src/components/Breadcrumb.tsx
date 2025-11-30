import { Link } from 'react-router-dom';

export interface BreadcrumbItem {
  name: string;
  path: string;
}

interface BreadcrumbProps {
  items: BreadcrumbItem[];
  currentPath?: string;
}

export function Breadcrumb({ items, currentPath }: BreadcrumbProps) {
  return (
    <nav className="flex items-center space-x-2 text-sm text-gray-600 dark:text-theme-muted mb-4">
      {items.map((item, index) => {
        const isLast = index === items.length - 1;
        const isCurrent = item.path === currentPath;
        
        return (
          <div key={item.path} className="flex items-center">
            {index > 0 && (
              <span className="mx-2 text-theme-muted">/</span>
            )}
            {isLast || isCurrent ? (
              <span className="font-medium text-gray-900 dark:text-theme">
                {item.name}
              </span>
            ) : (
              <Link
                to={`/browse/${item.path}`}
                className="hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
              >
                {item.name}
              </Link>
            )}
          </div>
        );
      })}
    </nav>
  );
}
