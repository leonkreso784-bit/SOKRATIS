// ZAŠTO OVAKO (cigla M2/54 — raspored grafova kao čiste funkcije, S-035)
// Nema DOM-a u vitestu, pa SVE što se može pogrešno izračunati (koordinate, ticksovi, stack, najbliža
// točka) živi ovdje i ima test; Svelte komponente samo mapiraju ovaj izlaz u `<rect>`/`<path>`.
// `scaleBand` (d3-scale) dijeli širinu na jednake trake s razmakom — isti obrazac za dane, tjedne i
// mjesece. `stack` iz d3-shape nije potreban: naslagani stupci su jedno zbrajanje po nizu.
// Dopunjeno M2/56 — `heatmapLayout` mapira dan u (stupac = ISO tjedan, red = dan u tjednu) pa razinu
// boje svodi LINEARNO na udio vrijednosti dana u najvećoj vrijednosti prikazanog razdoblja, u 4
// koraka (`heatLevel`, ispravljeno M2/61 — ranija rečenica ovdje je krivo tvrdila „kvantil");
// `histogramLayout` je `scaleBand` nad 24 sata, isti obrazac kao stupci iznad, samo bez datuma.
// Dopunjeno M2/57 — `ganttLayout` je vremenska skala (kao `lineLayout`) nad trakama faza umjesto
// točaka, s „danas" kao dodatnom okomicom; `hbarsLayout` je vodoravni stupac po grani, `linear` bez
// `scaleBand` jer redovi nisu jednako razmaknuti kategorije nego već poredan popis (grane su
// poredane u `Report`, ovdje se samo crta); `bucketTotal` (R55) izdvaja zbroj/najveću vrijednost
// jednog dana iz `barsLayout`, da `map`+`reduce` ne žive naslagani u jednom izrazu.
// Dopunjeno M2/62 — tri nalaza dimnog testa ploče: (G1) `fitLabel` krati predug natpis retka (≈20
// znakova stane u lijevu marginu 140px uz font-size 10) s „…" na kraju, PUNI naziv ostaje u
// `label`/`<title>`; (G2) `withEdgeAnchor` prebacuje `text-anchor` zadnjeg X-ticka na 'end' kad bi mu
// centriran natpis izašao izvan `viewBox`-a; (G3) `dayLevelTicks` (poziva ga `dateTicks` u
// `scales.ts`) ne pušta d3 ispod razine dana kad je format dnevni, pa se oznaka ne ponavlja na
// jednodnevnom/dvodnevnom rasponu. Sve tri su ČISTE funkcije s testom (R37) — isto pravilo kao gore.
import { scaleBand } from 'd3-scale';
import { dateTicks, finiteMax, formatDate, formatMonth, formatWeekday, linear, niceMax, parseYmd, timeScale, toYmd, yTicks } from './scales';
import { bucketStart, type Granularity } from './bucket';
import type { Lang, Phase, PhaseState } from '../types';

export interface Margins { top: number; right: number; bottom: number; left: number }
export interface Frame { width: number; height: number; m: Margins; innerW: number; innerH: number }
// `anchor` je NEOBVEZAN (T62/G2): zadano `'middle'` (centriran natpis) kad polje nedostaje, komponenta
// (`Axis.svelte`) čita `t.anchor ?? 'middle'`. Postavlja ga SAMO `withEdgeAnchor` niže, za tick koji bi
// centriran izašao izvan `viewBox`-a.
export interface XTick { x: number; label: string; anchor?: 'middle' | 'end' }
export interface YTick { y: number; label: string }
export interface Series { id: string; label: string; color: string }
export interface Bar { x: number; y: number; w: number; h: number; series: string; value: number; start: string }
export interface BarsLayout { bars: Bar[]; xTicks: XTick[]; yTicks: YTick[]; baseline: number; centers: number[] }
export interface LinePoint { x: number; y: number; value: number; date: string }
export interface LineLayout { paths: { series: string; d: string; points: LinePoint[] }[]; xTicks: XTick[]; yTicks: YTick[] }
export interface HeatCell { x: number; y: number; w: number; h: number; date: string; value: number; level: 0 | 1 | 2 | 3 | 4 }
export interface HeatmapLayout { cells: HeatCell[]; monthLabels: XTick[]; weekdayLabels: YTick[]; cell: number }
export interface HistogramLayout { bars: Bar[]; xTicks: XTick[]; yTicks: YTick[] }
// `label` ostaje PUN naziv (koristi ga `<title>` reda u komponenti, T62/G1); `shortLabel` je isti
// naziv skraćen za lijevu marginu (`fitLabel` niže) — komponenta CRTA `shortLabel`, ne `label`.
export interface GanttRow { y: number; h: number; x0: number; x1: number; label: string; shortLabel: string; state: PhaseState; from: string | null; to: string | null }
export interface GanttLayout { rows: GanttRow[]; xTicks: XTick[]; todayX: number | null; rowH: number }
export interface HBar { y: number; h: number; x: number; w: number; label: string; shortLabel: string; value: number; merged: boolean }
export interface HBarsLayout { bars: HBar[]; xTicks: XTick[]; rowH: number }

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

// T62 (G1+G2) — procjena širine znaka za `font-size="10"` (isti font-size kao `<text>` u
// Axis/Gantt/HBars). Vitest nema DOM pa se stvarna širina teksta ne mjeri; 0,6 × veličina fonta po
// znaku je uobičajena gruba procjena za proporcionalno sans-serif pismo (radije prerano skratimo ili
// pomaknemo natpis nego da ga ostavimo odrezanog izvan viewBoxa).
const CHAR_WIDTH_AT_10PX = 6;
function estimateLabelWidth(label: string): number {
  return label.length * CHAR_WIDTH_AT_10PX;
}

// G1 — natpis retka (Gantt/HBars) je desno poravnat na `x = f.m.left - 6` (6 = razmak od margine,
// isti broj kao u komponenti); granica u ZNAKOVIMA je raspoloživa širina (margina minus taj razmak)
// podijeljena prosjekom širine znaka gore. Za marginu 140 (stvarna vrijednost u obje komponente) to
// je (140 − 6) / 6 ≈ 22 znaka — dovoljno da natpisi do ≈ 20 znakova (nalaz G1) stanu nepromijenjeni.
function maxRowLabelChars(marginLeft: number): number {
  const ROW_LABEL_GAP = 6;
  return Math.max(1, Math.floor((marginLeft - ROW_LABEL_GAP) / CHAR_WIDTH_AT_10PX));
}

// G1 — čista funkcija (bez DOM-a, testirana izravno u `layout.test.ts`): tekst dulji od granice se
// skrati i završi znakom „…" (jedan znak, ne tri točke) tako da UKUPNA duljina ostane ≤ granice.
export function fitLabel(text: string, maxChars: number): string {
  if (text.length <= maxChars) return text;
  if (maxChars <= 1) return '…';
  return `${text.slice(0, maxChars - 1)}…`;
}

// G2 — zadnji X-tick (uvijek na desnom rubu okvira, T62/G3) bi centriran (`text-anchor="middle"`)
// desnom polovicom izašao izvan `viewBox`-a jer margina (`m.right`) ostavlja manje mjesta nego pola
// procijenjene širine natpisa. `anchor: 'end'` pomakne tekst tako da mu DESNI rub sjedi na ticku —
// ostaje unutar `viewBox`-a bez pomicanja same koordinate ticka.
function withEdgeAnchor(ticks: XTick[], viewBoxRight: number): XTick[] {
  return ticks.map((t) => {
    const half = estimateLabelWidth(t.label) / 2;
    return viewBoxRight - t.x < half ? { ...t, anchor: 'end' as const } : t;
  });
}

// R55 (deferred minor T54) — bio je ternary + `reduce` unutar `map` u `barsLayout`; imenovana
// funkcija čita se kao rečenica ("zbroj dana" ili "najveća vrijednost dana"), a `map` niže samo
// poziva. Ponašanje nepromijenjeno — dokaz su postojeći `barsLayout` testovi iznad.
function bucketTotal(b: { values: Record<string, number> }, series: Series[], stacked: boolean): number {
  if (stacked) return series.reduce((sum, sr) => sum + (b.values[sr.id] ?? 0), 0);
  return Math.max(0, ...series.map((sr) => b.values[sr.id] ?? 0));
}

export function barsLayout(f: Frame, buckets: { start: string; values: Record<string, number> }[], series: Series[], g: Granularity, lang: Lang, stacked = false): BarsLayout {
  const starts = buckets.map((b) => b.start);
  const x = scaleBand<string>().domain(starts).range([f.m.left, f.m.left + f.innerW]).paddingInner(0.2).paddingOuter(0.1);
  const totals = buckets.map((b) => bucketTotal(b, series, stacked));
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
  // T62 (G3) — domena ovdje NE širi umjetno kraj na idući dan kad niz ima jednu točku: `scaleUtc` s
  // domenom širine nula ostaje konačan (mapira na sredinu raspona, provjereno testom), a `dateTicks`
  // za takvu domenu vraća točno jednu oznaku. Ranije širenje je davalo tick za dan koji ne postoji U
  // PODACIMA i (kroz `dateTicks`) ticksove ispod dana s ponovljenom oznakom (isti uzrok kao G3).
  const domain: [Date, Date] = [parseYmd(first), parseYmd(last)];
  const x = timeScale(domain, [f.m.left, f.m.left + f.innerW]);
  const max = Math.max(0, ...all.map((p) => p.value).filter(Number.isFinite));
  const y = yScale(f, max);
  const paths = series.map((s) => {
    const points = [...s.points].sort((a, b) => (a.date < b.date ? -1 : 1)).map((p) => ({ x: x(parseYmd(p.date)), y: y(p.value), value: p.value, date: p.date }));
    return { series: s.id, d: linePath(points), points };
  });
  const rawTicks = dateTicks(domain, f.innerW, lang).map((t) => ({ x: f.m.left + t.x, label: t.label }));
  const xTicks = withEdgeAnchor(rawTicks, f.width); // G2: zadnji tick ostaje unutar viewBoxa
  return { paths, xTicks, yTicks: yTicksFor(f, max) };
}

function heatLevel(value: number, max: number): 0 | 1 | 2 | 3 | 4 {
  if (value <= 0 || max <= 0) return 0;
  return Math.min(4, 1 + Math.floor((3 * value) / max)) as 0 | 1 | 2 | 3 | 4;
}

// Dopunjeno M2/56 — kalendar po ISO tjednima (stupac = tjedan, red = dan u tjednu 0…6) i histogram
// doba dana (24 bina). Obje funkcije ostaju čiste kao gornje: primaju već izmjerene dane/brojeve i
// vraćaju koordinate, isto pravilo koje je već vodilo `barsLayout`/`lineLayout` (bez DOM-a, R37).
export function heatmapLayout(f: Frame, days: { date: string; value: number }[], range: [string, string], lang: Lang): HeatmapLayout {
  const byDate = new Map(days.map((d) => [d.date, d.value]));
  const max = finiteMax(days.map((d) => d.value));
  const startWeek = parseYmd(bucketStart(range[0], 'week'));
  const endWeek = parseYmd(bucketStart(range[1], 'week'));
  const weeks = Math.max(1, Math.round((endWeek.getTime() - startWeek.getTime()) / (7 * 86_400_000)) + 1);
  const cell = Math.min(f.innerW / weeks, f.innerH / 7);
  const cells: HeatCell[] = [];
  const monthLabels: XTick[] = [];
  let prevMonth = '';
  for (let i = 0; i < weeks * 7; i++) {
    const date = new Date(startWeek.getTime() + i * 86_400_000);
    const ymd = toYmd(date);
    const col = Math.floor(i / 7);
    const row = (date.getUTCDay() + 6) % 7;
    if (row === 0) {
      // Prvi red svakog stupca (ponedjeljak) odlučuje otvara li stupac novi mjesec — provjera SAMO
      // ovdje, ne za svaki od 7 dana u stupcu, jer bi inače isti mjesec dobio do 7 uzastopnih natpisa.
      const month = ymd.slice(0, 7);
      if (month !== prevMonth) {
        monthLabels.push({ x: f.m.left + col * cell + cell / 2, label: formatMonth(ymd, lang) });
        prevMonth = month;
      }
    }
    const value = byDate.get(ymd) ?? 0;
    cells.push({ x: f.m.left + col * cell, y: f.m.top + row * cell, w: cell, h: cell, date: ymd, value, level: heatLevel(value, max) });
  }
  const weekdayLabels: YTick[] = [0, 2, 4].map((row) => ({ y: f.m.top + row * cell + cell / 2, label: formatWeekday(row, lang) }));
  return { cells, monthLabels, weekdayLabels, cell };
}

export function histogramLayout(f: Frame, counts: number[]): HistogramLayout {
  const hours = Array.from({ length: 24 }, (_, h) => String(h));
  const x = scaleBand<string>().domain(hours).range([f.m.left, f.m.left + f.innerW]).paddingInner(0.2).paddingOuter(0.1);
  const max = finiteMax(counts);
  const y = yScale(f, max);
  const baseline = y(0);
  const bw = x.bandwidth();
  const bars: Bar[] = hours.map((h, i) => {
    // `counts[i]` je `number | undefined` (noUncheckedIndexedAccess) iako je `i` unutar 0..23.
    const v = counts[i] ?? 0;
    const x0 = x(h) ?? f.m.left;
    return { x: x0, y: y(v), w: bw, h: baseline - y(v), series: 'commits', value: v, start: h };
  });
  const xTicks: XTick[] = [0, 6, 12, 18].map((h) => ({ x: (x(String(h)) ?? 0) + bw / 2, label: `${h} h` }));
  return { bars, xTicks, yTicks: yTicksFor(f, max) };
}

function hasFrom(p: Phase): p is Phase & { from: string } {
  return p.from !== null;
}

export function ganttLayout(f: Frame, phases: Phase[], today: string, lang: Lang): GanttLayout {
  const rowH = f.innerH / Math.max(1, phases.length);
  if (phases.length === 0) return { rows: [], xTicks: [], todayX: null, rowH };
  const withFrom = phases.filter(hasFrom);
  let domain: [Date, Date];
  if (withFrom.length === 0) {
    const t = parseYmd(today);
    domain = [t, new Date(t.getTime() + 86_400_000)];
  } else {
    const min = Math.min(...withFrom.map((p) => parseYmd(p.from).getTime()));
    const max = Math.max(...withFrom.map((p) => parseYmd(p.to ?? today).getTime()));
    domain = [new Date(min), new Date(max === min ? min + 86_400_000 : max)];
  }
  const x = timeScale(domain, [f.m.left, f.m.left + f.innerW]);
  // G1 — granica u znakovima ovisi SAMO o lijevoj margini OVOG grafa (`f.m.left`), ista formula kao
  // `HBars` niže: obje komponente crtaju natpis u istom razmaku od margine i istim font-size 10.
  const maxChars = maxRowLabelChars(f.m.left);
  const rows: GanttRow[] = phases.map((p, i) => {
    const y = f.m.top + i * rowH + rowH * 0.2;
    const h = rowH * 0.6;
    const shortLabel = fitLabel(p.name, maxChars);
    // Faza bez `from` nema poznatu točku na vremenskoj osi — traka se svodi na jednu točku uz rub
    // (crta je nula širine), natpis stanja dodaje komponenta (R60).
    if (!hasFrom(p)) return { y, h, x0: f.m.left, x1: f.m.left, label: p.name, shortLabel, state: p.state, from: p.from, to: p.to };
    const x0 = x(parseYmd(p.from));
    const toYmdVal = p.to ?? (p.state === 'running' ? today : p.from);
    const x1 = x(parseYmd(toYmdVal));
    return { y, h, x0, x1, label: p.name, shortLabel, state: p.state, from: p.from, to: p.to };
  });
  const todayMs = parseYmd(today).getTime();
  const withinDomain = todayMs >= domain[0].getTime() && todayMs <= domain[1].getTime();
  const todayX = withinDomain ? x(parseYmd(today)) : null;
  const rawTicks = dateTicks(domain, f.innerW, lang).map((t) => ({ x: f.m.left + t.x, label: t.label }));
  const xTicks = withEdgeAnchor(rawTicks, f.width); // G2: zadnji tick ostaje unutar viewBoxa
  return { rows, xTicks, todayX, rowH };
}

export function hbarsLayout(f: Frame, items: { label: string; value: number; merged: boolean }[]): HBarsLayout {
  const rowH = f.innerH / Math.max(1, items.length);
  const max = finiteMax(items.map((it) => it.value));
  const x = linear([0, niceMax(max)], [f.m.left, f.m.left + f.innerW]);
  const maxChars = maxRowLabelChars(f.m.left); // G1 — ista formula kao `ganttLayout` iznad
  const bars: HBar[] = items.map((it, i) => ({
    y: f.m.top + i * rowH + rowH * 0.2,
    h: rowH * 0.6,
    x: f.m.left,
    w: x(it.value) - f.m.left,
    label: it.label,
    shortLabel: fitLabel(it.label, maxChars),
    value: it.value,
    merged: it.merged,
  }));
  const xTicks: XTick[] = yTicks(max).map((v) => ({ x: x(v), label: String(v) }));
  return { bars, xTicks, rowH };
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
