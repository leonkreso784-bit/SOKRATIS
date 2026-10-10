// ZAŠTO OVAKO (cigla 1.0.1/T67 — obavijesti o licencama, čiste funkcije)
// Modul ne čita disk i ne zove procese: prima tekst (izlaz `cargo tree`, package-lock kao objekt,
// već pročitane datoteke licence) i vraća tekst. Zato ga vitest mjeri bez carga i node_modules, a sav
// I/O živi u `notices-build.mjs` — ista podjela kao `sokratis-core` / `sokratis-io` (S-002).
// Isti tekst licence ide jednom: `Map` čiji je ključ tekst bez razlika u razmacima, a vrijednost
// pamti sve komponente koje ga nose. Sortira se običnom usporedbom znakova (ne `localeCompare`),
// jer bi jezik sustava inače mijenjao redoslijed — a `notices:check` traži isti izlaz na svakom stroju.

/** Imena datoteka koje nose tekst licence; `LICENSE.spdx` je strojni SPDX-zapis, ne tekst licence. */
const LICENSE_FILE = /^(LICEN[CS]E|COPYING|NOTICE|UNLICENSE)/i;
const OWN_PREFIX = 'sokratis-';
const KIND_ORDER = { crate: 0, npm: 1 };

export function isLicenseFile(name) {
  return LICENSE_FILE.test(name) && !/\.spdx$/i.test(name);
}

/**
 * SPDX izraz u obliku po kojem se dvije komponente mogu usporediti: stara kosa crta (`MIT/Apache-2.0`)
 * znači OR, a redoslijed alternativa ne mijenja licencu. Izraz sa zagradama ostaje kakav jest (osim
 * razmaka) — puni SPDX parser nije potreban za usporedbu „iste licence".
 */
export function normalizeSpdx(expr) {
  const s = String(expr ?? '').trim().replace(/\s*\/\s*/g, ' OR ').replace(/\s+/g, ' ');
  if (s.includes('(')) return s;
  return s
    .split(' OR ')
    .map((alt) => alt.split(' AND ').sort().join(' AND '))
    .sort()
    .join(' OR ');
}

/** Tekst licence bez BOM-a, s LF krajevima redaka, bez razmaka na kraju retka i praznih rubova. */
export function normalizeText(text) {
  return String(text)
    .replace(/^﻿/, '')
    .replace(/\r\n?/g, '\n')
    .split('\n')
    .map((line) => line.trimEnd())
    .join('\n')
    .replace(/^\n+|\n+$/g, '');
}

const compareText = (a, b) => (a < b ? -1 : a > b ? 1 : 0);

/** Verzije dio po dio: brojevi kao brojevi (0.2.0 prije 0.10.0), ostalo kao tekst. */
function compareVersions(a, b) {
  const pa = String(a).split(/[.+-]/);
  const pb = String(b).split(/[.+-]/);
  for (let i = 0; i < Math.min(pa.length, pb.length); i++) {
    const x = pa[i];
    const y = pb[i];
    const d = /^\d+$/.test(x) && /^\d+$/.test(y) ? Number(x) - Number(y) : compareText(x, y);
    if (d !== 0) return d;
  }
  return pa.length - pb.length;
}

function compareComponents(a, b) {
  return KIND_ORDER[a.kind] - KIND_ORDER[b.kind] || compareText(a.name, b.name) || compareVersions(a.version, b.version);
}

const label = (c) => `${c.name} ${c.version}`;

/** Duljina zajedničkog početka dvaju imena (`@tauri-apps/plugin-dialog` i `@tauri-apps/api` → 12). */
function commonPrefix(a, b) {
  let i = 0;
  while (i < a.length && i < b.length && a[i] === b[i]) i++;
  return i;
}

/**
 * Od kandidata iste SPDX licence (već sortiranih) bira onog čije ime najdulje dijeli početak s
 * komponentom — paket iz iste obitelji obično nosi i istog nositelja autorskog prava; pri
 * izjednačenju pobjeđuje prvi po redoslijedu. Prazan popis → `undefined`.
 */
function pickDonor(c, candidates) {
  let best;
  let bestLen = -1;
  for (const d of candidates) {
    const len = commonPrefix(c.name, d.name);
    if (len > bestLen) {
      best = d;
      bestLen = len;
    }
  }
  return best;
}

/** Izlaz `cargo tree --prefix none`: jedinstveni `ime@verzija`, sortirano; `(*)`, `(proc-macro)` i putanja otpadaju. */
export function parseCargoTree(stdout) {
  const out = new Set();
  for (const line of String(stdout).split(/\r?\n/)) {
    const m = line.trim().match(/^(\S+) v(\S+)/);
    if (m) out.add(`${m[1]}@${m[2]}`);
  }
  return [...out].sort(compareText);
}

/** Paketi iz package-lock.json koji NISU `dev: true` (oni ulaze u izgrađeno sučelje); korijen `""` nije paket. */
export function npmPackages(lock) {
  const marker = 'node_modules/';
  return Object.entries(lock?.packages ?? {})
    .filter(([path, p]) => path !== '' && !p.dev)
    .map(([path, p]) => ({ name: path.slice(path.lastIndexOf(marker) + marker.length), version: p.version, path }))
    .sort((a, b) => compareText(a.path, b.path));
}

/**
 * Cijeli THIRD-PARTY-NOTICES.txt iz popisa komponenti. Komponenta bez vlastite datoteke licence
 * dobiva tekstove prve (po redoslijedu) komponente iste SPDX licence koja datoteku ima; ako takve
 * nema, funkcija baca grešku s imenima — tekst licence se ne izmišlja.
 */
export function renderNotices(components) {
  const list = components.filter((c) => !c.name.startsWith(OWN_PREFIX)).sort(compareComponents);

  // 1. Tekstovi iz vlastitih datoteka; ključ je tekst bez razlika u razmacima.
  const texts = new Map();
  const ownKeys = new Map();
  for (const c of list) {
    const keys = [];
    for (const f of [...c.files].sort((a, b) => compareText(a.name, b.name))) {
      const text = normalizeText(f.text);
      if (!text) continue;
      const key = text.replace(/\s+/g, ' ');
      if (!texts.has(key)) texts.set(key, { text, source: `${label(c)} / ${f.name}`, users: [] });
      if (!keys.includes(key)) keys.push(key);
    }
    ownKeys.set(c, keys);
  }

  // 2. Komponente bez datoteke posuđuju tekstove komponente iste SPDX licence.
  const keysOf = new Map(ownKeys);
  const borrowedFrom = new Map();
  const missing = [];
  for (const c of list) {
    if (ownKeys.get(c).length) continue;
    const spdx = normalizeSpdx(c.license);
    const donor = pickDonor(
      c,
      list.filter((d) => ownKeys.get(d).length && normalizeSpdx(d.license) === spdx),
    );
    if (!donor) {
      missing.push(`${label(c)} (${c.license || 'licenca nije navedena'})`);
      continue;
    }
    keysOf.set(c, ownKeys.get(donor));
    borrowedFrom.set(c, donor);
  }
  if (missing.length) {
    throw new Error(
      `notices: ${missing.length} komponenti bez datoteke licence i bez druge komponente iste SPDX licence — ` +
        `tekst se ne izmišlja: ${missing.join(', ')}`,
    );
  }

  for (const c of list) for (const key of keysOf.get(c)) texts.get(key).users.push(label(c));

  const line = (c) => {
    const donor = borrowedFrom.get(c);
    const note = donor ? ` (no licence file in the package; uses the licence text of ${label(donor)})` : '';
    return `  ${label(c)} — ${c.license}${note}`;
  };
  const crates = list.filter((c) => c.kind === 'crate');
  const npm = list.filter((c) => c.kind === 'npm');
  const all = [...texts.values()];

  const out = [
    'THIRD-PARTY NOTICES',
    '===================',
    '',
    'Sokratis includes the third-party components listed below. Their licences ask that their',
    'notices travel with every copy of Sokratis; this file carries them. Sokratis itself is',
    'licensed separately: see LICENSE.txt next to this file.',
    '',
    'Generated by apps/desktop/scripts/notices-build.mjs (npm run notices) from Cargo.lock',
    '(Rust crates compiled into Sokratis for x86_64-pc-windows-msvc) and package-lock.json',
    '(npm packages bundled into its user interface). Do not edit by hand.',
    '',
    `Components: ${list.length} (Rust crates: ${crates.length}, npm packages: ${npm.length}). Licence texts: ${all.length}.`,
    '',
    'RUST CRATES',
    '-----------',
    ...crates.map(line),
    '',
    'NPM PACKAGES',
    '------------',
    ...npm.map(line),
    '',
    'LICENCE TEXTS',
    '-------------',
  ];
  all.forEach((t, i) => {
    out.push('', `======== [${i + 1}/${all.length}] ========`, `Used by: ${t.users.join(', ')}`, `Source: ${t.source}`, '', t.text);
  });
  return out.join('\n') + '\n';
}
