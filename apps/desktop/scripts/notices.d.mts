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
  /** nositelji prava iz metapodataka paketa; ispisuju se uz komponentu koja posuđuje tekst */
  holders?: string[];
};
export type BundledPackage = { name: string; path: string };
export type Banner = { name: string; pattern: RegExp };
export type Person = string | { name?: string; email?: string; url?: string };

export const BANNERS: Banner[];

export function isLicenseFile(name: string): boolean;
export function normalizeSpdx(expr: string): string;
export function normalizeText(text: string): string;
export function parseCargoTree(stdout: string): string[];
export function packagesFromSources(sources: string[]): BundledPackage[];
export function packagesFromBanners(text: string): BundledPackage[];
export function personNames(field: Person | Person[] | null | undefined): string[];
export function renderNotices(components: Component[]): string;
