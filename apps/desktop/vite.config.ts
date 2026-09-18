// ZAŠTO OVAKO (cigla M2/1 — Vite kostur)
// Dva ulaza (glavni prozor i splash) = dvije HTML stranice u jednom buildu; Tauri ih otvara kao
// dva prozora. Port 1420 je fiksan jer ga `tauri.conf.json` očekuje (`devUrl`).
// `defineConfig` dolazi iz `vitest/config` jer samo taj tip poznaje ključ `test`. Putanje su
// relativne prema korijenu (`apps/desktop`) — Vite ih sam razrješava, pa ne treba `node:url`
// ni paket `@types/node` samo radi ove datoteke (ovisnost manje, pravilo #6).
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [svelte(), tailwindcss()],
  clearScreen: false,
  // `fs.allow` do korijena repoa: MockApi (M2/25) uvozi insta snapshot iz `crates/…/tests/snapshots`
  // s `?raw` — jedan izvor podataka za dev-prikaz i za test, bez kopije (S-010).
  server: {
    port: 1420,
    strictPort: true,
    fs: { allow: ['../..'] },
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      input: {
        main: 'index.html',
        splash: 'splash.html',
      },
    },
  },
  test: { include: ['tests/**/*.test.ts'] },
});
