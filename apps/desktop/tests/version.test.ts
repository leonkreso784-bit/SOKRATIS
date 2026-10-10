// ZAŠTO OVAKO (cigla M2/37): verzija ima JEDAN izvor — `[workspace.package]` u korijenskom Cargo.toml.
// Tauri je čita odande kad `tauri.conf.json` nema `version`; package.json je samo zrcalo i ovaj test
// pada čim se raziđu (S-010 — jedna činjenica, jedno mjesto).
import { describe, expect, it } from 'vitest';
import cargo from '../../../Cargo.toml?raw';
import pkg from '../package.json';
import conf from '../src-tauri/tauri.conf.json';

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
