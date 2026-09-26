// ZAŠTO OVAKO (cigla M2/53 — d3-matematika bez DOM-a, S-035)
// Testovi provjeravaju da su naši omotači oko `d3-scale`/`d3-time-format` konačni (bez NaN),
// monotoni i dvojezični — komponente (T54+) im vjeruju bez ponovne provjere.
import { describe, expect, it } from 'vitest';
import {
  arcPath,
  dateTicks,
  finiteMax,
  formatDate,
  formatMonth,
  formatWeekday,
  linear,
  niceMax,
  parseYmd,
  ringSegments,
  svgA11y,
  toYmd,
  yTicks,
} from '../src/lib/charts/scales';

describe('scales', () => {
  it('parseYmd/toYmd su inverzi u UTC-u', () => {
    expect(toYmd(parseYmd('2026-09-10'))).toBe('2026-09-10');
    expect(parseYmd('2026-09-10').getUTCHours()).toBe(0);
  });
  it('7 dana → dnevni ticksi s HR datumima, 6 mjeseci → mjesečni', () => {
    const week = dateTicks([parseYmd('2026-09-07'), parseYmd('2026-09-13')], 600, 'hr');
    expect(week.length).toBeGreaterThanOrEqual(4);
    expect(week[0]?.label).toMatch(/^\d{2}\.\d{2}\.$/); // '07.09.'
    const half = dateTicks([parseYmd('2026-03-01'), parseYmd('2026-09-01')], 600, 'hr');
    expect(half.some((t) => /ožu|tra|svi|lip|srp|kol|ruj/.test(t.label))).toBe(true);
    const en = dateTicks([parseYmd('2026-03-01'), parseYmd('2026-09-01')], 600, 'en');
    expect(en.some((t) => /Mar|Apr|May|Jun|Jul|Aug|Sep/.test(t.label))).toBe(true);
  });
  it('ticksi su unutar raspona i monotoni', () => {
    const t = dateTicks([parseYmd('2026-08-29'), parseYmd('2026-09-23')], 600, 'en');
    for (let i = 1; i < t.length; i++) expect(t[i]!.x).toBeGreaterThan(t[i - 1]!.x);
    expect(t[0]?.x).toBeGreaterThanOrEqual(0);
    expect(t.at(-1)!.x).toBeLessThanOrEqual(600);
  });
  it('yTicks i niceMax bez NaN i za nulu', () => {
    expect(yTicks(0)).toEqual([0, 1]);
    expect(niceMax(0)).toBe(1);
    expect(niceMax(Number.NaN)).toBe(1);
    expect(yTicks(37)).toEqual([0, 10, 20, 30, 40]);
  });
  it('formatDate po zrnatosti', () => {
    expect(formatDate('2026-09-10', 'hr', 'day')).toBe('10.09.');
    expect(formatDate('2026-09-07', 'hr', 'week')).toBe('tj. 37');
    expect(formatDate('2026-09-01', 'hr', 'month')).toBe('ruj 2026');
    expect(formatDate('2026-09-01', 'en', 'month')).toBe('Sep 2026');
  });
  // Dopunjeno M2/56 (dopuna T56) — toplinska karta treba mjesec BEZ godine i dan BEZ datuma.
  it('formatMonth vraća skraćeni mjesec bez godine', () => {
    expect(formatMonth('2026-09-01', 'hr')).toBe('ruj');
  });
  it('formatWeekday po ISO danu (pon = 0 … ned = 6), neovisno o jeziku', () => {
    expect(formatWeekday(0, 'hr')).toBe('pon');
    expect(formatWeekday(2, 'en')).toBe('Wed');
  });
});

// Preseljeno iz scale.ts (M2/23, obrisan M2/55) — `linear`, `finiteMax`, `arcPath`, `ringSegments`,
// `svgA11y` i dalje trebaju vlastiti test jer `Sparkline`/`Ring` ne prolaze kroz `layout.ts` (S-035).
describe('preseljeno iz scale.ts (M2/55)', () => {
  it('linear preslikava domenu u raspon (i obrnuto za y)', () => {
    const y = linear([0, 10], [100, 0]);
    expect(y(0)).toBe(100);
    expect(y(10)).toBe(0);
    expect(y(5)).toBe(50);
    expect(linear([5, 5], [0, 100])(5)).toBe(0);
  });
  it('finiteMax računa maksimum SAMO nad konačnim vrijednostima', () => {
    expect(finiteMax([1, Number.NaN, 5])).toBe(5);
    expect(finiteMax([Number.NaN])).toBe(0);
    expect(finiteMax([])).toBe(0);
  });
  it('ringSegments i arcPath ostaju konačni na rubovima (prazno/nula/negativno/NaN)', () => {
    expect(ringSegments([0, 0])).toEqual([]);
    expect(ringSegments([])).toEqual([]);
    const s = ringSegments([1, 3]);
    expect(s.map((x) => +x.share.toFixed(2))).toEqual([0.25, 0.75]);
    expect(s[1]!.end).toBeCloseTo(Math.PI * 2, 6);
    expect(arcPath(50, 50, 40, 0, Math.PI / 2)).toMatch(/^M50 10 A40 40 0 0 1 90 50$/);
    const neg = ringSegments([-1, 3]);
    expect(neg.every((x) => Number.isFinite(x.share) && Number.isFinite(x.start) && Number.isFinite(x.end))).toBe(
      true,
    );
    expect(ringSegments([1, Number.NaN])).toEqual([]);
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
  it('svgA11y: graf S imenom je slika, BEZ imena je ukras', () => {
    expect(svgA11y('x')).toEqual({ role: 'img', 'aria-label': 'x' });
    expect(svgA11y(undefined)).toEqual({ 'aria-hidden': 'true' });
    expect(svgA11y('')).toEqual({ 'aria-hidden': 'true' });
    expect(svgA11y('   ')).toEqual({ 'aria-hidden': 'true' });
  });
});
