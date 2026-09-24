// ZAŠTO OVAKO (cigla M2/54 — raspored grafova kao čiste funkcije, S-035)
// Test dokazuje koordinate BEZ DOM-a: okvir, stupci (grupirani i naslagani), linija s datumima i
// najbliža točka za tooltip. Svelte komponente koje ovo crtaju nemaju vlastiti test (R37) — sve što
// se može pogrešno izračunati mora biti pokriveno ovdje.
import { describe, expect, it } from 'vitest';
import { barsLayout, frame, lineLayout, nearestIndex } from '../src/lib/charts/layout';

const S = [{ id: 'commits', label: 'commiti', color: 'var(--color-brand-500)' }];

describe('frame', () => {
  it('unutarnje mjere = vanjske − margine', () => {
    const f = frame(600, 200);
    expect(f.innerW).toBe(600 - 40 - 12);
    expect(f.innerH).toBe(200 - 12 - 28);
  });
});

describe('barsLayout', () => {
  it('prazan ulaz → bez stupaca, bez NaN, baseline unutar okvira', () => {
    const l = barsLayout(frame(600, 200), [], S, 'day', 'hr');
    expect(l.bars).toEqual([]);
    expect(Number.isFinite(l.baseline)).toBe(true);
    expect(l.yTicks.length).toBeGreaterThan(0);
  });
  it('tri dana → tri stupca, x ticksi s datumima, visina proporcionalna', () => {
    const l = barsLayout(frame(600, 200), [
      { start: '2026-09-10', values: { commits: 2 } },
      { start: '2026-09-11', values: { commits: 4 } },
      { start: '2026-09-12', values: { commits: 0 } },
    ], S, 'day', 'hr');
    expect(l.bars).toHaveLength(3);
    // Tri stupca su upravo tvrđena iznad — `!` je siguran, bez šireg suženja tipa.
    expect(l.bars[1]!.h).toBeCloseTo(l.bars[0]!.h * 2, 5);
    expect(l.bars[2]!.h).toBe(0);
    expect(l.xTicks.map((t) => t.label)).toEqual(['10.09.', '11.09.', '12.09.']);
    for (const b of l.bars) expect([b.x, b.y, b.w, b.h].every(Number.isFinite)).toBe(true);
  });
  it('naslagano: drugi niz sjedi na prvom', () => {
    const two = [...S, { id: 'hours', label: 'sati', color: 'var(--color-accent)' }];
    const l = barsLayout(frame(600, 200), [{ start: '2026-09-10', values: { commits: 2, hours: 3 } }], two, 'day', 'hr', true);
    expect(l.bars).toHaveLength(2);
    // Dva stupca su upravo tvrđena iznad — `!` je siguran, bez šireg suženja tipa.
    const a = l.bars[0]!;
    const b = l.bars[1]!;
    expect(b.y + b.h).toBeCloseTo(a.y, 5);
  });
  it('mnogo tjedana → ticksi prorijeđeni (≤ 10 natpisa)', () => {
    const buckets = Array.from({ length: 30 }, (_, i) => ({ start: `2026-${String(1 + Math.floor(i / 4)).padStart(2, '0')}-${String(1 + (i % 4) * 7).padStart(2, '0')}`, values: { commits: 1 } }));
    const l = barsLayout(frame(600, 200), buckets, S, 'week', 'hr');
    expect(l.xTicks.length).toBeLessThanOrEqual(10);
  });
});

describe('lineLayout', () => {
  it('dva niza → dvije putanje, ticksi s datumima, prazno → bez putanja', () => {
    const l = lineLayout(frame(600, 200), [
      { ...S[0]!, points: [{ date: '2026-09-10', value: 1 }, { date: '2026-09-20', value: 5 }] },
      { id: 'w', label: 'w', color: 'var(--color-warn)', points: [{ date: '2026-09-15', value: 2 }] },
    ], 'en');
    expect(l.paths).toHaveLength(2);
    expect(l.paths[0]!.d.startsWith('M')).toBe(true);
    expect(l.xTicks.length).toBeGreaterThan(1);
    expect(lineLayout(frame(600, 200), [], 'en').paths).toEqual([]);
  });
});

describe('nearestIndex', () => {
  it('najbliži x, −1 za prazno', () => {
    expect(nearestIndex([10, 20, 30], 22)).toBe(1);
    expect(nearestIndex([], 5)).toBe(-1);
  });
});
