// ZAŠTO OVAKO (cigla 1.0.1/T67 — obavijesti o licencama)
// Testira samo ČISTE funkcije iz `scripts/notices.mjs`: ulaz su gotovi nizovi i objekti (kao da ih
// je `notices-build.mjs` već pročitao s diska), izlaz je tekst ili iznimka. Zato test ne treba
// ni cargo ni node_modules — isti obrazac kao `contrast.test.ts`, koji mjeri tekst, ne datoteku.
// `toThrow(/ime/)` provjerava da pad IMENUJE krivca: brana koja samo kaže „nešto fali" ne pomaže.
import { describe, expect, it } from 'vitest';
import {
  isLicenseFile,
  normalizeSpdx,
  normalizeText,
  npmPackages,
  parseCargoTree,
  renderNotices,
  type Component,
} from '../scripts/notices.mjs';

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

  it('isti tekst licence ide jednom, s popisom svih komponenti koje ga nose', () => {
    const out = renderNotices([
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
    const out = renderNotices(items);
    expect(out).toBe(renderNotices([...items].reverse()));
    // verzije se uspoređuju kao brojevi: 0.2.0 prije 0.10.0 (kao tekst bi bilo obrnuto)
    const order = ['alfa 0.2.0', 'alfa 0.10.0', 'zeta 1.0.0', 'd3-array 3.2.4'].map((s) => out.indexOf('  ' + s + ' — '));
    expect(order.every((i) => i >= 0)).toBe(true);
    expect([...order].sort((a, b) => a - b)).toEqual(order);
  });

  it('komponenta bez vlastite datoteke dobiva uputu na tekst iste SPDX licence iz druge komponente', () => {
    const out = renderNotices([
      crate('davatelj', 'Apache-2.0 OR MIT', [{ name: 'LICENSE-MIT', text: MIT_A }, { name: 'LICENSE-APACHE', text: APACHE }]),
      crate('bez-datoteke', 'MIT/Apache-2.0', []),
    ]);
    expect(out).toContain('bez-datoteke 1.0.0 — MIT/Apache-2.0 (no licence file in the package; uses the licence text of davatelj 1.0.0)');
    expect(out).toContain('Used by: bez-datoteke 1.0.0, davatelj 1.0.0');
  });

  it('od više davatelja iste licence prednost ima onaj s najduljim zajedničkim početkom imena (ista obitelj paketa)', () => {
    const npm = (name: string, files: Component['files']): Component => ({ kind: 'npm', name, version: '2.0.0', license: 'MIT OR Apache-2.0', files });
    const out = renderNotices([
      crate('anyhow', 'MIT OR Apache-2.0', [{ name: 'LICENSE-MIT', text: MIT_A }]),
      npm('@tauri-apps/api', [{ name: 'LICENSE_MIT', text: 'MIT License\n\nCopyright (c) Tauri' }]),
      npm('@tauri-apps/plugin-dialog', []),
    ]);
    expect(out).toContain('@tauri-apps/plugin-dialog 2.0.0 — MIT OR Apache-2.0 (no licence file in the package; uses the licence text of @tauri-apps/api 2.0.0)');
  });

  it('komponenta bez datoteke i bez davatelja iste licence ruši izgradnju s njenim imenom', () => {
    expect(() =>
      renderNotices([crate('ima', 'MIT', [{ name: 'LICENSE', text: MIT_A }]), crate('siroce', 'MPL-2.0', [])]),
    ).toThrow(/siroce 1\.0\.0 \(MPL-2\.0\)/);
  });

  it('vlastiti crateovi sokratis-* se preskaču', () => {
    const out = renderNotices([crate('sokratis-core', 'LicenseRef-Sokratis', []), crate('alfa', 'MIT', [{ name: 'LICENSE', text: MIT_A }])]);
    expect(out).not.toContain('sokratis-core');
    expect(out).toContain('alfa 1.0.0 — MIT');
  });

  it('cargo tree: jedinstveni name@version, bez (*), (proc-macro) i putanje', () => {
    const stdout = 'sokratis-desktop v1.0.0 (C:\\x\\src-tauri)\nserde v1.0.228\nserde_derive v1.0.228 (proc-macro)\nserde v1.0.228 (*)\n\n';
    expect(parseCargoTree(stdout)).toEqual(['serde@1.0.228', 'serde_derive@1.0.228', 'sokratis-desktop@1.0.0']);
  });

  it('package-lock: samo paketi bez dev: true, ime iz zadnjeg node_modules/ u ključu', () => {
    const lock = {
      packages: {
        '': { name: 'sokratis-desktop', version: '1.0.0' },
        'node_modules/@tauri-apps/api': { version: '2.11.1', license: 'Apache-2.0 OR MIT' },
        'node_modules/vite': { version: '8.3.0', dev: true },
        'node_modules/a/node_modules/b': { version: '0.1.0' },
      },
    };
    expect(npmPackages(lock)).toEqual([
      { name: '@tauri-apps/api', version: '2.11.1', path: 'node_modules/@tauri-apps/api' },
      { name: 'b', version: '0.1.0', path: 'node_modules/a/node_modules/b' },
    ]);
  });
});
