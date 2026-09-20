// ZAŠTO OVAKO (cigla M2/25 — MockApi daje jedan projekt iz snimke, override se pamti)
// Test ide kroz javno sučelje `Api`, ne kroz privatna polja `MockApi` — isto što bi vidio Pregled
// i Dnevnik u pregledniku: popis projekata, pa `Report`, pa promjena vrste jednom commitu.
import { describe, expect, it } from 'vitest';
import { MockApi } from '../src/lib/api';

describe('MockApi', () => {
  it('daje jedan projekt i izvještaj iz snapshota; override se pamti', async () => {
    const api = new MockApi();
    const ps = await api.listProjects();
    expect(ps).toHaveLength(1);
    const r = await api.getReport(ps[0]!.id, { preset: 'all' });
    expect(r.touched.commits).toBe(190);
    if (r.commits.length) {
      await api.setOverride(ps[0]!.id, r.commits[0]!.sha, 'polish');
      const again = await api.getReport(ps[0]!.id, { preset: 'all' });
      expect(again.commits[0]).toMatchObject({ kind: 'polish', overridden: true });
    }
  });
});
