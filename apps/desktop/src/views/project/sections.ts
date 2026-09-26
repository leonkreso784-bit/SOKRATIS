// ZAŠTO OVAKO (cigla M2/58 — popis sekcija na JEDNOM mjestu, S-010): skok-izbornik, sidra i naslovi
// čitaju isti niz; test veže naslove na oba rječnika (`tests/helpers.test.ts`). Šest bivših prikaznih
// pogleda (Tempo · Vrste rada · Pokazatelji · Faze · Isporuke · Dokumentacija) zadržavaju svoje stare
// `nav.*` ključeve kao naslove sekcija — samo `summary`/`branches` su novi (S-034).
export const SECTIONS = [
  'summary',
  'tempo',
  'branches',
  'kinds',
  'phases',
  'deliveries',
  'indicators',
  'docs',
] as const;

export type Section = (typeof SECTIONS)[number];

const KEYS: Record<Section, string> = {
  summary: 'section.summary',
  tempo: 'nav.tempo',
  branches: 'section.branches',
  kinds: 'nav.kinds',
  phases: 'nav.phases',
  deliveries: 'nav.deliveries',
  indicators: 'nav.indicators',
  docs: 'nav.docs',
};

export const sectionId = (s: Section): string => `sec-${s}`;
export const sectionKey = (s: Section): string => KEYS[s];
