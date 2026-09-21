// ZAŠTO OVAKO (cigla M2/20 — tipovi za contrast.mjs)
// `.mjs` bez `allowJs` u tsconfigu (izvan vlasništva ove cigle) ne nosi tipove — svelte-check bez
// ovoga baca TS7016 na svaki uvoz iz `contrast.mjs`. Ručna `.d.mts` uz izvornik je službeni TS
// mehanizam za to (isti naziv datoteke, sufiks `.d.mts` za `.mjs` modul) — opisuje OBLIK, ne mijenja
// izvršni kod ni tsconfig.json.
export type Rgb = [number, number, number];
export type Failure = {
  theme: string;
  rule: string;
  token: string;
  on: string;
  value: number;
  min: number;
};

export const TEXT_MIN: number;
export const UI_MIN: number;
export const HUE_MIN: number;

export function parseHex(v: string): Rgb | null;
export function luminance(rgb: Rgb): number;
export function contrast(a: Rgb, b: Rgb): number;
export function hue(rgb: Rgb): number;
export function stripComments(css: string): string;
export function themes(css: string): Record<string, Record<string, string>>;
export function checkTokens(css: string): Failure[];
