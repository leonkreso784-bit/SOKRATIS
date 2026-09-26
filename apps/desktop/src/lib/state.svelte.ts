// ZAŠTO OVAKO (cigla M2/25 — stanje aplikacije kao Svelte 5 runa; dopunjeno M2/34 — greška vidljiva
// korisniku umjesto tihog odbijenog Promisea, i "zadnji zahtjev pobjeđuje" za `getReport`; dopunjeno
// M2/38 — `applyMotion`/`syncMotion` pišu `data-motion` isto kao `applyTheme` piše `data-theme`;
// `syncMotion` je OVDJE, ne u `App.svelte`, jer `Settings.svelte` treba istu odluku bez pristupa
// lokalnoj `MediaQueryList` iz tuđeg `onMount`; dopunjeno M2/39 — `epoch` broji koliko je puta
// glavni prozor postao vidljiv korisniku; `App.svelte` njime omata poglede u `{#key}` da se ulazna
// animacija grafova (S-026) prikaže tek kad ima tko gledati, ne dok je prozor skriven iza splasha;
// dopunjeno M2/58 (S-034) — `selectProject` je zamijenjen s `enterProject` (klik na karticu Pregleda
// otvara ploču `project`) i `leaveProject` („‹ Pregled"); stari `selectProject` je obrisan jer je
// nakon ove cigle ostao bez ijednog pozivatelja (R58, S-010))
// `$state` izvan komponente treba nastavak `.svelte.ts` da ga kompajler prepozna kao rune-modul
// (isti obrazac kao `src/lib/i18n/index.svelte.ts`). Jedan objekt `app` je jedini izvor istine za
// okvir — Topbar/Sidebar/pogledi ga čitaju izravno, bez proslijeđivanja kroz propse; promjena jednog
// polja sama osvježi svaku komponentu koja ga čita (S-012: sučelje samo drži ono što `Api` vrati).
import { api } from './api';
import { motionOff } from './motion';
import type { ProjectSummary, Range, Report, Settings, Theme, View } from './types';

export const app = $state({
  projects: [] as ProjectSummary[],
  currentId: null as number | null,
  view: 'overview' as View,
  range: { preset: 'all' } as Range,
  report: null as Report | null,
  settings: { theme: 'academic', lang: 'hr', autostart: false, motion: true } as Settings,
  loading: false,
  error: null as string | null,
  epoch: 0,
});

// `TauriApi` odbija Promise s golim tekstom (Rust `Err(String)`, ne `Error` objektom); `MockApi`/
// vitest svejedno mogu odbiti s bilo čim, pa sve tri mogućnosti svode na jedan čitljiv tekst.
function errorText(e: unknown): string {
  return typeof e === 'string' ? e : e instanceof Error ? e.message : String(e);
}

// Jedno mjesto koje puni traku greške (App.svelte) — svaki `api.*` poziv u `views/`/`lib/shell/`
// koji nema svoj `catch` zove OVU funkciju (S-010: jedan mehanizam, ne sedam).
export function setError(e: unknown): void {
  app.error = errorText(e);
}

export function dismissError(): void {
  app.error = null;
}

// Piše `data-theme` na `<html>` — `tokens.css` sluša taj atribut za sve četiri palete (S-017).
export function applyTheme(theme: Theme): void {
  document.documentElement.dataset.theme = theme;
}

// Piše `data-motion` na `<html>` — `motion.css` sluša taj atribut i svodi svaki `--motion-dur` na 0
// kad je `"off"` (S-026). Čista upisna funkcija (bez odluke) da je `motionOff` (`./motion`) jedino
// mjesto koje odlučuje JE LI pokret ugašen — ovdje se ta odluka samo primjenjuje.
export function applyMotion(off: boolean): void {
  document.documentElement.dataset.motion = off ? 'off' : 'on';
}

// Jedno mjesto koje spaja postavku korisnika i postavku OS-a (`prefers-reduced-motion`) u odluku —
// zove ga `App.svelte` pri pokretanju i na svaku promjenu OS-postavke, te `Settings.svelte` nakon
// svakog upisa prekidača `motion` (R24): oboje moraju vidjeti ISTU odluku, ne dvije kopije iste
// logike.
export function syncMotion(): void {
  applyMotion(motionOff(app.settings.motion, window.matchMedia('(prefers-reduced-motion: reduce)').matches));
}

// Popis projekata za birač u gornjoj traci — poziva se jednom pri pokretanju (`App.svelte onMount`)
// i na svaki `report_updated`. Greška ide u `app.error`, ne u odbačen Promise (dopuna T34).
export async function loadProjects(): Promise<void> {
  try {
    app.projects = await api.listProjects();
  } catch (e) {
    setError(e);
  }
}

// Ulaz u projekt s Pregleda (S-034): klik na karticu otvara ploču `project` s njegovim izvještajem —
// jedini put u taj pogled (nema više birača u gornjoj traci).
export async function enterProject(id: number): Promise<void> {
  app.currentId = id;
  app.view = 'project';
  await loadReport();
}

// „‹ Pregled" u gornjoj traci: samo mijenja pogled, `currentId` OSTAJE — brz povratak u Dnevnik/
// Vizije ili ponovni ulazak u istu ploču ne traži nov odabir projekta.
export function leaveProject(): void {
  app.view = 'overview';
}

// Brojač raste sa svakim pozivom; odgovor se prihvaća SAMO ako je njegov token i dalje najnoviji —
// isti obrazac kao `requestToken` u `Indicators.svelte`. Bez ovoga bi korisnik koji brzo mijenja
// raspon mogao vidjeti STARIJI izvještaj kako pregazi noviji, ovisno koji `getReport` nad repoom
// stigne prvi (dopuna T34, "zadnji zahtjev pobjeđuje").
let reportRequestToken = 0;

// Ponovno dohvaća izvještaj za trenutni projekt i raspon — zove ga `RangePicker` na svaku promjenu
// i Topbar na klik „Osvježi". Bez odabranog projekta briše stari izvještaj (nema što prikazati).
export async function loadReport(): Promise<void> {
  if (app.currentId === null) {
    app.report = null;
    return;
  }
  const id = app.currentId;
  const range = app.range;
  const token = ++reportRequestToken;
  app.loading = true;
  try {
    const report = await api.getReport(id, range);
    if (token !== reportRequestToken) return; // stigao je noviji zahtjev — ovaj odgovor je zastario
    app.report = report;
  } catch (e) {
    if (token === reportRequestToken) setError(e);
  } finally {
    if (token === reportRequestToken) app.loading = false;
  }
}
