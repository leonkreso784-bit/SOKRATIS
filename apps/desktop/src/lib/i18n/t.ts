// ZAŠTO OVAKO (cigla M2/21 — prijevod bez biblioteke)
// Čista funkcija (rječnik + ključ → tekst) živi odvojeno od Svelte stanja da je vitest testira bez
// runes. Nepoznat ključ se vraća kakav jest: prazan natpis bi bio tiha greška, ključ na ekranu je glasna.
export type Dict = Record<string, string>;
export function translate(dict: Dict, key: string, params?: Record<string, string | number>): string {
  const s = dict[key] ?? key;
  return s.replace(/\{(\w+)\}/g, (_, k: string) => (params && k in params ? String(params[k]) : `{${k}}`));
}
