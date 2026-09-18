// ZAŠTO OVAKO (cigla M2/21 — brana jednakih ključeva u `npm run check`)
// Rječnici su izvor istine za natpise (S-021) — brana ih učita i usporedi ključeve, jer natpis koji
// postoji samo u jednom jeziku bi u drugom tiho pao na ključ (vidljivo, ali neplanirano).
import { readFileSync } from 'node:fs';

const load = (f) => JSON.parse(readFileSync(new URL(`../src/lib/i18n/${f}`, import.meta.url), 'utf8'));
const hr = load('hr.json'),
  en = load('en.json');
const onlyHr = Object.keys(hr).filter((k) => !(k in en)),
  onlyEn = Object.keys(en).filter((k) => !(k in hr));
const empty = [...Object.entries(hr), ...Object.entries(en)].filter(([, v]) => !String(v).trim()).map(([k]) => k);
if (onlyHr.length || onlyEn.length || empty.length) {
  console.error('check:i18n — samo hr:', onlyHr, ' samo en:', onlyEn, ' prazno:', empty);
  process.exit(1);
}
console.log(`check:i18n — ${Object.keys(hr).length} ključeva, hr = en`);
