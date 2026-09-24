// ZAŠTO OVAKO (cigla M2/25 — ugovor Report vezan na snapshot jezgre)
// `?raw` je isti uvoz koji koristi `api.ts` (S-010, jedan izvor čitanja) — `node:fs` ovdje ne postoji
// jer sučelje nema `@types/node` i nikakav `node:*` uvoz u `.ts` ne prolazi kroz `svelte-check`
// (odluka orkestratora za ovu ciglu; `.snap` nije `.css`, pa Tailwind-plugin ne guta ovaj `?raw` uvoz).
import { describe, expect, it } from 'vitest';
import snapRaw from '../../../crates/sokratis-core/tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap?raw';
import { parseSnap } from '../src/lib/api';
import type { Report } from '../src/lib/types';

describe('ugovor Report', () => {
  it('snapshot iz jezgre ima točno ključeve koje TS tip očekuje', () => {
    const r = parseSnap(snapRaw) as Report;
    expect(Object.keys(r).sort()).toEqual([
      'branch',
      'branches',
      'commits',
      'days',
      'deliveries',
      'docs',
      'generated_at',
      'indicators',
      'kinds',
      'phases',
      'scope',
      'signals',
      'since',
      'touched',
      'until',
      'vision_totals',
      'visions',
    ]);
    expect(r.indicators).toHaveLength(18);
    expect(r.days[0]).toHaveProperty('commits_cumulative');
    expect(r.commits.length > 0 ? r.commits[0] : { kind: 'execution', sub: 'other', overridden: false }).toMatchObject({
      overridden: expect.any(Boolean),
    });
    expect(r.scope).toBe('default');
    expect(r.branches[0]).toMatchObject({ name: 'main', merged: true });
  });
});
