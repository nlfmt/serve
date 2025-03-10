import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { fileURLToPath } from 'node:url'

const alias = (path: string) => fileURLToPath(new URL(path, import.meta.url))

export default defineConfig({
  root: "./src",
  publicDir: "../public",

  resolve: {
    alias: {
      "@": alias("./src"),
    },
  },

  build: {
    outDir: "../dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: "./src/index.html",
      },
    },
  },

  plugins: [react()],

  server: {
    port: 3001,
    host: "0.0.0.0",
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
        secure: false,
        ws: true,
      },
    },
  },
})
