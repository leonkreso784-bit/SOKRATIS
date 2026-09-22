// ZAŠTO OVAKO (cigla M2/25 — TS zrcalo ugovora Report)
// `interface`/`type` bez implementacije: ovo je SAMO oblik podataka koje Rust (`crates/sokratis-core
// /src/model.rs`) šalje kao JSON (S-012), sučelje ništa ne računa. Polja, imena i redoslijed prate
// `model.rs` doslovno — `Option<T>` u Rustu postaje `T | null` (serde ga serijalizira kao `null`,
// nikad ga ne izostavlja, pa `?` ovdje ne bi bio točan opis). Enumi su `snake_case` string-literali
// jer `#[serde(rename_all = "snake_case")]` u Rustu daje `"debugging"`, ne `"Debugging"` (S-008).
// Dopunjeno M2/38 — `Settings.motion` i `View`+`'settings'` niže: oblici koje `model.rs` ne zna
// (popis projekata, raspon, postavke), sučelje ih drži samo za sebe.
import type { Lang } from './i18n/index.svelte';

export type { Lang };

export type WorkKind = 'planning' | 'documentation' | 'execution' | 'polish' | 'debugging';
export type SubKind = 'brick' | 'gate_or_measure' | 'deploy' | 'other';
export type Severity = 'info' | 'warn' | 'alert';
export type PhaseState = 'planned' | 'running' | 'closed';
export type IndicatorKind = 'measure' | 'proxy';

export interface Touched {
  commits: number;
  lines: number;
  files: number;
  skipped_lines: number;
}

export interface DayStats {
  date: string;
  commits: number;
  commits_cumulative: number;
  lines: number;
  hours: number;
  deliveries: number;
  deploys: number;
  test_lines: number;
}

export interface KindStats {
  kind: WorkKind;
  commits: number;
  share: number;
  lines: number;
}

// Jedan redak Dnevnika (M2/5): commit s vrstom, podvrstom i oznakom ručnog overridea.
// M2/29a: author_time (unix sekunde) odmah iza date — Pregled iz njega crta "prije X".
export interface CommitRow {
  sha: string;
  date: string;
  author_time: number;
  subject: string;
  kind: WorkKind;
  sub: SubKind;
  overridden: boolean;
}

export interface Delivery {
  date: string;
  model: string;
  title: string;
  kind: WorkKind;
  deploy: boolean;
}

export interface Indicator {
  id: string;
  value: number;
  kind: IndicatorKind;
  formula: string;
}

export interface Phase {
  id: string;
  name: string;
  state: PhaseState;
  total_bricks: number;
  done_bricks: number;
  from: string | null;
  to: string | null;
  days: number | null;
  commits: number;
}

export interface Vision {
  title: string;
  source: string;
  state: string;
  percent: number | null;
  note: string;
}

// Zbroj vizija po stanju (`vision_totals` u `model.rs`) — mjerenje, ne prikaz.
export interface VisionTotal {
  state: string;
  count: number;
}

export interface Finding {
  check: string;
  path: string;
  line: number | null;
  message: string;
}

export interface DocsHealth {
  score: number;
  findings: Finding[];
  lag_days: number | null;
}

export interface Signal {
  rule: string;
  severity: Severity;
  title_key: string;
  evidence: string[];
  since: number | null;
}

// Zrcali `Report` iz `model.rs` polje po polje — vidi zaglavlje datoteke.
export interface Report {
  generated_at: number;
  since: string;
  until: string | null;
  branch: string;
  touched: Touched;
  days: DayStats[];
  kinds: KindStats[];
  commits: CommitRow[];
  deliveries: Delivery[];
  indicators: Indicator[];
  phases: Phase[];
  visions: Vision[];
  vision_totals: VisionTotal[];
  docs: DocsHealth | null;
  signals: Signal[];
}

// ── Ono što `model.rs` ne zna: oblici koje treba SAMO sučelje (popis projekata, raspon, postavke). ──

export interface ProjectSummary {
  id: number;
  name: string;
  root_path: string;
  worktrees: number;
  last_refresh: number | null;
  last_commit: { sha: string; time: number; subject: string } | null;
  worst: Severity | null;
  signals: { info: number; warn: number; alert: number };
  error: string | null;
}

/// Birač raspona (6.1 u spec-u): četiri gotova presjeka ili vlastiti `since`/`until`.
export type Range =
  | { preset: 'all' | '7d' | '30d' | 'month' }
  | { preset: 'custom'; since: string; until: string };

export interface TrendPoint {
  taken_on: string;
  value: number;
  profile_changed: boolean;
}

// Četiri `data-theme` vrijednosti iz `src/styles/tokens.css` — zadana je „academic" (S-017).
export type Theme = 'academic' | 'chalk' | 'mint' | 'carbon';

export interface Settings {
  theme: Theme;
  lang: Lang;
  autostart: boolean;
  motion: boolean;
}

// Devet stavki lijevog izbornika (spec 6.2: Pregled + osam pogleda); imena prate ključeve `nav.*`
// u `src/lib/i18n/{hr,en}.json` tako da `t(\`nav.${view}\`)` uvijek pogodi postojeći natpis.
// `'settings'` je deseti, globalni pogled izvan popisa (M2/38): `Sidebar` ga crta zasebno, NIJE u
// `VIEWS` niže, jer radi i bez odabranog projekta (R22, spec §13.2).
export type View =
  | 'overview'
  | 'tempo'
  | 'kinds'
  | 'indicators'
  | 'phases'
  | 'diary'
  | 'deliveries'
  | 'visions'
  | 'docs'
  | 'settings';

export const VIEWS: readonly View[] = [
  'overview',
  'tempo',
  'kinds',
  'indicators',
  'phases',
  'diary',
  'deliveries',
  'visions',
  'docs',
];
