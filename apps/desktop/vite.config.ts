// ZAŠTO OVAKO (cigla M2/1 — Vite kostur)
// Dva ulaza (glavni prozor i splash) = dvije HTML stranice u jednom buildu; Tauri ih otvara kao
// dva prozora. Port 1420 je fiksan jer ga `tauri.conf.json` očekuje (`devUrl`).
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';
import { fileURLToPath } from 'node:url';

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  clearScreen: false,
  // `fs.allow` do korijena repoa: MockApi (M2/25) uvozi insta snapshot iz `crates/…/tests/snapshots`
  // s `?raw` — jedan izvor podataka za dev-prikaz i za test, bez kopije (S-010).
  server: {
    port: 1420,
    strictPort: true,
    fs: { allow: [fileURLToPath(new URL('../..', import.meta.url))] },
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      input: {
        main: fileURLToPath(new URL('./index.html', import.meta.url)),
        splash: fileURLToPath(new URL('./splash.html', import.meta.url)),
      },
    },
  },
  test: { include: ['tests/**/*.test.ts'] },
});
