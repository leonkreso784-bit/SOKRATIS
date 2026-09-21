// ZAŠTO OVAKO (cigla M2/25 — stanje aplikacije kao Svelte 5 runa)
// `$state` izvan komponente treba nastavak `.svelte.ts` da ga kompajler prepozna kao rune-modul
// (isti obrazac kao `src/lib/i18n/index.svelte.ts`). Jedan objekt `app` je jedini izvor istine za
// okvir — Topbar/Sidebar/pogledi ga čitaju izravno, bez proslijeđivanja kroz propse; promjena jednog
// polja sama osvježi svaku komponentu koja ga čita (S-012: sučelje samo drži ono što `Api` vrati).
import { api } from './api';
import type { ProjectSummary, Range, Report, Settings, Theme, View } from './types';

export const app = $state({
  projects: [] as ProjectSummary[],
  currentId: null as number | null,
  view: 'overview' as View,
  range: { preset: 'all' } as Range,
  report: null as Report | null,
  settings: { theme: 'academic', lang: 'hr', autostart: false } as Settings,
  loading: false,
});

// Piše `data-theme` na `<html>` — `tokens.css` sluša taj atribut za sve četiri palete (S-017).
export function applyTheme(theme: Theme): void {
  document.documentElement.dataset.theme = theme;
}

// Popis projekata za birač u gornjoj traci — poziva se jednom pri pokretanju (`App.svelte onMount`).
export async function loadProjects(): Promise<void> {
  app.projects = await api.listProjects();
}

// Odabir projekta u gornjoj traci: postavlja trenutni projekt i odmah učita njegov izvještaj.
export async function selectProject(id: number): Promise<void> {
  app.currentId = id;
  await loadReport();
}

// Ponovno dohvaća izvještaj za trenutni projekt i raspon — zove ga `RangePicker` na svaku promjenu
// i Topbar na klik „Osvježi". Bez odabranog projekta briše stari izvještaj (nema što prikazati).
export async function loadReport(): Promise<void> {
  if (app.currentId === null) {
    app.report = null;
    return;
  }
  app.loading = true;
  try {
    app.report = await api.getReport(app.currentId, app.range);
  } finally {
    app.loading = false;
  }
}
