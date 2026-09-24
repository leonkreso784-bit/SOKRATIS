// ZAŠTO OVAKO (cigla M2/26 — testovi za sortiranje, boju i sažetak signala u Pregledu;
// dopunjeno M2/27 — `phaseRows`/`bricksPerDay` za pogled Faze; krug popravka 1 — `kindColor`
// dijeljena mapa boja koju koriste i prsten i legenda u tablici u pogledu Vrste rada;
// dopunjeno M2/28 — `sortDiary`/`sortDeliveries` (poredak Dnevnika i Isporuka), `copyText`
// (kopiranje bez Rust naredbe, `vi.stubGlobal` zamjenjuje `navigator` jer Node pod vitestom nema
// `navigator.clipboard`) i `joinRepoPath` (sastavljanje putanje nalaza dokumentacije);
// dopuna M2/28 (uređivanje vizije) — `replaceVisionAt` (spec 6.2 "dodaj · uredi · promijeni stanje");
// krug popravka 1 — `parsePercent` izdvojen iz `Visions.svelte` (nalaz recenzije koda: čista funkcija
// s rubovima bez testa)
// Čiste funkcije, čist test bez DOM-a i bez Svelte runa: `hr.json` uvezen izravno kao `Dict`, isti
// obrazac kao `tests/format.test.ts` — `signalSummary` tako vidi PRAVI rječnik, ne ručno prepisan
// tekst (S-010), a hrvatska množina (jedan/dva-četiri/pet i više) se provjerava na stvarnim ključevima.
import { describe, expect, it, vi } from 'vitest';
import hr from '../src/lib/i18n/hr.json';
import type { CommitRow, Delivery, Phase, ProjectSummary, Vision, WorkKind } from '../src/lib/types';
import {
  bricksPerDay,
  copyText,
  joinRepoPath,
  kindColor,
  parsePercent,
  phaseRows,
  replaceVisionAt,
  severityClass,
  signalSummary,
  sortDeliveries,
  sortDiary,
  sortProjects,
} from '../src/views/helpers';

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

// Minimalna faza za testove — polja koja test ne provjerava dobivaju neutralnu vrijednost, ne `as Phase`
// (S-005 duh: prava vrijednost, ne pretvaranje da je tip zadovoljen).
const phase = (id: string, state: Phase['state'], days: Phase['days'] = null, doneBricks = 0): Phase => ({
  id,
  name: id,
  state,
  total_bricks: 1,
  done_bricks: doneBricks,
  from: null,
  to: null,
  days,
  commits: 0,
});

describe('phaseRows', () => {
  it('razvrstava faze po state u closed/running/planned', () => {
    const phases = [phase('a', 'closed'), phase('b', 'running'), phase('c', 'planned'), phase('d', 'closed')];
    const rows = phaseRows(phases);
    expect(rows.closed.map((p) => p.id)).toEqual(['a', 'd']);
    expect(rows.running.map((p) => p.id)).toEqual(['b']);
    expect(rows.planned.map((p) => p.id)).toEqual(['c']);
  });
  it('prazan popis daje tri prazna niza', () => {
    expect(phaseRows([])).toEqual({ closed: [], running: [], planned: [] });
  });
});

describe('bricksPerDay', () => {
  it('done_bricks / days', () => {
    expect(bricksPerDay(phase('x', 'running', 3, 6))).toBe(2);
  });
  it('days: 0 -> null (dijeljenje s nulom nema smisla)', () => {
    expect(bricksPerDay(phase('x', 'running', 0, 6))).toBeNull();
  });
  it('days: null -> null (faza bez razdoblja)', () => {
    expect(bricksPerDay(phase('x', 'planned', null, 0))).toBeNull();
  });
});

describe('kindColor', () => {
  it('svih pet vrsta rada ima svoj token boje (var(--color-...))', () => {
    const kinds: WorkKind[] = ['planning', 'documentation', 'execution', 'polish', 'debugging'];
    for (const k of kinds) {
      expect(kindColor(k)).toMatch(/^var\(--color-[a-z0-9-]+\)$/);
    }
  });
  it('svih pet je MEĐUSOBNO različitih tokena (segment prstena i redak tablice se ne smiju stopiti)', () => {
    const kinds: WorkKind[] = ['planning', 'documentation', 'execution', 'polish', 'debugging'];
    const colors = kinds.map(kindColor);
    expect(new Set(colors).size).toBe(5);
  });
  it('nepoznata vrsta ne pogađa tiho na krivu boju iz mape — vraća neutralni fallback', () => {
    expect(kindColor('nepostojeca' as WorkKind)).toBe('var(--color-ink-2)');
  });
});

// Minimalan redak Dnevnika — polja koja test ne provjerava dobivaju neutralnu vrijednost (isti duh
// kao `phase()` iznad: prava vrijednost tipa, ne `as CommitRow`).
const commitRow = (sha: string, date: string): CommitRow => ({
  sha,
  date,
  author_time: 0,
  subject: sha,
  kind: 'execution',
  sub: 'other',
  overridden: false,
  branch: 'main',
});

describe('sortDiary', () => {
  it('obrće redoslijed po datumu — najnoviji prvo (Step 1 cigle M2/28)', () => {
    const rows = [commitRow('a', '2026-01-01'), commitRow('b', '2026-01-02'), commitRow('c', '2026-01-03')];
    expect(sortDiary(rows).map((r) => r.sha)).toEqual(['c', 'b', 'a']);
  });
  it('isti datum zadržava ulazni redoslijed (stabilno sortiranje)', () => {
    const rows = [commitRow('a', '2026-01-01'), commitRow('b', '2026-01-01'), commitRow('c', '2026-01-02')];
    expect(sortDiary(rows).map((r) => r.sha)).toEqual(['c', 'a', 'b']);
  });
  it('ne mijenja ulazno polje — vraća novo, sortirano', () => {
    const rows = [commitRow('a', '2026-01-01'), commitRow('b', '2026-01-02')];
    const original = rows.map((r) => r.sha);
    const sorted = sortDiary(rows);
    expect(rows.map((r) => r.sha)).toEqual(original);
    expect(sorted).not.toBe(rows);
  });
});

const delivery = (title: string, date: string): Delivery => ({
  date,
  model: '',
  title,
  kind: 'execution',
  deploy: false,
});

describe('sortDeliveries', () => {
  it('isti poredak kao sortDiary — najnovije prvo, stabilno za isti datum', () => {
    const rows = [delivery('a', '2026-01-01'), delivery('b', '2026-01-01'), delivery('c', '2026-01-02')];
    expect(sortDeliveries(rows).map((r) => r.title)).toEqual(['c', 'a', 'b']);
  });
});

describe('joinRepoPath', () => {
  it('spaja korijen projekta i relativnu putanju nalaza s "\\" razdjelnikom', () => {
    expect(joinRepoPath('C:\\Users\\leonk\\Documents\\sokratstudy.dev', 'docs/records/DECISIONS.md')).toBe(
      'C:\\Users\\leonk\\Documents\\sokratstudy.dev\\docs\\records\\DECISIONS.md',
    );
  });
  it('normalizira "/" u korijenu isto kao u putanji nalaza (Finding.path uvijek dolazi s "/")', () => {
    expect(joinRepoPath('C:/repo', 'docs/a.md')).toBe('C:\\repo\\docs\\a.md');
  });
});

describe('copyText', () => {
  it('vraća false kad navigator.clipboard ne postoji (test okoliš pod vitestom/Nodeom)', async () => {
    expect(await copyText('nešto')).toBe(false);
  });
  it('poziva navigator.clipboard.writeText i vraća true kad postoji (WebView2 uz gestu korisnika)', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal('navigator', { clipboard: { writeText } });
    expect(await copyText('C:\\put\\do\\datoteke.md')).toBe(true);
    expect(writeText).toHaveBeenCalledWith('C:\\put\\do\\datoteke.md');
    vi.unstubAllGlobals();
  });
  it('vraća false kad writeText odbije (npr. dopuštenje odbijeno) — ne baca dalje', async () => {
    const writeText = vi.fn().mockRejectedValue(new Error('denied'));
    vi.stubGlobal('navigator', { clipboard: { writeText } });
    expect(await copyText('x')).toBe(false);
    vi.unstubAllGlobals();
  });
});

// Minimalna vizija — polja koja test ne provjerava dobivaju neutralnu vrijednost (isti duh kao
// `phase()`/`commitRow()` iznad: prava vrijednost tipa, ne `as Vision`).
const vision = (title: string, state: string): Vision => ({
  title,
  source: 'test',
  state,
  percent: null,
  note: '',
});

describe('replaceVisionAt', () => {
  it('zamjenjuje viziju na danom indeksu, ostale ostaju netaknute (dopuna cigle M2/28 — uređivanje)', () => {
    const visions = [vision('a', 'planned'), vision('b', 'planned'), vision('c', 'planned')];
    const updated = vision('b', 'done');
    const result = replaceVisionAt(visions, 1, updated);
    expect(result.map((v) => v.state)).toEqual(['planned', 'done', 'planned']);
    expect(result[1]).toBe(updated);
  });
  it('ne mijenja ulazno polje — vraća novo', () => {
    const visions = [vision('a', 'planned')];
    const original = visions.map((v) => v.state);
    const result = replaceVisionAt(visions, 0, vision('a', 'done'));
    expect(visions.map((v) => v.state)).toEqual(original);
    expect(result).not.toBe(visions);
  });
  it('indeks izvan raspona ostavlja sadržaj nepromijenjen (svejedno nova referenca)', () => {
    const visions = [vision('a', 'planned')];
    const result = replaceVisionAt(visions, 5, vision('x', 'done'));
    expect(result).toEqual(visions);
    expect(result).not.toBe(visions);
  });
});

describe('parsePercent', () => {
  it('prazan (ili sam razmak) string -> null — "nepoznato", ne nula (krug popravka 1)', () => {
    expect(parsePercent('')).toBeNull();
    expect(parsePercent('   ')).toBeNull();
  });
  it('nevaljan unos (nije broj) -> null', () => {
    expect(parsePercent('abc')).toBeNull();
  });
  it('valjan broj unutar 0-100 prolazi nepromijenjen', () => {
    expect(parsePercent('45')).toBe(45);
    expect(parsePercent('0')).toBe(0);
    expect(parsePercent('100')).toBe(100);
  });
  it('izvan raspona 0-100 se steže — <input min/max> su samo UI-nagovještaj, ne brana', () => {
    expect(parsePercent('150')).toBe(100);
    expect(parsePercent('-5')).toBe(0);
  });
});
