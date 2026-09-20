// ZAŠTO OVAKO (cigla M2/26 — čisti izračuni odvojeni od Overview.svelte)
// Sortiranje, boja po težini i sažetak signala su čisti izračuni bez DOM-a/Svelte runa — žive u
// `.ts` modulu da ih vitest testira izravno (isti obrazac kao `format.ts`, S-012), a `Overview.svelte`
// ostaje tanak prikaz koji samo poziva ove funkcije. Množina ide iz rječnika (S-021), ova datoteka
// samo bira KOJI oblik (jedan/dva-četiri/pet-i-više) — sam tekst nikad nije ovdje ušiven.
import { translate, type Dict } from '../lib/i18n/t';
import type { ProjectSummary, Severity } from '../lib/types';

const SEVERITY_RANK: Record<Severity, number> = { alert: 0, warn: 1, info: 2 };
const NO_SIGNAL_RANK = 3;

// Natpis se boji SAMO kroz tekstualne tokene (`tokens.css`/`app.css`) — nikad heksica u markupu.
export function severityClass(s: Severity | null): 'text-ink-2' | 'text-ink-blue' | 'text-warn-ink' | 'text-danger-ink' {
  if (s === 'alert') return 'text-danger-ink';
  if (s === 'warn') return 'text-warn-ink';
  if (s === 'info') return 'text-ink-blue';
  return 'text-ink-2';
}

// Najteži signal prvo (uzbuna → upozorenje → info → bez signala), pa po imenu unutar iste težine —
// projekt koji nešto vapi neka bude prvi u mreži kartica (spec 6.1).
export function sortProjects(ps: ProjectSummary[]): ProjectSummary[] {
  const rank = (worst: Severity | null): number => (worst === null ? NO_SIGNAL_RANK : SEVERITY_RANK[worst]);
  // Krug popravka 1 (recenzija): `'hr'` je OBVEZAN drugi argument, ne kozmetika — `localeCompare`
  // BEZ njega uzima zadani locale izvršnog konteksta (na ovom stroju Node vraća 'en-CA' i stavlja
  // "Čokolada" ISPRED "Cvijet"; hrvatska abeceda traži C < Č < D, dakle "Cvijet" prvo). WebView2 i
  // Node dijele isto ICU/CLDR (oba Chromium), pa je poredak izmjeren u `tests/helpers.test.ts`
  // ugovor koji ostaje isti bez obzira na jezik operacijskog sustava (isti rizik koji `format.ts`
  // M2/22 izričito imenuje za brojeve/datume — ovdje je riječ o slaganju imena).
  return [...ps].sort((a, b) => rank(a.worst) - rank(b.worst) || a.name.localeCompare(b.name, 'hr'));
}

// CLDR kategorije hrvatske množine (one/few/many): 1 → jedan, 2-4 → few (osim 12-14), ostalo → many.
// Engleski rječnik iste ključeve samo puni istim tekstom za few/many (S-021 — pravilo je zajedničko,
// tekst po jeziku nije).
function pluralCategory(n: number): 'one' | 'few' | 'many' {
  const mod10 = n % 10;
  const mod100 = n % 100;
  if (mod10 === 1 && mod100 !== 11) return 'one';
  if (mod10 >= 2 && mod10 <= 4 && (mod100 < 12 || mod100 > 14)) return 'few';
  return 'many';
}

// "2 uzbune · 1 upozorenje" — samo težine koje se stvarno pojavljuju, uzbuna prvo; bez ijedne signal
// je t('signals.none'). Razdjelnik "·" je ovdje jer nije natpis nego interpunkcija između natpisa.
export function signalSummary(c: { info: number; warn: number; alert: number }, dict: Dict): string {
  const parts = (['alert', 'warn', 'info'] as const)
    .filter((sev) => c[sev] > 0)
    .map((sev) => translate(dict, `signals.count.${sev}.${pluralCategory(c[sev])}`, { n: c[sev] }));
  return parts.length > 0 ? parts.join(' · ') : translate(dict, 'signals.none');
}
