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
