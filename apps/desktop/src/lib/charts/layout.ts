// ZAŠTO OVAKO (cigla M2/54 — raspored grafova kao čiste funkcije, S-035)
// Nema DOM-a u vitestu, pa SVE što se može pogrešno izračunati (koordinate, ticksovi, stack, najbliža
// točka) živi ovdje i ima test; Svelte komponente samo mapiraju ovaj izlaz u `<rect>`/`<path>`.
// `scaleBand` (d3-scale) dijeli širinu na jednake trake s razmakom — isti obrazac za dane, tjedne i
// mjesece. `stack` iz d3-shape nije potreban: naslagani stupci su jedno zbrajanje po nizu.
import { scaleBand } from 'd3-scale';
import { dateTicks, formatDate, niceMax, parseYmd, timeScale, yTicks } from './scales';
import type { Granularity } from './bucket';
import type { Lang } from '../types';

export interface Margins { top: number; right: number; bottom: number; left: number }
export interface Frame { width: number; height: number; m: Margins; innerW: number; innerH: number }
export interface XTick { x: number; label: string }
export interface YTick { y: number; label: string }
export interface Series { id: string; label: string; color: string }
export interface Bar { x: number; y: number; w: number; h: number; series: string; value: number; start: string }
export interface BarsLayout { bars: Bar[]; xTicks: XTick[]; yTicks: YTick[]; baseline: number; centers: number[] }
export interface LinePoint { x: number; y: number; value: number; date: string }
export interface LineLayout { paths: { series: string; d: string; points: LinePoint[] }[]; xTicks: XTick[]; yTicks: YTick[] }

const M: Margins = { top: 12, right: 12, bottom: 28, left: 40 };

export function frame(width: number, height: number, m: Partial<Margins> = {}): Frame {
  const mm = { ...M, ...m };
  return { width, height, m: mm, innerW: Math.max(0, width - mm.left - mm.right), innerH: Math.max(0, height - mm.top - mm.bottom) };
}

function yScale(f: Frame, max: number): (v: number) => number {
  const top = niceMax(max);
  return (v) => f.m.top + f.innerH - (v / top) * f.innerH;
}
function yTicksFor(f: Frame, max: number): YTick[] {
  const y = yScale(f, max);
  return yTicks(max).map((v) => ({ y: y(v), label: String(v) }));
}

export function barsLayout(f: Frame, buckets: { start: string; values: Record<string, number> }[], series: Series[], g: Granularity, lang: Lang, stacked = false): BarsLayout {
  const starts = buckets.map((b) => b.start);
  const x = scaleBand<string>().domain(starts).range([f.m.left, f.m.left + f.innerW]).paddingInner(0.2).paddingOuter(0.1);
  const totals = buckets.map((b) => stacked ? series.reduce((s, sr) => s + (b.values[sr.id] ?? 0), 0) : Math.max(0, ...series.map((sr) => b.values[sr.id] ?? 0)));
  const max = Math.max(0, ...totals.filter(Number.isFinite));
  const y = yScale(f, max);
  const baseline = y(0);
  const bw = x.bandwidth();
  const bars: Bar[] = [];
  for (const b of buckets) {
    const x0 = x(b.start) ?? f.m.left;
    let acc = 0;
    series.forEach((sr, i) => {
      // `b.values[sr.id]` je `number | undefined` (noUncheckedIndexedAccess); `Number.isFinite`
      // provjerava vrijednost, ne sužava TYPE, pa najprije uzmemo siguran broj pa tek onda provjerimo.
      const raw = b.values[sr.id] ?? 0;
      const v = Number.isFinite(raw) ? raw : 0;
      if (stacked) {
        bars.push({ x: x0, y: y(acc + v), w: bw, h: baseline - y(v), series: sr.id, value: v, start: b.start });
        acc += v;
      } else {
        const w = bw / series.length;
        bars.push({ x: x0 + i * w, y: y(v), w, h: baseline - y(v), series: sr.id, value: v, start: b.start });
      }
    });
  }
  const every = Math.max(1, Math.ceil(starts.length / 10));
  const xTicks = starts.filter((_, i) => i % every === 0).map((s) => ({ x: (x(s) ?? 0) + bw / 2, label: formatDate(s, lang, g) }));
  return { bars, xTicks, yTicks: yTicksFor(f, max), baseline, centers: starts.map((s) => (x(s) ?? 0) + bw / 2) };
}

export function linePath(pts: { x: number; y: number }[]): string {
  return pts.filter((p) => Number.isFinite(p.x) && Number.isFinite(p.y)).map((p, i) => `${i ? 'L' : 'M'}${+p.x.toFixed(2)} ${+p.y.toFixed(2)}`).join(' ');
}

export function lineLayout(f: Frame, series: (Series & { points: { date: string; value: number }[] })[], lang: Lang): LineLayout {
  const all = series.flatMap((s) => s.points);
  if (all.length === 0) return { paths: [], xTicks: [], yTicks: yTicksFor(f, 0) };
  const dates = all.map((p) => p.date).sort();
  // Niz nije prazan (provjereno iznad), ali `noUncheckedIndexedAccess` to ne zna — `first`/`last` uz
  // `??` je čitljivije od `!` i i dalje ne mijenja ponašanje (isti obrazac kao `bucket.ts`/T53).
  const first = dates[0] ?? '';
  const last = dates.at(-1) ?? first;
  const domain: [Date, Date] = [parseYmd(first), parseYmd(last)];
  if (domain[0].getTime() === domain[1].getTime()) domain[1] = new Date(domain[1].getTime() + 86_400_000);
  const x = timeScale(domain, [f.m.left, f.m.left + f.innerW]);
  const max = Math.max(0, ...all.map((p) => p.value).filter(Number.isFinite));
  const y = yScale(f, max);
  const paths = series.map((s) => {
    const points = [...s.points].sort((a, b) => (a.date < b.date ? -1 : 1)).map((p) => ({ x: x(parseYmd(p.date)), y: y(p.value), value: p.value, date: p.date }));
    return { series: s.id, d: linePath(points), points };
  });
  const xTicks = dateTicks(domain, f.innerW, lang).map((t) => ({ x: f.m.left + t.x, label: t.label }));
  return { paths, xTicks, yTicks: yTicksFor(f, max) };
}

export function nearestIndex(xs: number[], x: number): number {
  let best = -1;
  let dist = Number.POSITIVE_INFINITY;
  xs.forEach((v, i) => {
    const d = Math.abs(v - x);
    if (d < dist) {
      dist = d;
      best = i;
    }
  });
  return best;
}
