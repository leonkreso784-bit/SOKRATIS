// ZAŠTO OVAKO (cigla 1.0.1/T67 — obavijesti o licencama)
// Testira samo ČISTE funkcije iz `scripts/notices.mjs`: ulaz su gotovi nizovi i objekti (kao da ih
// je `notices-build.mjs` već pročitao s diska), izlaz je tekst ili iznimka. Zato test ne treba
// ni cargo ni node_modules — isti obrazac kao `contrast.test.ts`, koji mjeri tekst, ne datoteku.
// `toThrow(/ime/)` provjerava da pad IMENUJE krivca: brana koja samo kaže „nešto fali" ne pomaže.
// (cigla mac/T87) Tablica ciljeva `NOTICE_TARGETS` i `planTargets(host)` su čisti podaci i funkcija:
// test kaže koji se popis mjeri na kojem domaćinu bez pokretanja carga na tom domaćinu.
import { describe, expect, it } from 'vitest';
import {
  isLicenseFile,
  normalizeSpdx,
  normalizeText,
  BANNERS,
  packagesFromBanners,
  packagesFromSources,
  parseCargoTree,
  personNames,
  NOTICE_TARGETS,
  planTargets,
  renderNotices,
  targetCrates,
  type Component,
} from '../scripts/notices.mjs';

const WINDOWS = 'x86_64-pc-windows-msvc';
/** Postojeći testovi teksta ne ovise o cilju; zaglavlje s ciljem ima svoj test niže. */
const render = (components: Component[]) => renderNotices(components, WINDOWS);

const MIT_A = 'MIT License\n\nCopyright (c) Ana\n\nPermission is hereby granted, free of charge...';
const APACHE = 'Apache License\nVersion 2.0, January 2004\n...';

const crate = (name: string, license: string, files: Component['files'], version = '1.0.0'): Component => ({
  kind: 'crate',
  name,
  version,
  license,
  files,
});

describe('obavijesti o licencama — čiste funkcije', () => {
  it('prepoznaje datoteke licence, a SPDX-metapodatke i README ne', () => {
    for (const n of ['LICENSE', 'LICENSE-MIT', 'license-apache-2.0', 'LICENCE', 'COPYING', 'NOTICE', 'UNLICENSE', 'LICENSE.md']) {
      expect(isLicenseFile(n), n).toBe(true);
    }
    for (const n of ['LICENSE.spdx', 'README.md', 'Cargo.toml', 'src']) expect(isLicenseFile(n), n).toBe(false);
  });

  it('SPDX izraz: stara kosa crta = OR, redoslijed alternativa ne mijenja licencu', () => {
    expect(normalizeSpdx('MIT/Apache-2.0')).toBe(normalizeSpdx('Apache-2.0 OR MIT'));
    expect(normalizeSpdx('Apache-2.0 / MIT')).toBe(normalizeSpdx('MIT OR Apache-2.0'));
    expect(normalizeSpdx('MIT')).not.toBe(normalizeSpdx('MIT OR Apache-2.0'));
  });

  it('tekst: CRLF i prazni rubovi se ne broje kao razlika', () => {
    expect(normalizeText('\r\nA  \r\nB\r\n\r\n')).toBe('A\nB');
  });

  it('tekst: BOM na početku se skida', () => {
    expect(normalizeText('﻿MIT License')).toBe('MIT License');
  });

  it('isti tekst licence ide jednom, s popisom svih komponenti koje ga nose', () => {
    const out = render([
      crate('beta', 'MIT', [{ name: 'LICENSE', text: MIT_A }]),
      crate('alfa', 'MIT', [{ name: 'LICENSE-MIT', text: MIT_A.replace(/\n/g, '\r\n') + '\r\n' }]),
    ]);
    expect(out.split('Copyright (c) Ana').length - 1).toBe(1);
    expect(out).toContain('Used by: alfa 1.0.0, beta 1.0.0');
  });

  it('popis je sortiran (crateovi pa npm, po imenu pa verziji) i isti za svaki redoslijed ulaza', () => {
    const items: Component[] = [
      { kind: 'npm', name: 'd3-array', version: '3.2.4', license: 'ISC', files: [{ name: 'LICENSE', text: 'ISC A' }] },
      crate('zeta', 'MIT', [{ name: 'LICENSE', text: MIT_A }]),
      crate('alfa', 'Apache-2.0', [{ name: 'LICENSE', text: APACHE }], '0.2.0'),
      crate('alfa', 'Apache-2.0', [{ name: 'LICENSE', text: APACHE }], '0.10.0'),
    ];
    const out = render(items);
    expect(out).toBe(render([...items].reverse()));
    // verzije se uspoređuju kao brojevi: 0.2.0 prije 0.10.0 (kao tekst bi bilo obrnuto)
    const order = ['alfa 0.2.0', 'alfa 0.10.0', 'zeta 1.0.0', 'd3-array 3.2.4'].map((s) => out.indexOf('  ' + s + ' — '));
    expect(order.every((i) => i >= 0)).toBe(true);
    expect([...order].sort((a, b) => a - b)).toEqual(order);
  });

  it('komponenta bez vlastite datoteke dobiva uputu na tekst iste SPDX licence iz druge komponente', () => {
    const out = render([
      crate('davatelj', 'Apache-2.0 OR MIT', [{ name: 'LICENSE-MIT', text: MIT_A }, { name: 'LICENSE-APACHE', text: APACHE }]),
      { ...crate('bez-datoteke', 'MIT/Apache-2.0', []), holders: ['Ivo Ivić', 'Ana Anić'] },
    ]);
    expect(out).toContain(
      'bez-datoteke 1.0.0 — MIT/Apache-2.0 (no licence file in the package; licence terms as in the text of davatelj 1.0.0; copyright: Ivo Ivić, Ana Anić)',
    );
    expect(out).toContain('Used by: bez-datoteke 1.0.0, davatelj 1.0.0');
  });

  it('od više davatelja iste licence prednost ima onaj s najduljim zajedničkim početkom imena (ista obitelj paketa)', () => {
    const npm = (name: string, files: Component['files']): Component => ({ kind: 'npm', name, version: '2.0.0', license: 'MIT OR Apache-2.0', files });
    const out = render([
      crate('anyhow', 'MIT OR Apache-2.0', [{ name: 'LICENSE-MIT', text: MIT_A }]),
      npm('@tauri-apps/api', [{ name: 'LICENSE_MIT', text: 'MIT License\n\nCopyright (c) Tauri' }]),
      npm('@tauri-apps/plugin-dialog', []),
    ]);
    // bez `holders` (ni autora u metapodacima) to se kaže izrijekom — ne izmišlja se nositelj prava
    expect(out).toContain(
      '@tauri-apps/plugin-dialog 2.0.0 — MIT OR Apache-2.0 (no licence file in the package; licence terms as in the text of @tauri-apps/api 2.0.0; copyright holders not stated in package metadata)',
    );
  });

  it('komponenta bez datoteke i bez davatelja iste licence ruši izgradnju s njenim imenom', () => {
    expect(() =>
      render([crate('ima', 'MIT', [{ name: 'LICENSE', text: MIT_A }]), crate('siroce', 'MPL-2.0', [])]),
    ).toThrow(/siroce 1\.0\.0 \(MPL-2\.0\)/);
  });

  it('vlastiti crateovi sokratis-* se preskaču', () => {
    const out = render([crate('sokratis-core', 'LicenseRef-Sokratis', []), crate('alfa', 'MIT', [{ name: 'LICENSE', text: MIT_A }])]);
    expect(out).not.toContain('sokratis-core');
    expect(out).toContain('alfa 1.0.0 — MIT');
  });

  it('cargo tree: jedinstveni name@version, bez (*), (proc-macro) i putanje', () => {
    const stdout = 'sokratis-desktop v1.0.0 (C:\\x\\src-tauri)\nserde v1.0.228\nserde_derive v1.0.228 (proc-macro)\nserde v1.0.228 (*)\n\n';
    expect(parseCargoTree(stdout)).toEqual(['serde@1.0.228', 'serde_derive@1.0.228', 'sokratis-desktop@1.0.0']);
  });

  it('cilj: crate samo-domaćina (ovisnost proc-makroa, npr. swift-rs na Macu) otpada', () => {
    const tree = ['base64@0.21.7', 'serde@1.0.228', 'swift-rs@1.0.7'];
    const metadata = new Set(['serde@1.0.228', 'windows-sys@0.59.0']);
    expect(targetCrates(tree, metadata)).toEqual(['serde@1.0.228']);
  });

  it('cilj: crate prisutan i u stablu i u metapodacima cilja ostaje; višak metapodataka ne ulazi', () => {
    const tree = ['serde@1.0.228', 'serde_derive@1.0.228'];
    const metadata = ['serde_derive@1.0.228', 'serde@1.0.228', 'nekoristen@2.0.0'];
    expect(targetCrates(tree, metadata)).toEqual(['serde@1.0.228', 'serde_derive@1.0.228']);
  });

  it('cilj: izlaz je sortiran kao parseCargoTree i ne ovisi o poretku ulaza', () => {
    const metadata = new Set(['Zeta@1.0.0', 'alfa@1.0.0', 'beta@0.10.0', 'beta@0.2.0']);
    const a = targetCrates(['beta@0.2.0', 'alfa@1.0.0', 'Zeta@1.0.0', 'beta@0.10.0', 'alfa@1.0.0'], metadata);
    const b = targetCrates(['beta@0.10.0', 'Zeta@1.0.0', 'beta@0.2.0', 'alfa@1.0.0'], [...metadata].reverse());
    expect(a).toEqual(['Zeta@1.0.0', 'alfa@1.0.0', 'beta@0.10.0', 'beta@0.2.0']);
    expect(b).toEqual(a);
  });

  it('sourcemap: ime paketa iz node_modules/ (i @scope/ime, i ugniježđeni paket), vlastiti kod i virtualni moduli otpadaju', () => {
    const sources = [
      '../../../../Documents/x/apps/desktop/node_modules/svelte/src/internal/client/index.js',
      '../../node_modules/svelte/src/index-client.js',
      '../../node_modules/@tauri-apps/api/core.js',
      '..\\..\\node_modules\\clsx\\dist\\clsx.mjs',
      '../../node_modules/a/node_modules/b/x.js',
      '../../src/main.ts',
      '\0vite/preload-helper.js',
    ];
    expect(packagesFromSources(sources)).toEqual([
      { name: '@tauri-apps/api', path: 'node_modules/@tauri-apps/api' },
      { name: 'b', path: 'node_modules/a/node_modules/b' },
      { name: 'clsx', path: 'node_modules/clsx' },
      { name: 'svelte', path: 'node_modules/svelte' },
    ]);
  });

  it('banner u izgrađenom CSS-u imenuje paket koji sourcemap ne vidi (tailwindcss)', () => {
    expect(BANNERS.map((b) => b.name)).toContain('tailwindcss');
    const css = '/*! tailwindcss v4.3.3 | MIT License | https://tailwindcss.com */\n@layer theme;';
    expect(packagesFromBanners(css)).toEqual([{ name: 'tailwindcss', path: 'node_modules/tailwindcss' }]);
    expect(packagesFromBanners('/* tailwindcss bez uskličnika nije banner */ .a{}')).toEqual([]);
  });

  it('nositelji prava iz metapodataka: niz ili objekt, bez e-pošte i poveznice; ničega → prazno', () => {
    expect(personNames('Ana Anić <ana@example.com> (https://example.com)')).toEqual(['Ana Anić']);
    expect(personNames({ name: 'Ivo', email: 'ivo@example.com' })).toEqual(['Ivo']);
    expect(personNames(['A <a@example.com>', { name: 'B' }, '', { email: 'samo@example.com' }])).toEqual(['A', 'B']);
    expect(personNames(undefined)).toEqual([]);
  });
});

describe('obavijesti o licencama — ciljevi po platformi (mac/T87)', () => {
  const MIT = crate('alfa', 'MIT', [{ name: 'LICENSE', text: MIT_A }]);

  it('dva cilja: Windows u THIRD-PARTY-NOTICES.txt, macOS (aarch64) u THIRD-PARTY-NOTICES-macos.txt', () => {
    expect(NOTICE_TARGETS.map((t) => [t.name, t.triple, t.file])).toEqual([
      ['Windows', 'x86_64-pc-windows-msvc', 'THIRD-PARTY-NOTICES.txt'],
      ['macOS', 'aarch64-apple-darwin', 'THIRD-PARTY-NOTICES-macos.txt'],
    ]);
  });

  it('na Macu se mjere oba popisa, ništa se ne preskače', () => {
    const plan = planTargets('darwin');
    expect(plan.measure.map((t) => t.file)).toEqual(['THIRD-PARTY-NOTICES.txt', 'THIRD-PARTY-NOTICES-macos.txt']);
    expect(plan.skip).toEqual([]);
  });

  it('na Windowsu i Linuxu se mjeri samo Windows popis; Mac popis se preskače (proc-makroi domaćina)', () => {
    for (const host of ['win32', 'linux']) {
      const plan = planTargets(host);
      expect(plan.measure.map((t) => t.file), host).toEqual(['THIRD-PARTY-NOTICES.txt']);
      expect(plan.skip.map((t) => t.file), host).toEqual(['THIRD-PARTY-NOTICES-macos.txt']);
    }
  });

  it('zaglavlje imenuje svoj cilj; Windows tekst je isti kao prije cigle', () => {
    const win = renderNotices([MIT], 'x86_64-pc-windows-msvc');
    const mac = renderNotices([MIT], 'aarch64-apple-darwin');
    expect(win).toContain('compiled into Sokratis for x86_64-pc-windows-msvc (cargo tree, Cargo.lock). npm packages: those\n');
    expect(mac).toContain('compiled into Sokratis for aarch64-apple-darwin (cargo tree, Cargo.lock).');
    expect(mac).not.toContain('x86_64-pc-windows-msvc');
    expect(mac.replace('aarch64-apple-darwin', 'x86_64-pc-windows-msvc')).toBe(win);
  });

  it('bez cilja nema teksta — zaglavlje ne smije šutke tvrditi krivi cilj', () => {
    expect(() => renderNotices([MIT], '')).toThrow(/cilj/);
  });
});
