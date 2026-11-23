// ZAŠTO OVAKO (cigla M2/37): verzija ima JEDAN izvor — `[workspace.package]` u korijenskom Cargo.toml.
// Tauri je čita odande kad `tauri.conf.json` nema `version`; package.json je samo zrcalo i ovaj test
// pada čim se raziđu (S-010 — jedna činjenica, jedno mjesto).
// ZAŠTO OVAKO (cigla mac/T82 — Mac paket): JSON nema komentara, pa razlog stoji ovdje. Tauri 2 na macOS-u
// spaja `tauri.macos.conf.json` preko glavnog (JSON Merge Patch): Mac gradi `.app` i `.dmg`. Minimum je
// macOS 13.3: WKWebView je sistemski WebKit; Tailwind v4 traži Safari 16.4, koji macOS ima u sustavu tek
// od 13.3 (Ventura). Identifikator se mijenja SAMO na Macu (S-043 O2) — `.app` na kraju sudara se s
// nastavkom paketa, dok Windows nadogradnja ovisi o starom `dev.sokratis.app`.
// (cigla mac/T87 — popis licenci po platformi, S-043 O6) Merge Patch spaja mapu `resources` ključ po
// ključ, a `null` BRIŠE ključ: Mac datoteka briše Windows izvor i pod istim imenom
// `THIRD-PARTY-NOTICES.txt` stavlja Mac popis. `LICENSE` se nasljeđuje iz glavne datoteke.
// (cigla mac/T88 — ad-hoc potpis, S-043 O3) `signingIdentity: "-"` tjera Tauri da ad-hoc potpiše CIJELI
// `.app` (Info.plist vezan, resursi zapečaćeni, identitet `dev.sokratis.desktop`), a ne samo Mach-O
// linkerom. Uz bundle id `dev.sokratis.desktop` macOS veže obavijesti; TCC (pristup mapama) kod ad-hoc
// potpisa veže dopuštenje uz cdhash (`codesign -d -r-` → `designated => cdhash H"…"`), pa se ono
// vjerojatno traži ponovno nakon svakog builda. Uz `signingIdentity` Tauri sam uključuje hardened runtime
// (`hardened_runtime` je u tauri-utils zadano true), bez entitlementa. Glavni `tauri.conf.json` je
// zajednička osnova (Windows ga koristi bez Mac zakrpe), pa to polje nema.
import { describe, expect, it } from 'vitest';
import cargo from '../../../Cargo.toml?raw';
import pkg from '../package.json';
import conf from '../src-tauri/tauri.conf.json';
import macConf from '../src-tauri/tauri.macos.conf.json';

describe('verzija', () => {
  it('Cargo.toml je izvor, package.json zrcalo, tauri.conf.json je nema', () => {
    const m = cargo.match(/\[workspace\.package\][^[]*?\bversion\s*=\s*"([^"]+)"/s);
    expect(m).not.toBeNull();
    expect(pkg.version).toBe(m![1]);
    expect('version' in (conf as Record<string, unknown>)).toBe(false);
  });
  it('instalater je uključen i gradi samo NSIS', () => {
    const b = (conf as { bundle: { active: boolean; targets: unknown } }).bundle;
    expect(b.active).toBe(true);
    expect(b.targets).toEqual(['nsis']);
  });
  // Dopunjeno 1.0.1/T67: licence putuju uz instalater. Oblik mape (izvor → ime u mapi instalacije)
  // jer bi niz putanja s `../` Tauri spremio pod `_up_/`, a LICENSE treba nastavak `.txt` za Windows.
  it('instalater nosi THIRD-PARTY-NOTICES.txt i LICENSE.txt u mapu instalacije', () => {
    const r = (conf as { bundle: { resources?: Record<string, string> } }).bundle.resources;
    expect(r).toEqual({
      '../../../THIRD-PARTY-NOTICES.txt': 'THIRD-PARTY-NOTICES.txt',
      '../../../LICENSE': 'LICENSE.txt',
    });
  });
});

describe('Mac paket (tauri.macos.conf.json)', () => {
  const mac = macConf as {
    identifier?: string;
    bundle?: {
      targets?: unknown;
      macOS?: { minimumSystemVersion?: string; signingIdentity?: string };
      resources?: Record<string, string | null>;
    };
  };
  it('gradi .app i .dmg za macOS 13.3+', () => {
    expect(mac.bundle?.targets).toEqual(['app', 'dmg']);
    expect(mac.bundle?.macOS?.minimumSystemVersion).toBe('13.3');
  });
  it('identifikator je samo Macov: različit od glavnog i ne završava s .app', () => {
    expect(typeof mac.identifier).toBe('string');
    expect(mac.identifier).not.toBe(conf.identifier);
    expect(mac.identifier!.endsWith('.app')).toBe(false);
  });
  it('nema polje version (izvor je Cargo.toml)', () => {
    expect('version' in (mac as Record<string, unknown>)).toBe(false);
  });
  it('.app nosi Mac popis licenci pod imenom THIRD-PARTY-NOTICES.txt; Windows popis je obrisan (null)', () => {
    expect(mac.bundle?.resources).toEqual({
      '../../../THIRD-PARTY-NOTICES.txt': null,
      '../../../THIRD-PARTY-NOTICES-macos.txt': 'THIRD-PARTY-NOTICES.txt',
    });
    // nakon spajanja (RFC 7396) ostaju Mac popis i LICENSE.txt iz glavne datoteke
    const base = (conf as { bundle: { resources: Record<string, string> } }).bundle.resources;
    const merged: Record<string, string> = { ...base };
    for (const [k, v] of Object.entries(mac.bundle?.resources ?? {})) {
      if (v === null) delete merged[k];
      else merged[k] = v;
    }
    expect(merged).toEqual({
      '../../../LICENSE': 'LICENSE.txt',
      '../../../THIRD-PARTY-NOTICES-macos.txt': 'THIRD-PARTY-NOTICES.txt',
    });
  });
  it('cijeli paket je ad-hoc potpisan (signingIdentity "-"); glavna datoteka nema macOS.signingIdentity', () => {
    expect(mac.bundle?.macOS?.signingIdentity).toBe('-');
    const base = (conf as { bundle: { macOS?: Record<string, unknown> } }).bundle;
    expect(base.macOS === undefined || !('signingIdentity' in base.macOS)).toBe(true);
  });
});
