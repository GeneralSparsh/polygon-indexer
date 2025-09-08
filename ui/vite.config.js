import { defineConfig } from 'vite'

export default defineConfig({
  // Use ui/src as the project root (where index.html currently is)
  root: 'src',

  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:3000',
      '/ws': {
        target: 'ws://localhost:3000',
        ws: true,
      },
    },
    // optional: open the built entry in dev
    // open: '/index.html',
  },

  build: {
    // Output build to ui/dist (one level up from src)
    outDir: '../dist',
    assetsDir: 'assets',
    emptyOutDir: true, // required because outDir is outside of root
    rollupOptions: {
      // Explicitly set HTML entry to src/index.html (optional when root is set)
      input: 'src/index.html',
    },
  },
})
