// Dopunjeno M2/27 (Ruling orkestratora #7): pogled Pokazatelji zove `t('ind.' + id)` za svih 18
// pokazatelja koje jezgra stvarno vraća — ovaj test čita te id-jeve IZ PRAVE snimke jezgre (isti
// `?raw` uvoz kao `tests/types.test.ts`, S-010), da nedostajući ključ ne padne tiho na natpisu.
import { describe, expect, it } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import en from '../src/lib/i18n/en.json';
import { translate } from '../src/lib/i18n/t';
import snapRaw from '../../../crates/sokratis-core/tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap?raw';
import { parseSnap } from '../src/lib/api';
import type { Report } from '../src/lib/types';

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
  it('svih 18 id-jeva pokazatelja iz snimke jezgre ima natpis u OBA rječnika (nije sam ključ)', () => {
    const report = parseSnap(snapRaw) as Report;
    expect(report.indicators.length).toBe(18);
    for (const ind of report.indicators) {
      const key = `ind.${ind.id}`;
      expect(translate(hr, key), key).not.toBe(key);
      expect(translate(en, key), key).not.toBe(key);
    }
  });
});
