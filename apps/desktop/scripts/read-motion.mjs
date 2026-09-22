// ZAŠTO OVAKO (cigla M2/38 — čitanje motion.css mimo Viteove CSS-transformacije)
// Isti nalaz kao `read-tokens.mjs` (M2/20): `@tailwindcss/vite` hvata SVAKI `.css` uvoz i vraća
// prazan modul kad datoteka nema vlastiti `@import "tailwindcss"` — `?raw` iz brifa daje `""`.
// `node:fs` izvan Viteova grafa čita datoteku onakvu kakva jest; `.mjs` jer bi `node:fs` u `.ts`
// pao na svelte-checku (nema `@types/node`, isti razlog kao `contrast.test.ts`).
import { readFileSync } from 'node:fs';

export const motionCss = readFileSync(new URL('../src/styles/motion.css', import.meta.url), 'utf8');
