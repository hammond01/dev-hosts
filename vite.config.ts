import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

const tauriHost = process.env.TAURI_DEV_HOST;

export default defineConfig({
  clearScreen: false,
  plugins: [solid()],
  server: {
    port: 1420,
    strictPort: true,
    host: tauriHost ?? false,
    hmr: tauriHost
      ? {
          protocol: "ws",
          host: tauriHost,
          port: 1421
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"]
    }
  }
});

