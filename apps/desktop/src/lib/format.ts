// ZAŠTO OVAKO (cigla M2/22 — oblikovanje na jednom mjestu)
// Jezgra daje razlomke i sekunde (S-008: brojka bez jedinice), sučelje ih oblikuje OVDJE i nigdje
// drugdje — inače bi postotak u Tempu i u Vrstama rada mogao izgledati različito (dug M3 iz M1).
// `Intl.NumberFormat` zna zarez/točku po jeziku bez ručnih zamjena; razmak i predznak se dodatno
// normaliziraju jer `hr-HR` u Node/ICU vraća nedjeljivi razmak (U+00A0) i pravi predznak minus
// (U+2212) — WebView2 ima svoj ICU pa ponašanje mora biti izmjereno i osigurano kodom, ne pretpostavljeno.
//
// Dopunjeno M2/63 — m1: `localYmd` dodan jer je „danas" u Ganttu i toplinskoj karti računao UTC dan
// (`toISOString().slice(0, 10)`), a natpisi obećavaju datum OVOG računala — vidi poziv u
// `TempoSection.svelte`/`PhasesSection.svelte`.
import type { Lang } from './i18n/index.svelte';
import { translate, type Dict } from './i18n/t';

const DASH = '—';
const locale = (lang: Lang): string => (lang === 'hr' ? 'hr-HR' : 'en-GB');
const bad = (x: unknown): x is null | undefined =>
  x === null || x === undefined || (typeof x === 'number' && Number.isNaN(x));
// U+00A0 (nedjeljivi razmak) i U+202F (uski nedjeljivi razmak) → obični razmak; U+2212 (pravi minus) → crtica.
const clean = (s: string): string => s.replace(/[  ]/g, ' ').replace(/−/g, '-');

export function num(n: number | null | undefined, lang: Lang, digits = 0): string {
  if (bad(n)) return DASH;
  const formatted = new Intl.NumberFormat(locale(lang), {
    minimumFractionDigits: digits,
    maximumFractionDigits: digits,
  }).format(n);
  return clean(formatted);
}

export function percent(x: number | null | undefined, lang: Lang, digits = 1): string {
  if (bad(x)) return DASH;
  return num(x * 100, lang, digits) + (lang === 'hr' ? ' %' : '%');
}

export function hours(h: number | null | undefined, lang: Lang): string {
  return bad(h) ? DASH : `${num(h, lang, 1)} h`;
}

// Dopunjeno M2/63 — m1: `getFullYear`/`getMonth`/`getDate` čitaju LOKALNO vrijeme računala (za
// razliku od `toISOString`, koji prvo pretvara u UTC) — Gantt i toplinska karta crtaju „danas" po
// satu stroja na kojem Sokratis radi, pa oko ponoći ne smiju iskočiti u sutrašnji (UTC) dan.
export function localYmd(d: Date): string {
  const y = d.getFullYear();
  const m = String(d.getMonth() + 1).padStart(2, '0');
  const day = String(d.getDate()).padStart(2, '0');
  return `${y}-${m}-${day}`;
}

export function ymd(d: string | null | undefined, lang: Lang): string {
  if (!d) return DASH;
  const [y, m, day] = d.split('-').map(Number);
  if (!y || !m || !day) return d;
  return lang === 'hr' ? `${day}. ${m}. ${y}.` : d;
}

export function relative(nowSec: number, thenSec: number | null | undefined, dict: Dict): string {
  if (bad(thenSec)) return DASH;
  const s = Math.max(0, nowSec - thenSec);
  if (s < 60) return translate(dict, 'time.now');
  if (s < 3600) return translate(dict, 'time.minutes', { n: Math.floor(s / 60) });
  if (s < 86400) return translate(dict, 'time.hours', { n: Math.floor(s / 3600) });
  return translate(dict, 'time.days', { n: Math.floor(s / 86400) });
}

export function phaseState(state: 'planned' | 'running' | 'closed', dict: Dict): string {
  return translate(dict, `phase.${state}`);
}

export function kindLabel(kind: string, dict: Dict): string {
  return translate(dict, `kind.${kind}`);
}

export function subLabel(sub: string, dict: Dict): string {
  return translate(dict, `sub.${sub}`);
}
