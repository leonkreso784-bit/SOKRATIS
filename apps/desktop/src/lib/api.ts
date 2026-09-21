// ZAŠTO OVAKO (cigla M2/25 — jedan API, dvije izvedbe)
// Sučelje vidi SAMO sučelje `Api`, nikad izravno Tauri ni mock (ovisnost o apstrakciji, ne o izvedbi).
// `MockApi` čita insta snapshot prave jezgre kroz Viteov `?raw` uvoz, pa dev-prikaz i testovi crtaju
// TOČNO one brojke koje jezgra stvarno izračuna — nema ručno prepisane kopije podataka (S-010).
// `TauriApi` (poziva Rust preko `invoke`) dolazi u T34; do tada `createApi()` uvijek vraća mock.
import snapRaw from '../../../../crates/sokratis-core/tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap?raw';
import type {
  CommitRow,
  DocsHealth,
  ProjectSummary,
  Range,
  Report,
  Settings,
  Severity,
  Signal,
  TrendPoint,
  Vision,
  VisionTotal,
  WorkKind,
} from './types';

// Insta piše YAML-zaglavlje omeđeno retkom `---` na vrhu i na dnu zaglavlja, a JSON počinje odmah
// iza drugog `---`. Prvi `indexOf` preskače početni redak, drugi traži redak koji zatvara zaglavlje.
export function parseSnap(text: string): unknown {
  const headerStart = text.indexOf('---\n');
  const headerEnd = text.indexOf('\n---\n', headerStart + 4);
  return JSON.parse(text.slice(headerEnd + 5));
}

export interface Api {
  listProjects(): Promise<ProjectSummary[]>;
  addProject(): Promise<ProjectSummary | null>;
  renameProject(id: number, name: string): Promise<void>;
  removeProject(id: number): Promise<void>;
  getReport(id: number, range: Range): Promise<Report>;
  getTrend(id: number, metric: string, range: Range): Promise<TrendPoint[]>;
  setOverride(id: number, sha: string, kind: WorkKind | null): Promise<void>;
  saveVisions(id: number, visions: Vision[]): Promise<void>;
  refresh(id?: number): Promise<void>;
  getSettings(): Promise<Settings>;
  setSetting<K extends keyof Settings>(key: K, value: Settings[K]): Promise<void>;
  onReportUpdated(cb: (id: number) => void): () => void;
  onSignalRaised(cb: (e: { project_id: number; rule: string; severity: Severity }) => void): () => void;
}

const MOCK_PROJECT_ID = 1;

// Jedan lažni projekt, „Sokrat Study (snimka)" — `Report` dolazi iz insta snapshota jezgre, pa
// dev-prikaz i vitest vide iste brojke koje bi jezgra stvarno izračunala nad pravim repozitorijem.
export class MockApi implements Api {
  private readonly report: Report = parseSnap(snapRaw) as Report;
  private readonly overrides = new Map<string, WorkKind>();
  private visions: Vision[] = [];
  private readonly listeners = new Set<(id: number) => void>();

  // Snimka jezgre (M2/17) nema signala — pravila (M2/29+) se nisu ni pokrenula kad je snimljena, pa
  // `this.report.signals` je uvijek `[]`. Bez primjera traka signala (M2/26) i kartica u Pregledu
  // nemaju što pokazati u dev-prikazu ni testu. Ovo je SAMO mock koji se koristi kad snimka nema
  // nijedan signal — `TauriApi` (T34) uvijek vraća prave signale koje je jezgra stvarno izračunala.
  private readonly mockSignals: Signal[] = [
    {
      rule: 'unmerged-branches',
      severity: 'warn',
      title_key: 'signal.unmerged_branches',
      evidence: ['feat/x nije spojena u main (12 commita iza)', 'feat/y nije spojena u main (3 commita iza)'],
      since: null,
    },
  ];

  // Dopuna M2/28 (Ruling orkestratora #2): snimka jezgre (M2/17) ima `docs: null` jer je snimljena
  // NAD PROJEKTOM koji tada nije imao mapu dokumentacije prepoznatu profilom. Bez primjera pogled
  // Dokumentacija (M2/28) ne bi imao što pokazati u dev-prikazu ni testu osim `docs.na`. Poruke u
  // nalazima su izričito označene "MOCK" da nitko ne pomisli da je ovo prava ocjena — `TauriApi`
  // (T34) uvijek vraća pravu `DocsHealth` koju izračuna `core`, ovaj primjer se tad sam prestaje koristiti.
  private readonly mockDocsHealth: DocsHealth = {
    score: 82,
    findings: [
      {
        check: 'stale-decision',
        path: 'docs/records/DECISIONS.md',
        line: 12,
        message: 'MOCK primjer: odluka bez datuma zatvaranja (dev-prikaz dok TauriApi ne postoji, T34)',
      },
      {
        check: 'missing-glossary-entry',
        path: 'docs/workflow/RUST.md',
        line: null,
        message: 'MOCK primjer: nov Rust-konstrukt bez unosa u pojmovnik (dev-prikaz dok TauriApi ne postoji, T34)',
      },
    ],
    lag_days: 3,
  };

  async listProjects(): Promise<ProjectSummary[]> {
    const counts = this.countSignalsBySeverity();
    return [
      {
        id: MOCK_PROJECT_ID,
        name: 'Sokrat Study (snimka)',
        root_path: 'C:\\Users\\leonk\\Documents\\sokratstudy.dev',
        worktrees: 5,
        last_refresh: this.report.generated_at,
        last_commit: null,
        worst: this.worstSeverity(counts),
        signals: counts,
        error: null,
      },
    ];
  }

  async addProject(): Promise<ProjectSummary | null> {
    // Mock ne otvara dijalog za odabir mape — T34 (TauriApi) to zamjenjuje pravim dijalogom.
    return null;
  }

  async renameProject(_id: number, _name: string): Promise<void> {
    // Mock nema trajnu pohranu za ime projekta; ostaje bez efekta do TauriApija (T34).
  }

  async removeProject(_id: number): Promise<void> {
    // Isto — jedini mock-projekt je zakucan, uklanjanje nema smisla dok ne postoji registar (STORE).
  }

  async refresh(_id?: number): Promise<void> {
    for (const listener of this.listeners) listener(MOCK_PROJECT_ID);
  }

  async getReport(_id: number, _range: Range): Promise<Report> {
    // Mock ignorira `_id` (postoji samo jedan projekt) i `_range` (nema drugog razdoblja u snimci) —
    // parametri postoje da poziv izvana izgleda točno kao poziv prema pravom `TauriApi`-ju (T34).
    return {
      ...this.report,
      commits: this.commitsWithOverrides(),
      visions: this.visions,
      vision_totals: this.visionTotals(),
      docs: this.docsForReport(),
      signals: this.signalsForReport(),
    };
  }

  async getTrend(_id: number, _metric: string, _range: Range): Promise<TrendPoint[]> {
    // Dvije točke dovoljne su za sparkline u pregledu pokazatelja (T27); prava povijest dolazi iz
    // snimki u `sokratis-store`, koje mock nema.
    return [
      { taken_on: '2026-09-16', value: 180, profile_changed: false },
      { taken_on: '2026-09-17', value: 190, profile_changed: false },
    ];
  }

  async setOverride(_id: number, sha: string, kind: WorkKind | null): Promise<void> {
    if (kind === null) {
      this.overrides.delete(sha);
    } else {
      this.overrides.set(sha, kind);
    }
  }

  async saveVisions(_id: number, visions: Vision[]): Promise<void> {
    this.visions = visions;
  }

  async getSettings(): Promise<Settings> {
    return { theme: 'academic', lang: 'hr', autostart: false };
  }

  async setSetting(_key: keyof Settings, _value: Settings[keyof Settings]): Promise<void> {
    // Mock ne pamti postavke između pokretanja — `state.svelte.ts` drži trenutnu vrijednost u
    // memoriji, a TauriApi (T34) je stvarno sprema u `sokratis-store`.
  }

  onReportUpdated(cb: (id: number) => void): () => void {
    this.listeners.add(cb);
    return () => this.listeners.delete(cb);
  }

  onSignalRaised(_cb: (e: { project_id: number; rule: string; severity: Severity }) => void): () => void {
    // Mock nikad ne javlja nov signal (nema pozadinski watcher); T34 veže pravi Tauri-event.
    return () => {};
  }

  // ── pomoćne metode — rastavljaju brifov zgusnuti zapis u čitljive korake, ponašanje isto ──

  private commitsWithOverrides(): CommitRow[] {
    return this.report.commits.map((commit) => {
      const overriddenKind = this.overrides.get(commit.sha);
      if (overriddenKind === undefined) return commit;
      return { ...commit, kind: overriddenKind, overridden: true };
    });
  }

  private visionTotals(): VisionTotal[] {
    const states = [...new Set(this.visions.map((vision) => vision.state))].sort();
    return states.map((state) => ({
      state,
      count: this.visions.filter((vision) => vision.state === state).length,
    }));
  }

  private countSignalsBySeverity(): { info: number; warn: number; alert: number } {
    const signals = this.signalsForReport();
    const info = signals.filter((s) => s.severity === 'info').length;
    const warn = signals.filter((s) => s.severity === 'warn').length;
    const alert = signals.filter((s) => s.severity === 'alert').length;
    return { info, warn, alert };
  }

  // Prava snimka nikad nema signala (gore) — dok postoji, mock ih nadomjesti primjerom da Pregled i
  // traka signala imaju što pokazati; čim jezgra jednom isporuči neprazan `signals`, ovo se samo od
  // sebe prestaje koristiti.
  private signalsForReport(): Signal[] {
    return this.report.signals.length > 0 ? this.report.signals : this.mockSignals;
  }

  // Isti obrazac kao `signalsForReport` iznad, ali za `docs`: snimka ima `null`, mock ga zamijeni
  // OZNAČENIM primjerom (§ komentar uz `mockDocsHealth`) — grana `docs === null` u `Docs.svelte`
  // time i dalje ostaje pravi, testiran kôd za projekt koji STVARNO nema mapu dokumentacije.
  private docsForReport(): DocsHealth | null {
    return this.report.docs ?? this.mockDocsHealth;
  }

  // Info se ne broji u „najgore" — nijedno pravilo (M2) danas ne javlja Info, a i da javi, to nije
  // razlog za crvenu karticu u Pregledu.
  private worstSeverity(counts: { info: number; warn: number; alert: number }): Severity | null {
    if (counts.alert > 0) return 'alert';
    if (counts.warn > 0) return 'warn';
    return null;
  }
}

export function createApi(): Api {
  // T34 ovdje dodaje granu za pravi Tauri prozor (provjera `'__TAURI_INTERNALS__' in window`,
  // isti obrazac kao `Splash.svelte`); do tada je mock jedina izvedba.
  return new MockApi();
}

export const api: Api = createApi();
