// ZAŠTO OVAKO (cigla M2/37): verzija ima JEDAN izvor — `[workspace.package]` u korijenskom Cargo.toml.
// Tauri je čita odande kad `tauri.conf.json` nema `version`; package.json je samo zrcalo i ovaj test
// pada čim se raziđu (S-010 — jedna činjenica, jedno mjesto).
// ZAŠTO OVAKO (cigla mac/T82 — Mac paket): JSON nema komentara, pa razlog stoji ovdje. Tauri 2 na macOS-u
// spaja `tauri.macos.conf.json` preko glavnog (JSON Merge Patch): Mac gradi `.app` i `.dmg` za macOS 11+
// (prvi za Apple Silicon, S-043 O4), a identifikator se mijenja SAMO na Macu (O2) — `.app` na kraju
// sudara se s nastavkom paketa, dok Windows nadogradnja ovisi o starom `dev.sokratis.app`.
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
    bundle?: { targets?: unknown; macOS?: { minimumSystemVersion?: string } };
  };
  it('gradi .app i .dmg za macOS 11+', () => {
    expect(mac.bundle?.targets).toEqual(['app', 'dmg']);
    expect(mac.bundle?.macOS?.minimumSystemVersion).toBe('11.0');
  });
  it('identifikator je samo Macov: različit od glavnog i ne završava s .app', () => {
    expect(typeof mac.identifier).toBe('string');
    expect(mac.identifier).not.toBe(conf.identifier);
    expect(mac.identifier!.endsWith('.app')).toBe(false);
  });
  it('nema polje version (izvor je Cargo.toml)', () => {
    expect('version' in (mac as Record<string, unknown>)).toBe(false);
  });
});
