// ZAŠTO OVAKO (cigla M2/53 — d3-matematika bez DOM-a, S-035)
// Testovi provjeravaju da su naši omotači oko `d3-scale`/`d3-time-format` konačni (bez NaN),
// monotoni i dvojezični — komponente (T54+) im vjeruju bez ponovne provjere.
import { describe, expect, it } from 'vitest';
import { dateTicks, formatDate, niceMax, parseYmd, toYmd, yTicks } from '../src/lib/charts/scales';

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
});
