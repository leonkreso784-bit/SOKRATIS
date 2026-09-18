// ZAŠTO OVAKO (cigla M2/23 — grafovi bez biblioteke)
// Sva matematika grafova živi u `scale.ts`; komponente se ne testiraju renderiranjem (S-018), pa
// je ovo jedino mjesto koje jamči da SVG nikad ne dobije NaN/Infinity u atributu.
import { describe, expect, it } from 'vitest';
import { arcPath, cumulative, linePath, linear, niceMax, ringSegments, ticks } from '../src/lib/charts/scale';

describe('scale', () => {
  it('linear preslikava domenu u raspon (i obrnuto za y)', () => {
    const y = linear([0, 10], [100, 0]);
    expect(y(0)).toBe(100);
    expect(y(10)).toBe(0);
    expect(y(5)).toBe(50);
    expect(linear([5, 5], [0, 100])(5)).toBe(0);
  });
  it('niceMax i ticks daju čitke granice', () => {
    expect([0, 7, 13, 190, 70164].map(niceMax)).toEqual([1, 8, 15, 200, 80000]);
    expect(ticks(190)).toEqual([0, 50, 100, 150, 200]);
  });
  it('cumulative, linePath, ringSegments, arcPath', () => {
    expect(cumulative([1, 2, 3])).toEqual([1, 3, 6]);
    expect(linePath([])).toBe('');
    expect(
      linePath([
        { x: 0, y: 1 },
        { x: 2, y: 3 },
      ]),
    ).toBe('M0 1 L2 3');
    expect(ringSegments([0, 0])).toEqual([]);
    const s = ringSegments([1, 3]);
    expect(s.map((x) => +x.share.toFixed(2))).toEqual([0.25, 0.75]);
    expect(s[1]!.end).toBeCloseTo(Math.PI * 2, 6);
    expect(arcPath(50, 50, 40, 0, Math.PI / 2)).toMatch(/^M50 10 A40 40 0 0 1 90 50$/);
  });

  // Rubovi izvan brifa: prazan niz, jedan element, sve nule, negativna vrijednost, puni krug —
  // scale.ts mora ostati konačan (bez NaN/Infinity) jer SVG s NaN tiho ne crta ništa.
  it('prazan niz i jedan element ne ruše matematiku', () => {
    expect(niceMax(0)).toBe(1);
    expect(cumulative([])).toEqual([]);
    expect(linePath([{ x: 0, y: 1 }])).toBe('M0 1');
    expect(ringSegments([])).toEqual([]);
  });
  it('negativan max i negativna vrijednost ostaju konačni', () => {
    expect(niceMax(-5)).toBe(1);
    const s = ringSegments([-1, 3]);
    expect(s.every((x) => Number.isFinite(x.share) && Number.isFinite(x.start) && Number.isFinite(x.end))).toBe(
      true,
    );
  });
  it('puni krug (jedan segment = 100 %) crta konačan, ne prazan luk', () => {
    const full = arcPath(50, 50, 40, 0, Math.PI * 2);
    expect(full).not.toMatch(/NaN|Infinity/);
    const nums = full.match(/-?\d+(\.\d+)?/g)!.map(Number);
    expect(nums.every((n) => Number.isFinite(n))).toBe(true);
    // Početna i završna točka luka se NE smiju poklopiti — inače se puni krug ne crta.
    expect(full.startsWith('M50 10')).toBe(true);
    expect(full.endsWith('50 10')).toBe(false);
  });
});
