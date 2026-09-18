// ZAŠTO OVAKO (cigla M2/21 — jezik kao Svelte 5 stanje)
// `$state` izvan komponente treba nastavak `.svelte.ts` da ga kompajler prepozna kao rune-modul;
// zato ova tanka ljuska (jezik + `t()`) živi odvojeno od `t.ts`, koji ostaje čista funkcija za vitest.
import hr from './hr.json';
import en from './en.json';
import { translate, type Dict } from './t';

export type Lang = 'hr' | 'en';
const dicts: Record<Lang, Dict> = { hr, en };
let current = $state<Lang>('hr');

export function getLang(): Lang {
  return current;
}
export function setLang(l: Lang): void {
  current = l;
  document.documentElement.lang = l;
}
export function t(key: string, params?: Record<string, string | number>): string {
  return translate(dicts[current], key, params);
}
