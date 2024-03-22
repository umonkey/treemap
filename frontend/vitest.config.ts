import { defineConfig, mergeConfig } from "vitest/config"
import viteConfig from "./vite.config"

export default mergeConfig(
  viteConfig,
  defineConfig({
    test: {
      globals: true,
      environment: "jsdom",
      setupFiles: "./vitest.setup.ts",
      css: true,
      include: [
        "./src/**/*.test.ts",
        "./src/**/*.test.tsx",
      ],
      exclude: [
        "./src/**/*.stories.tsx",
      ],
      coverage: {
        reportsDirectory: "./var/coverage",
        provider: "v8",
        include: [
          "src/**/*.ts",
          "src/**/*.tsx",
        ],
        exclude: [
          "src/**/index.ts",
          "src/**/*.stories.tsx",
        ],
      }
    },
  })
);
