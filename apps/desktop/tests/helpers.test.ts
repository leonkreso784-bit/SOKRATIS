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
