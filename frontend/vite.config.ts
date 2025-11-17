import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': {
        target: 'http://localhost:3000',
        changeOrigin: true,
      },
    },
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          'react-vendor': ['react', 'react-dom', 'react-router-dom'],
          'ui-components': [
            './src/components/common/Breadcrumb.tsx',
            './src/components/common/LoadingSpinner.tsx',
            './src/components/common/Pagination.tsx',
            './src/components/common/SearchBar.tsx',
            './src/components/common/Skeleton.tsx',
            './src/components/common/EmptyState.tsx',
            './src/components/common/Toast.tsx',
          ],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
    minify: 'esbuild', // Use esbuild instead of terser (faster and no extra dep)
  },
  optimizeDeps: {
    include: ['react', 'react-dom', 'react-router-dom'],
  },
})
