// ZAŠTO OVAKO (cigla M2/53 — dan/tjedan/mjesec je preslagivanje već izmjerenih dana, ne mjerenje;
// dopuna S-012). Sve funkcije su čiste; rubovi: prazan ulaz, prijelaz godine, ISO tjedan koji
// počinje u prošloj godini, sat u lokalnoj zoni.
import { describe, expect, it } from 'vitest';
import { bucketDays, bucketDeliveries, bucketKinds, bucketStart, hourHistogram, hourOfDay } from '../src/lib/charts/bucket';
import type { CommitRow, DayStats, Delivery } from '../src/lib/types';

const day = (date: string, commits: number, hours = 1): DayStats => ({
  date, commits, commits_cumulative: 0, lines: 10, hours, deliveries: 0, deploys: 0, test_lines: 0,
});

// R42 (dopuna T53): `CommitRow` u ovom stablu još nema `branch` (dodaje ga T46, drugi tok).
// Base-objekt + spread umjesto literala s `: CommitRow` anotacijom prolazi svelte-check i prije i
// poslije spajanja T46 — TypeScript ne provjerava višak svojstava kad dolaze kroz spread.
const baseRow = {
  sha: '', date: '', author_time: 0, subject: '', kind: 'execution', sub: 'other', overridden: false, branch: 'main',
} as const;
const row = (date: string, kind: CommitRow['kind'], author_time = 0): CommitRow => ({
  ...baseRow, sha: date + kind, date, kind, author_time,
});

describe('bucketStart', () => {
  it('dan je sam sebi početak', () => expect(bucketStart('2026-09-10', 'day')).toBe('2026-09-10'));
  it('tjedan počinje ISO ponedjeljkom, i preko granice godine', () => {
    expect(bucketStart('2026-09-10', 'week')).toBe('2026-09-07'); // četvrtak → ponedjeljak
    expect(bucketStart('2026-09-07', 'week')).toBe('2026-09-07');
    expect(bucketStart('2026-09-13', 'week')).toBe('2026-09-07'); // nedjelja → isti tjedan
    expect(bucketStart('2027-01-01', 'week')).toBe('2026-12-28'); // petak → ponedjeljak prošle godine
  });
  it('mjesec počinje prvim', () => expect(bucketStart('2026-09-10', 'month')).toBe('2026-09-01'));
});

describe('bucketDays', () => {
  it('vraća [] za prazan ulaz', () => expect(bucketDays([], 'week')).toEqual([]));
  it('zbraja dane u tjedne uzlazno', () => {
    const out = bucketDays([day('2026-09-13', 2, 1.5), day('2026-09-07', 1, 0.5), day('2026-09-14', 3)], 'week');
    expect(out.map((b) => [b.start, b.commits, b.hours])).toEqual([
      ['2026-09-07', 3, 2],
      ['2026-09-14', 3, 1],
    ]);
  });
  it('dan = identitet po datumu', () => {
    expect(bucketDays([day('2026-09-10', 2)], 'day')[0]).toMatchObject({ start: '2026-09-10', commits: 2 });
  });
});

describe('bucketKinds / bucketDeliveries', () => {
  it('broji vrste po tjednu sa svih pet ključeva', () => {
    const out = bucketKinds([row('2026-09-10', 'execution'), row('2026-09-11', 'debugging'), row('2026-09-15', 'execution')], 'week');
    expect(out).toHaveLength(2);
    expect(out[0]?.counts).toEqual({ planning: 0, documentation: 0, execution: 1, polish: 0, debugging: 1 });
  });
  it('broji isporuke i deploye po tjednu', () => {
    const d = (date: string, deploy: boolean): Delivery => ({ date, model: '', title: 't', kind: 'execution', deploy });
    expect(bucketDeliveries([d('2026-09-10', true), d('2026-09-11', false)], 'week')).toEqual([
      { start: '2026-09-07', count: 2, deploys: 1 },
    ]);
  });
});

describe('doba dana', () => {
  it('sat je u lokalnoj zoni računala i histogram ima 24 bina', () => {
    const t = Date.UTC(2026, 8, 10, 12, 0, 0) / 1000; // podne UTC
    expect(hourOfDay(t)).toBe(new Date(t * 1000).getHours());
    const h = hourHistogram([row('2026-09-10', 'execution', t)]);
    expect(h).toHaveLength(24);
    expect(h.reduce((a, b) => a + b, 0)).toBe(1);
  });
});
