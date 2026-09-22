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
});
