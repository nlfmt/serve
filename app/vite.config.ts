import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import { fileURLToPath } from 'node:url'
import fs from "fs"

const alias = (path: string) => fileURLToPath(new URL(path, import.meta.url))

function getCargoVersion() {
  const cargoToml = fs.readFileSync("../Cargo.toml", "utf-8")
  const match = cargoToml.match(/version\s*=\s*"([^"]+)"/)
  return match?.[1] || "unknown"
}

export default defineConfig({
  root: "./src",
  publicDir: "../public",
  
  define: {
    __VERSION__: `"${getCargoVersion()}"`
  },

  resolve: {
    alias: {
      "@": alias("./src"),
    },
  },

  build: {
    outDir: "../dist",
    emptyOutDir: true,
    minify: "esbuild",
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
