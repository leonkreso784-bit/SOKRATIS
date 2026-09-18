// ZAŠTO OVAKO (cigla M2/24 — splash)
// `frame(ms)` je čista funkcija bez platna, pa se testira izravno bez laživog DOM-a (bez jsdom).
// Vrijednosti su Leonove (sokratis-intro-clean-graph.html, 2026-09-18) — brojke su ovdje ugovor.
import { describe, expect, it } from 'vitest';
import { TOTAL_MS, frame, once } from '../src/splash/intro';

describe('vremenska crta animacije (Leon, 2026-09-18)', () => {
  it('na 0 ms ništa nije počelo', () => {
    const f = frame(0);
    expect(f.bars).toEqual([0, 0, 0, 0]);
    expect(f.ring).toBe(0);
    expect(f.portrait).toBe(0);
    expect(f.settle).toBe(0);
  });
  it('stupci rastu jedan za drugim (razmak 110 ms)', () => {
    const f = frame(500);
    expect(f.bars[0]).toBeGreaterThan(f.bars[1]);
    expect(f.bars[1]).toBeGreaterThan(f.bars[2]);
    expect(f.bars[2]).toBeGreaterThan(f.bars[3]);
  });
  it('prsten kreće na 1400, lik na 2200, skupljanje na 3400', () => {
    expect(frame(1399).ring).toBe(0);
    expect(frame(1600).ring).toBeGreaterThan(0);
    expect(frame(2199).portrait).toBe(0);
    expect(frame(2700).portrait).toBeGreaterThan(0);
    expect(frame(3399).settle).toBe(0);
    expect(frame(3800).settle).toBeGreaterThan(0);
  });
  it('na kraju je sve 1 i TOTAL_MS je 4200', () => {
    const f = frame(TOTAL_MS);
    expect(f.bars).toEqual([1, 1, 1, 1]);
    expect(f.ring).toBe(1);
    expect(f.portrait).toBe(1);
    expect(f.settle).toBe(1);
    expect(TOTAL_MS).toBe(4200);
  });
  // izvan brifa, dodano: rubovi koje `ease`/`clamp` moraju stegnuti bez NaN-a u izlazu
  it('negativno, predugo i NaN vrijeme daju vrijednosti stegnute u 0..1', () => {
    const before = frame(-500);
    expect(before.bars).toEqual([0, 0, 0, 0]);
    expect(before.ring).toBe(0);
    expect(before.portrait).toBe(0);
    expect(before.settle).toBe(0);

    const after = frame(TOTAL_MS + 10_000);
    expect(after.bars).toEqual([1, 1, 1, 1]);
    expect(after.ring).toBe(1);
    expect(after.portrait).toBe(1);
    expect(after.settle).toBe(1);

    const nan = frame(Number.NaN);
    for (const v of [...nan.bars, nan.wipe, nan.wipeAlpha, nan.whole, nan.ring, nan.portrait, nan.settle]) {
      expect(Number.isNaN(v)).toBe(false);
      expect(v).toBeGreaterThanOrEqual(0);
      expect(v).toBeLessThanOrEqual(1);
    }
  });
});

// krug popravka 1 (recenzija): splash mora javiti `splash:done` TOČNO JEDNOM što god se dogodilo
// (kraj animacije, klik, tipka, greška učitavanja slike) — `once` je zajednička brava za to.
describe('once() — brava protiv dvostrukog poziva', () => {
  it('poziva zamotanu funkciju samo prvi put', () => {
    let calls = 0;
    const fn = once(() => {
      calls += 1;
    });
    fn();
    fn();
    fn();
    expect(calls).toBe(1);
  });
});
