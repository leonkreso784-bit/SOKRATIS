// ZAŠTO OVAKO (cigla M2/26 — testovi za sortiranje, boju i sažetak signala u Pregledu)
// Čiste funkcije, čist test bez DOM-a i bez Svelte runa: `hr.json` uvezen izravno kao `Dict`, isti
// obrazac kao `tests/format.test.ts` — `signalSummary` tako vidi PRAVI rječnik, ne ručno prepisan
// tekst (S-010), a hrvatska množina (jedan/dva-četiri/pet i više) se provjerava na stvarnim ključevima.
import { describe, expect, it } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import type { ProjectSummary } from '../src/lib/types';
import { severityClass, signalSummary, sortProjects } from '../src/views/helpers';

const project = (name: string, worst: ProjectSummary['worst']): ProjectSummary => ({
  id: name.length,
  name,
  root_path: 'C:\\p\\' + name,
  worktrees: 1,
  last_refresh: null,
  last_commit: null,
  worst,
  signals: { info: 0, warn: 0, alert: 0 },
  error: null,
});

describe('sortProjects', () => {
  it('stavlja alert prije warn prije bez signala; unutar iste težine po imenu', () => {
    const ps = [project('Zeta', null), project('Beta', 'warn'), project('Alfa', 'alert'), project('Cazu', 'alert')];
    expect(sortProjects(ps).map((p) => p.name)).toEqual(['Alfa', 'Cazu', 'Beta', 'Zeta']);
  });
  it('info stoji između warn i bez signala (alert > warn > info > null)', () => {
    const ps = [project('D', null), project('C', 'info'), project('B', 'warn'), project('A', 'alert')];
    expect(sortProjects(ps).map((p) => p.name)).toEqual(['A', 'B', 'C', 'D']);
  });
  // Krug popravka 1 (recenzija): tie-break po imenu mora koristiti FIKSNI locale ('hr'), ne zadani
  // locale izvršnog konteksta — na ovom stroju (Node, zadani locale en-CA) `localeCompare` BEZ
  // drugog argumenta stavlja "Čokolada" ISPRED "Cvijet" (naslijeđeni encoding-poredak dijakritika),
  // a hrvatska abeceda traži C < Č < D → "Cvijet" prvo. WebView2 dijeli isti ICU/CLDR (Chromium) kao
  // Node s punim ICU-om, pa je ishod izmjeren OVDJE ugovor koji `'hr'` mora održati posvuda.
  it('tie-break po imenu koristi hrvatsku abecedu bez obzira na sustavski jezik (dijakritici)', () => {
    const ps = [project('Dunja', 'warn'), project('Čokolada', 'warn'), project('Cvijet', 'warn')];
    expect(sortProjects(ps).map((p) => p.name)).toEqual(['Cvijet', 'Čokolada', 'Dunja']);
  });
  it('ne mijenja ulazno polje — vraća novo, sortirano', () => {
    const ps = [project('Zeta', null), project('Alfa', 'alert')];
    const original = ps.map((p) => p.name);
    const sorted = sortProjects(ps);
    expect(ps.map((p) => p.name)).toEqual(original);
    expect(sorted).not.toBe(ps);
  });
});

describe('severityClass', () => {
  it("'alert' nosi boju koja sadrži 'danger'", () => {
    expect(severityClass('alert')).toContain('danger');
  });
  it('bez signala je najmirnija (neutralna) boja', () => {
    expect(severityClass(null)).toBe('text-ink-2');
  });
});

describe('signalSummary', () => {
  it('bez ijednog signala vraća t(signals.none)', () => {
    expect(signalSummary({ info: 0, warn: 0, alert: 0 }, hr)).toBe('nema signala');
  });
  it('uzbuna prije upozorenja, hrvatska množina za broj 2 (few oblik)', () => {
    expect(signalSummary({ info: 0, warn: 1, alert: 2 }, hr)).toBe('2 uzbune · 1 upozorenje');
  });
  it('pet i više ide na "many" oblik hrvatske množine', () => {
    expect(signalSummary({ info: 0, warn: 0, alert: 5 }, hr)).toBe('5 uzbuna');
  });
});
