// ZAŠTO OVAKO (cigla M2/41): popis objašnjivih id-eva živi na JEDNOM mjestu (`ids.ts`), a test ga veže
// na oba rječnika u oba smjera — id bez teksta pada, tekst bez id-a (siroče) pada. Id-evi pokazatelja se
// čitaju iz PRAVE snimke jezgre, kao u `i18n.test.ts`, da novi pokazatelj u jezgri ne ostane bez objašnjenja.
import { describe, expect, it } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import en from '../src/lib/i18n/en.json';
import { EXPLAIN_IDS, explainKeys } from '../src/lib/explain/ids';
import snapRaw from '../../../crates/sokratis-core/tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap?raw';
import { parseSnap } from '../src/lib/api';
import type { Report } from '../src/lib/types';

const dicts: Record<string, Record<string, string>> = { hr, en };
const UI_KEYS = new Set(['explain.title.what', 'explain.title.how', 'explain.title.read', 'explain.close', 'explain.formula']);

describe('kartica s objašnjenjem', () => {
  it('svaki objašnjivi id ima što · kako · čitanje u oba jezika', () => {
    for (const id of EXPLAIN_IDS) {
      for (const key of Object.values(explainKeys(id))) {
        for (const [lang, d] of Object.entries(dicts)) {
          expect((d[key] ?? '').trim().length, `${lang}:${key}`).toBeGreaterThan(0);
        }
      }
    }
  });
  it('nijedan ključ explain.* nije siroče', () => {
    const expected = new Set(EXPLAIN_IDS.flatMap((id) => Object.values(explainKeys(id))));
    for (const key of Object.keys(hr).filter((k) => k.startsWith('explain.'))) {
      expect(expected.has(key) || UI_KEYS.has(key), key).toBe(true);
    }
  });
  it('svih 18 pokazatelja iz snimke jezgre je objašnjivo', () => {
    const report = parseSnap(snapRaw) as Report;
    expect(report.indicators.length).toBe(18);
    for (const ind of report.indicators) expect(EXPLAIN_IDS, ind.id).toContain(`ind.${ind.id}`);
  });
  it('popis nema duplikata', () => {
    expect(new Set(EXPLAIN_IDS).size).toBe(EXPLAIN_IDS.length);
  });
});
