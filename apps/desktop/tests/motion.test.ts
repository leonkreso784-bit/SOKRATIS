// ZAŠTO OVAKO (cigla M2/38): vitest ovdje nema DOM, pa se pokret čuva na dva mjesta koja se DAJU
// testirati — čista odluka `motionOff` i sam tekst `motion.css`. ODSTUPANJE OD BRIFA: `?raw` na
// `.css` pod `@tailwindcss/vite` vraća prazan modul (isti nalaz kao `read-tokens.mjs`, M2/20), pa
// čitanje ide kroz `scripts/read-motion.mjs` (node:fs). Pravila niže vrijede i za T39–T41 dodatke u
// istu datoteku.
import { describe, expect, it } from 'vitest';
import { motionOff } from '../src/lib/motion';
import { motionCss as css } from '../scripts/read-motion.mjs';

describe('pokret', () => {
  it('gasi ga prekidač ILI prefers-reduced-motion', () => {
    expect(motionOff(true, false)).toBe(false);
    expect(motionOff(false, false)).toBe(true);
    expect(motionOff(true, true)).toBe(true);
    expect(motionOff(false, true)).toBe(true);
  });
  it('trajanje je jedna varijabla i nije veće od 250 ms (S-026)', () => {
    const m = css.match(/--motion-dur:\s*(\d+)ms/);
    expect(m).not.toBeNull();
    expect(Number(m![1])).toBeLessThanOrEqual(250);
  });
  it('svaka animacija i prijelaz u motion.css troše --motion-dur', () => {
    const decls = [...css.matchAll(/^\s*(?:animation|transition):\s*([^;]+);/gm)].map((d) => d[1]!);
    for (const d of decls) expect(d, d).toContain('var(--motion-dur)');
  });
  it('data-motion="off" svodi sva trajanja na nulu', () => {
    expect(css).toMatch(/:root\[data-motion="off"\][^{]*\{[^}]*animation-duration:\s*0s\s*!important/);
    expect(css).toMatch(/:root\[data-motion="off"\][^{]*\{[^}]*transition-duration:\s*0s\s*!important/);
  });
  it('grafovi imaju ulazne animacije definirane na jednom mjestu', () => {
    for (const name of ['chart-grow', 'chart-draw', 'chart-fade']) expect(css).toContain(`@keyframes ${name}`);
    for (const cls of ['.chart-bar', '.chart-line', '.chart-dot']) expect(css).toContain(cls);
  });
});
