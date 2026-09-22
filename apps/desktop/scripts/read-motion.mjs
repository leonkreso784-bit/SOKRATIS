// ZAŠTO OVAKO (cigla M2/38 — čitanje motion.css mimo Viteove CSS-transformacije)
// Isti nalaz kao `read-tokens.mjs` (cigla M2/20): `@tailwindcss/vite` ima `enforce: 'pre'` transform
// koji hvata SVAKI uvoz koji završava na `.css` (uključujući `...css?raw`) i vraća prazan modul kad
// datoteka nema vlastiti `@import "tailwindcss"` — izmjereno pod vitestom, `motion.css?raw` iz brifa
// vraća `""`. `node:fs` izvan Viteova grafa čita datoteku onakvu kakva jest; `.mjs` jer bi `node:fs`
// u `.ts` pao na svelte-checku (nema `@types/node`, isti razlog kao `contrast.test.ts`).
import { readFileSync } from 'node:fs';

export const motionCss = readFileSync(new URL('../src/styles/motion.css', import.meta.url), 'utf8');
