// ZAŠTO OVAKO (cigla M2/53 — d3-matematika bez DOM-a, S-035)
// `d3-scale` daje `nice()`, `ticks()` i datumske ticksove koji znaju kad prijeći s dana na tjedne i
// mjesece — to je dio koji smo do sada pisali sami i zato nismo imali datume na osi. `scaleUtc` (ne
// `scaleTime`): datumi u `Report` su civilni (`YYYY-MM-DD`), bez zone; UTC ponoć drži ticksove na
// istom mjestu bez obzira na ljetno vrijeme računala. `timeFormatLocale` nosi HR nazive mjeseci i
// dana; `d3-time` se NE uvozi izravno (tranzitivan, nije pinan) — ticksove daje sama skala.
import { scaleLinear, scaleUtc, type ScaleTime } from 'd3-scale';
import { timeFormatLocale, type TimeLocaleDefinition } from 'd3-time-format';
import type { Lang } from '../types';

const HR: TimeLocaleDefinition = {
  dateTime: '%A, %e. %B %Y. %X', date: '%d.%m.%Y.', time: '%H:%M:%S', periods: ['AM', 'PM'],
  days: ['nedjelja', 'ponedjeljak', 'utorak', 'srijeda', 'četvrtak', 'petak', 'subota'],
  shortDays: ['ned', 'pon', 'uto', 'sri', 'čet', 'pet', 'sub'],
  months: ['siječanj', 'veljača', 'ožujak', 'travanj', 'svibanj', 'lipanj', 'srpanj', 'kolovoz', 'rujan', 'listopad', 'studeni', 'prosinac'],
  shortMonths: ['sij', 'velj', 'ožu', 'tra', 'svi', 'lip', 'srp', 'kol', 'ruj', 'lis', 'stu', 'pro'],
};
const EN: TimeLocaleDefinition = {
  dateTime: '%x, %X', date: '%m/%d/%Y', time: '%H:%M:%S', periods: ['AM', 'PM'],
  days: ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'],
  shortDays: ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'],
  months: ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'],
  shortMonths: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
};
const locale = (lang: Lang) => timeFormatLocale(lang === 'hr' ? HR : EN);

export function parseYmd(ymd: string): Date {
  const [y, m, d] = ymd.split('-').map(Number);
  return new Date(Date.UTC(y ?? 1970, (m ?? 1) - 1, d ?? 1));
}
export function toYmd(d: Date): string {
  return d.toISOString().slice(0, 10);
}
export function timeScale(domain: [Date, Date], range: [number, number]): ScaleTime<number, number> {
  return scaleUtc().domain(domain).range(range);
}
export function niceMax(max: number): number {
  if (!Number.isFinite(max) || max <= 0) return 1;
  return scaleLinear().domain([0, max]).nice().domain()[1] ?? 1;
}
export function yTicks(max: number, count = 4): number[] {
  const top = niceMax(max);
  // Nikad više ticksova nego cijelih koraka: os s brojem commita ne smije pokazati 0,2 (Ruling R41,
  // dopuna T53) — `scaleLinear().domain([0, 1]).ticks(4)` bi inače vratio razlomke.
  return scaleLinear().domain([0, top]).ticks(Math.min(count, top));
}
// Dan-razina kad je raspon kraći od ≈ 2 mjeseca, inače mjesec-razina; d3 sam bira gustoću po `count`.
export function dateTicks(domain: [Date, Date], widthPx: number, lang: Lang): { at: Date; x: number; label: string }[] {
  const scale = timeScale(domain, [0, widthPx]);
  const count = Math.max(2, Math.floor(widthPx / 80));
  const days = (domain[1].getTime() - domain[0].getTime()) / 86_400_000;
  const fmt = locale(lang).utcFormat(days <= 62 ? (lang === 'hr' ? '%d.%m.' : '%b %d') : '%b %Y');
  return scale.ticks(count).map((at) => ({ at, x: scale(at), label: fmt(at) }));
}
export function formatDate(ymd: string, lang: Lang, g: 'day' | 'week' | 'month'): string {
  const d = parseYmd(ymd);
  const l = locale(lang);
  if (g === 'day') return l.utcFormat(lang === 'hr' ? '%d.%m.' : '%b %d')(d);
  if (g === 'week') return (lang === 'hr' ? 'tj. ' : 'wk ') + l.utcFormat('%V')(d);
  return l.utcFormat('%b %Y')(d);
}

// Dopunjeno M2/56 — `formatDate(…, 'month')` vraća „ruj 2026" (mjesec + godina za osi grafova s
// duljim rasponom); toplinska karta treba SAMO skraćeni mjesec bez godine za stupac koji otvara novi
// mjesec, i naziv dana bez ijednog datuma (redovi kalendara su „pon/sri/pet", ne datumi) — otud dvije
// male funkcije umjesto pregovora s postojećom `formatDate`.
export function formatMonth(ymd: string, lang: Lang): string {
  return locale(lang).utcFormat('%b')(parseYmd(ymd));
}
// `isoDay`: pon = 0 … ned = 6. `2024-01-01` je poznat ponedjeljak (provjeren kalendarom) — dan u
// tjednu ne ovisi o TOM konkretnom datumu, samo o pomaku od njega, pa je siguran kao referentna točka.
const KNOWN_MONDAY = parseYmd('2024-01-01');
export function formatWeekday(isoDay: number, lang: Lang): string {
  const d = new Date(KNOWN_MONDAY.getTime() + isoDay * 86_400_000);
  return locale(lang).utcFormat('%a')(d);
}

// Dopunjeno M2/55 (S-035) — `linear`, `finiteMax`, `arcPath`, `ringSegments`, `svgA11y` preseljeni
// ovamo iz `scale.ts` (M2/23, sada obrisan): `Sparkline`/`Ring` trebaju čistu linearnu skalu i SVG
// pristupačnost bez datuma na osi, pa ne prolaze kroz `layout.ts`. Stari `niceMax`/`ticks` iz
// `scale.ts` se NE sele (zamijenjeni d3-inačicom gore, T53); `cumulative` se briše bez zamjene —
// nije imao pozivatelja (jezgra već šalje `commits_cumulative` gotov, S-012).
export const linear =
  ([d0, d1]: [number, number], [r0, r1]: [number, number]) =>
  (x: number): number =>
    d1 === d0 ? r0 : r0 + ((x - d0) / (d1 - d0)) * (r1 - r0);

// Domenu grafa treba računati SAMO nad konačnim vrijednostima — jedan `NaN` u nizu ne smije
// srušiti maksimum cijelog grafa na `NaN` (koristi ga svaka komponenta umjesto `Math.max(...)`).
export const finiteMax = (values: number[]): number => Math.max(0, ...values.filter(Number.isFinite));

export function arcPath(cx: number, cy: number, r: number, a0: number, a1: number): string {
  // Pun krug (a1 - a0 = 2π) ima istu početnu i završnu točku pa se SVG-luk ne bi nacrtao;
  // makni završetak za epsilon da luk ostane vidljiv, a ostali kutovi prolaze nepromijenjeni.
  const EPS = 1e-3;
  const end = a1 - a0 >= Math.PI * 2 - EPS ? a0 + Math.PI * 2 - EPS : a1;
  const pt = (a: number): [number, number] => [
    +(cx + r * Math.sin(a)).toFixed(2),
    +(cy - r * Math.cos(a)).toFixed(2),
  ];
  const [x0, y0] = pt(a0);
  const [x1, y1] = pt(end);
  const large = end - a0 > Math.PI ? 1 : 0;
  return `M${x0} ${y0} A${r} ${r} 0 ${large} 1 ${x1} ${y1}`;
}

export function ringSegments(values: number[]): { start: number; end: number; share: number }[] {
  const total = values.reduce((a, b) => a + b, 0);
  // `total > 0` (ne `total <= 0`) hvata i NaN: jedan NaN u `values` čini zbroj NaN-om, a
  // `NaN <= 0` je `false` pa bi stara provjera propustila NaN dalje u dijeljenje (isti uzrok kao u `niceMax`).
  if (!(total > 0)) return [];
  let a = 0;
  return values.map((v) => {
    const share = v / total;
    const seg = { start: a, end: a + share * Math.PI * 2, share };
    a = seg.end;
    return seg;
  });
}

// Graf S imenom je slika (`role="img"` + `aria-label`), graf BEZ imena je ukras (`aria-hidden`) —
// čitač ekrana inače najavi „slika" bez ičega kad `label` nedostaje (recenzija M2/23, krug 1).
export function svgA11y(
  name: string | undefined,
): { role: 'img'; 'aria-label': string } | { 'aria-hidden': 'true' } {
  return name && name.trim() !== '' ? { role: 'img', 'aria-label': name } : { 'aria-hidden': 'true' };
}
