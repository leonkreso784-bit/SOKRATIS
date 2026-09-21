// ZAŠTO OVAKO (cigla M2/26 — čisti izračuni odvojeni od Overview.svelte)
// Sortiranje, boja po težini i sažetak signala su čisti izračuni bez DOM-a/Svelte runa — žive u
// `.ts` modulu da ih vitest testira izravno (isti obrazac kao `format.ts`, S-012), a `Overview.svelte`
// ostaje tanak prikaz koji samo poziva ove funkcije. Množina ide iz rječnika (S-021), ova datoteka
// samo bira KOJI oblik (jedan/dva-četiri/pet-i-više) — sam tekst nikad nije ovdje ušiven.
import { translate, type Dict } from '../lib/i18n/t';
import type { CommitRow, Delivery, Phase, ProjectSummary, Severity, WorkKind } from '../lib/types';

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

// Pogled Faze (cigla M2/27) razvrstava `Report.phases` po `state` u tri odjeljka; jezgra već zna
// stanje svake faze (`model.rs`), ovo je samo razvrstavanje — nijedan izračun (S-012).
export function phaseRows(phases: Phase[]): { closed: Phase[]; running: Phase[]; planned: Phase[] } {
  return {
    closed: phases.filter((p) => p.state === 'closed'),
    running: phases.filter((p) => p.state === 'running'),
    planned: phases.filter((p) => p.state === 'planned'),
  };
}

// Odstupanje od S-012 (zapisano u ledgeru M2/27, odluka orkestratora): omjer "cigle/dan" (spec §6.2)
// jezgra još ne daje u `Phase` — dok taj dug ne bude zatvoren u `core`, računa se OVDJE i samo ovdje.
// `days` je `0` ili `null` kad faza nema razdoblje (npr. tek planirana) — dijeljenje bi dalo `NaN`/
// `Infinity`, pa oboje vraća `null` i `format.ts` ga ispisuje kao crticu.
export function bricksPerDay(p: Phase): number | null {
  if (!p.days) return null;
  return p.done_bricks / p.days;
}

// Krug popravka 1 (vizualna provjera M2/27): pogled Vrste rada crta boju NA DVA MJESTA (segment
// prstena i oznaka u retku tablice) — obje strane moraju čitati boju iz JEDNE mape, inače prva
// izmjena tokena razvuče prsten i legendu u dvije različite boje bez ijedne greške u testu (S-010).
// Pet tokena iz `tokens.css`, provjereno da sve postoje pod ovim imenima (M2/27 brief).
const KIND_COLORS: Record<WorkKind, string> = {
  planning: 'var(--color-brand-500)',
  documentation: 'var(--color-accent)',
  execution: 'var(--color-ink-green)',
  polish: 'var(--color-ink-amber)',
  debugging: 'var(--color-ink-red)',
};

// `WorkKind` je zatvoren enum u TypeScriptu, ali `Report` stiže kao JSON (runtime ne provjerava
// tip) — nepoznata vrsta NE smije tiho pogoditi susjedni token u mapi, zato eksplicitan fallback
// na neutralnu tintu umjesto pukog `KIND_COLORS[kind]` bez zaštite.
export function kindColor(kind: WorkKind): string {
  return KIND_COLORS[kind] ?? 'var(--color-ink-2)';
}

// ── Dnevnik i Isporuke (cigla M2/28) — najnovije prvo, jedan komparator dijele oba pogleda ──

// Datumi su ISO "YYYY-MM-DD": obična niz-usporedba daje ispravan kronološki poredak bez
// `localeCompare` i bez pitanja o jeziku sustava (za razliku od imena u `sortProjects`, gdje je
// abeceda jezično osjetljiva). `Array.prototype.sort` je stabilan od ES2019 — isti datum zadržava
// ulazni redoslijed, zato ga NIJE potrebno posebno kodirati kao tie-break.
function byDateDesc(a: { date: string }, b: { date: string }): number {
  if (a.date < b.date) return 1;
  if (a.date > b.date) return -1;
  return 0;
}

export function sortDiary(rows: CommitRow[]): CommitRow[] {
  return [...rows].sort(byDateDesc);
}

export function sortDeliveries(rows: Delivery[]): Delivery[] {
  return [...rows].sort(byDateDesc);
}

// ZAŠTO OVAKO (cigla M2/28 — kopiranje bez Rust naredbe; odstupanje zapisano u brifu T28 i za T34)
// `navigator.clipboard.writeText` postoji u WebView2 uz korisničku gestu (klik), pa Tauri naredba
// `copy_path` iz spec-a §5.1 OVDJE ne nastaje — T34 (TauriApi) ionako ne mijenja ovu funkciju.
// `typeof navigator === 'undefined'` čuva od okoliša bez DOM-a (Node pod vitestom NEMA
// `navigator.clipboard`, ali noviji Node ipak definira `navigator` bez njega — provjera oba sloja).
export async function copyText(text: string): Promise<boolean> {
  if (typeof navigator === 'undefined' || !navigator.clipboard?.writeText) return false;
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch {
    return false;
  }
}

// ZAŠTO OVAKO (cigla M2/28 — sastavljanje apsolutne putanje nalaza dokumentacije, Ruling brifa #5)
// `Finding.path` je relativan prema korijenu projekta i uvijek dolazi s "/" (jezgra ga čita s gita,
// S-012 — sučelje ništa ne izmišlja). Spajanje s `root_path` je JEDNA čista funkcija s testom — ne
// lijepljenje stringova u `Docs.svelte` — jer Windows-put (s "\") je ono što korisnik lijepi u
// Explorer ili urednik, ne git-put (s "/").
export function joinRepoPath(rootPath: string, path: string): string {
  const root = rootPath.replace(/\//g, '\\');
  const rel = path.replace(/\//g, '\\');
  return `${root}\\${rel}`;
}
