import { describe, expect, it } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import en from '../src/lib/i18n/en.json';
import { translate } from '../src/lib/i18n/t';

describe('i18n', () => {
  it('hr i en imaju iste ključeve', () => {
    expect(Object.keys(hr).sort()).toEqual(Object.keys(en).sort());
  });
  it('svaki natpis je neprazan i nosi iste {parametre} u oba jezika', () => {
    const params = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort();
    for (const k of Object.keys(hr)) {
      expect((hr as Record<string, string>)[k]!.trim().length, k).toBeGreaterThan(0);
      expect(params((hr as Record<string, string>)[k]!), k).toEqual(params((en as Record<string, string>)[k]!));
    }
  });
  it('translate zamjenjuje parametre i ne skriva nepoznat ključ', () => {
    expect(translate({ 'x.y': 'prije {n} h' }, 'x.y', { n: 2 })).toBe('prije 2 h');
    expect(translate({}, 'nema.toga')).toBe('nema.toga');
  });
});
