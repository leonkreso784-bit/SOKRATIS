// ZAŠTO OVAKO (cigla M2/20 — čitanje tokena mimo Viteove CSS-transformacije)
// Isprobano prvo s Viteovim `?raw` uvozom (kako brif predlaže) — pao je: `@tailwindcss/vite`
// ima `enforce: 'pre'` transform s filtrom `/\.css(?:\?.*)?$/`, koji hvata SVAKI uvoz koji
// završava na `.css`, uključujući `...css?raw`, i vraća prazan modul jer tokens.css nema
// vlastiti `@import "tailwindcss"` (izmjereno: uvezeni niz duljine 0 pod vitestom).
// `node:fs` izvan Viteova grafa čita datoteku onakvu kakva jest na disku — mehanizam koji već
// koristi `check-contrast.mjs`; ovaj modul je JEDNO mjesto čitanja za oboje (test i CLI brana, S-010).
import { readFileSync } from 'node:fs';

export const tokensCss = readFileSync(new URL('../src/styles/tokens.css', import.meta.url), 'utf8');
