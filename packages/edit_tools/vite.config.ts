// vite.config.ts
import path from "path";
import { defineConfig } from 'vite';

export default defineConfig({
  root: './',
  base: "./",
  build: {
    outDir: 'dist',
  },
  publicDir: 'public',
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
    mainFields: ['module', 'jsnext:main', 'jsnext'],

  },
    // Add this new configuration
    optimizeDeps: {
      include: ['@lexical/utils'],
    },
 
});
