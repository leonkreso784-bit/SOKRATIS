// ZAŠTO OVAKO (cigla M2/20 — brana kontrasta)
// Test čita PRAVU datoteku tokena i traži nula nalaza; drugi test podmeće pokvaren CSS i traži da
// brana imenuje token — brana koja ne zna pasti nije brana. Datoteka se čita kroz
// `scripts/read-tokens.mjs` (node:fs), NE Viteovim `?raw` uvozom: probano je prvo s `?raw`, ali
// `@tailwindcss/vite` hvata svaki `*.css(?query)` i vrati prazan modul (izmjereno pod vitestom,
// vidi izvještaj T20) — a `node:fs` u `.ts` datoteci bi opet pao na svelte-checku jer nema
// `@types/node`. Rješenje je čitanje izmješteno u `.mjs`, koji svelte-check ne provjerava.
import { describe, expect, it } from 'vitest';
import { checkTokens, contrast, hue, parseHex } from '../scripts/contrast.mjs';
import { tokensCss as css } from '../scripts/read-tokens.mjs';

describe('kontrast tokena', () => {
  it('formule: bijela/crna 21:1, hue cijana ≈ 190°', () => {
    expect(contrast(parseHex('#ffffff')!, parseHex('#000000')!)).toBeCloseTo(21, 1);
    expect(Math.round(hue(parseHex('#0e7490')!))).toBeGreaterThan(180);
  });
  it('sve četiri teme prolaze WCAG i hue-odvojenost', () => {
    expect(checkTokens(css)).toEqual([]);
  });
  it('pokvaren brand-500 na svijetloj temi pada s imenom tokena i teme', () => {
    const broken = css.replace('--color-brand-500: #0e7490;', '--color-brand-500: #00dce8;');
    const f = checkTokens(broken);
    expect(f.some((x) => x.theme === 'academic' && x.token === 'brand-500')).toBe(true);
  });
});
