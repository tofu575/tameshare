import { defineConfig } from "@playwright/test";

// ローカルまたはCIの起動中Backendと、Frontend開発サーバーで主要操作を検査する。
export default defineConfig({
  testDir: "./tests",
  workers: 1,
  use: {
    baseURL: "http://127.0.0.1:3000",
  },
  webServer: {
    command: "npm run dev",
    url: "http://127.0.0.1:3000/practices",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
