// ZAŠTO OVAKO (cigla 1.0.1/T67 — tipovi za notices.mjs)
// Isti razlog kao `contrast.d.mts`: `.mjs` bez `allowJs` ne nosi tipove, a test je `.ts` koji
// svelte-check provjerava. Ova datoteka opisuje OBLIK modula, ne mijenja izvršni kod.
export type LicenseFile = { name: string; text: string };
export type Component = {
  kind: 'crate' | 'npm';
  name: string;
  version: string;
  license: string;
  files: LicenseFile[];
};
export type NpmPackage = { name: string; version: string; path: string };

export function isLicenseFile(name: string): boolean;
export function normalizeSpdx(expr: string): string;
export function normalizeText(text: string): string;
export function parseCargoTree(stdout: string): string[];
export function npmPackages(lock: { packages?: Record<string, { version?: string; dev?: boolean }> }): NpmPackage[];
export function renderNotices(components: Component[]): string;
