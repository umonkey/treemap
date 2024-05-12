import { sentryVitePlugin } from "@sentry/vite-plugin";
import { defineConfig } from 'vite';
import { resolve } from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },

  build: {
    sourcemap: true,
  },

  plugins: [sentryVitePlugin({
    org: "trees-of-yerevan",
    project: "javascript-react"
  })]
})
