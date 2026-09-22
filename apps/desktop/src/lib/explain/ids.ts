// ZAŠTO OVAKO (cigla M2/41 — jedini popis objašnjivih id-eva)
// `EXPLAIN_IDS` je JEDNO mjesto koje zna koji `id` ima karticu s objašnjenjem (S-010) — `explain.test.ts`
// ga veže na oba rječnika u oba smjera, a `Indicators.svelte` iz njega čita `Explainable id={`ind.${ind.id}`}`.
// `explainKeys` je čista funkcija (isti obrazac kao `translate()` u `i18n/t.ts`): iz jednog `id`-a izvodi tri
// ključa rječnika umjesto da ih netko ručno slaže na tri mjesta i jednom pogriješi nastavak.
// Dopunjeno M2/42 — 19 novih id-eva pokriva preostalih devet pogleda i traku signala (spec §13.4): svaki
// graf i svaka brojka s vlastitim mjerenjem dobiva karticu, ne svaka ćelija tablice (R26/R28 iz dopune).
export const EXPLAIN_IDS: readonly string[] = [
  'ind.working_days',
  'ind.commits',
  'ind.commits_per_day',
  'ind.deliveries',
  'ind.deliveries_per_day',
  'ind.hours',
  'ind.commits_per_hour',
  'ind.lines_changed',
  'ind.test_lines',
  'ind.test_share',
  'ind.deploys',
  'ind.debugging_commits',
  'ind.debugging_share',
  'ind.docs_share',
  'ind.ci_fixes',
  'ind.owner_driven_deliveries',
  'ind.closed_phases_in_range',
  'ind.closed_phase_avg_days',
  'overview.signals',
  'overview.last_commit',
  'overview.worktrees',
  'tempo.bars',
  'tempo.cumulative',
  'tempo.hours',
  'kinds.ring',
  'kinds.share',
  'phases.days',
  'phases.commits',
  'phases.bricks_per_day',
  'diary.kind',
  'deliveries.list',
  'visions.totals',
  'visions.percent',
  'docs.score',
  'docs.lag',
  'docs.findings',
  'signals.severity',
];

export function explainKeys(id: string): { what: string; how: string; read: string } {
  return { what: `explain.${id}.what`, how: `explain.${id}.how`, read: `explain.${id}.read` };
}
