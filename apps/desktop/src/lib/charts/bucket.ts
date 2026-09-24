// ZAŠTO OVAKO (cigla M2/53 — preslagivanje dana u tjedne/mjesece; dopuna S-012)
// Ulaz su VEĆ izmjereni `DayStats`/`CommitRow`/`Delivery` iz jezgre; ovdje se samo zbrajaju po ključu
// razdoblja. ISO tjedan računamo sami nad UTC datumom (ponedjeljak = početak) — čista aritmetika,
// bez `d3-time`. Sat u danu je u ZONI OVOG RAČUNALA (jezgra ne nosi sat autora) — kartica to kaže.
import { parseYmd, toYmd } from './scales';
import type { CommitRow, DayStats, Delivery, WorkKind } from '../types';

export type Granularity = 'day' | 'week' | 'month';
export interface Bucket { start: string; commits: number; hours: number; lines: number; deliveries: number; deploys: number; test_lines: number }

const KINDS: WorkKind[] = ['planning', 'documentation', 'execution', 'polish', 'debugging'];

export function bucketStart(ymd: string, g: Granularity): string {
  if (g === 'day') return ymd;
  const d = parseYmd(ymd);
  if (g === 'month') return ymd.slice(0, 7) + '-01';
  d.setUTCDate(d.getUTCDate() - ((d.getUTCDay() + 6) % 7));
  return toYmd(d);
}

// `R extends { start: string }` (dopuna T53) umjesto casta u sortu — generički akumulator svakog
// bucketa uvijek nosi svoj početni datum, pa se po njemu može sortirati bez `as`.
function groupBy<T, R extends { start: string }>(
  rows: T[],
  g: Granularity,
  date: (r: T) => string,
  empty: (start: string) => R,
  add: (acc: R, r: T) => void,
): R[] {
  const map = new Map<string, R>();
  for (const r of rows) {
    const start = bucketStart(date(r), g);
    let acc = map.get(start);
    if (!acc) {
      acc = empty(start);
      map.set(start, acc);
    }
    add(acc, r);
  }
  return [...map.values()].sort((a, b) => (a.start < b.start ? -1 : 1));
}

export function bucketDays(days: DayStats[], g: Granularity): Bucket[] {
  return groupBy(
    days,
    g,
    (d) => d.date,
    (start) => ({ start, commits: 0, hours: 0, lines: 0, deliveries: 0, deploys: 0, test_lines: 0 }),
    (b, d) => {
      b.commits += d.commits;
      b.hours += d.hours;
      b.lines += d.lines;
      b.deliveries += d.deliveries;
      b.deploys += d.deploys;
      b.test_lines += d.test_lines;
    },
  );
}
export function bucketKinds(rows: CommitRow[], g: Granularity): { start: string; counts: Record<WorkKind, number> }[] {
  return groupBy(
    rows,
    g,
    (r) => r.date,
    (start) => ({ start, counts: Object.fromEntries(KINDS.map((k) => [k, 0])) as Record<WorkKind, number> }),
    (b, r) => {
      b.counts[r.kind] += 1;
    },
  );
}
export function bucketDeliveries(rows: Delivery[], g: Granularity): { start: string; count: number; deploys: number }[] {
  return groupBy(
    rows,
    g,
    (r) => r.date,
    (start) => ({ start, count: 0, deploys: 0 }),
    (b, r) => {
      b.count += 1;
      if (r.deploy) b.deploys += 1;
    },
  );
}
export function hourOfDay(authorTime: number): number {
  return new Date(authorTime * 1000).getHours();
}
export function hourHistogram(rows: CommitRow[]): number[] {
  const h = Array.from({ length: 24 }, () => 0);
  for (const r of rows) {
    const i = hourOfDay(r.author_time);
    h[i] = (h[i] ?? 0) + 1;
  }
  return h;
}
