// ZAŠTO OVAKO (cigla M2/34 — TauriApi testiran bez Taurija i bez DOM-a; dopunjeno M2/38 — test za
// `setSetting` sad tvrdi i `motion`, PO TIPU vrijednosti, ne po ključu)
// `vi.mock` zamjenjuje `@tauri-apps/api/core`/`event` lažnim `invoke`/`listen` PRIJE nego se `api.ts`
// uveze; `vi.hoisted` diže `invokeMock`/`listenMock` iznad `vi.mock` poziva jer Vitest sam diže
// `vi.mock` na vrh datoteke (obični `const` bi ondje inače bio nedostupan). Testovi provjeravaju SAMO
// ugovor prema `commands.rs` (ime naredbe, imena argumenata, oblik tereta događaja) — pravu IPC vezu
// nad pravim prozorom provjerava ručna lista izvan vitesta (T34-dopuna, Korak 2).
import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { UnlistenFn } from '@tauri-apps/api/event';
import type { Report } from '../src/lib/types';

const { invokeMock, listenMock } = vi.hoisted(() => ({
  invokeMock: vi.fn(),
  listenMock: vi.fn(),
}));

vi.mock('@tauri-apps/api/core', () => ({ invoke: invokeMock }));
vi.mock('@tauri-apps/api/event', () => ({ listen: listenMock }));

import { createApi, MockApi, TauriApi } from '../src/lib/api';
import { api } from '../src/lib/api';
import { app, dismissError, loadReport } from '../src/lib/state.svelte';

beforeEach(() => {
  invokeMock.mockReset();
  listenMock.mockReset();
});

describe('TauriApi — ugovor prema commands.rs', () => {
  it('listProjects/getSettings/addProject zovu naredbe bez dodatnih argumenata', async () => {
    invokeMock.mockResolvedValue(null);
    const t = new TauriApi();
    await t.listProjects();
    expect(invokeMock).toHaveBeenCalledWith('list_projects');
    await t.getSettings();
    expect(invokeMock).toHaveBeenCalledWith('get_settings');
    const added = await t.addProject();
    expect(invokeMock).toHaveBeenCalledWith('add_project');
    expect(added).toBeNull();
  });

  it('getReport šalje { id, range } nepromijenjen', async () => {
    invokeMock.mockResolvedValue({});
    const t = new TauriApi();
    await t.getReport(3, { preset: '7d' });
    expect(invokeMock).toHaveBeenCalledWith('get_report', { id: 3, range: { preset: '7d' } });
  });

  it('getTrend šalje { id, metric, range }', async () => {
    invokeMock.mockResolvedValue([]);
    const t = new TauriApi();
    await t.getTrend(3, 'hours', { preset: 'all' });
    expect(invokeMock).toHaveBeenCalledWith('get_trend', { id: 3, metric: 'hours', range: { preset: 'all' } });
  });

  it('renameProject/removeProject šalju { id, ... }', async () => {
    invokeMock.mockResolvedValue(undefined);
    const t = new TauriApi();
    await t.renameProject(4, 'novo ime');
    expect(invokeMock).toHaveBeenCalledWith('rename_project', { id: 4, name: 'novo ime' });
    await t.removeProject(4);
    expect(invokeMock).toHaveBeenCalledWith('remove_project', { id: 4 });
  });

  it('setOverride šalje kind: null kad se override vraća na izračunato', async () => {
    invokeMock.mockResolvedValue(undefined);
    const t = new TauriApi();
    await t.setOverride(3, 'abc123', null);
    expect(invokeMock).toHaveBeenCalledWith('set_override', { id: 3, sha: 'abc123', kind: null });
    await t.setOverride(3, 'abc123', 'polish');
    expect(invokeMock).toHaveBeenCalledWith('set_override', { id: 3, sha: 'abc123', kind: 'polish' });
  });

  it('saveVisions šalje cijeli popis', async () => {
    invokeMock.mockResolvedValue(undefined);
    const t = new TauriApi();
    const visions = [{ title: 'x', source: 's', state: 'st', percent: null, note: '' }];
    await t.saveVisions(3, visions);
    expect(invokeMock).toHaveBeenCalledWith('save_visions', { id: 3, visions });
  });

  it('refresh bez id znači "svi projekti"; refresh(id) šalje taj id', async () => {
    invokeMock.mockResolvedValue(undefined);
    const t = new TauriApi();
    await t.refresh();
    expect(invokeMock).toHaveBeenCalledWith('refresh', { id: undefined });
    await t.refresh(9);
    expect(invokeMock).toHaveBeenCalledWith('refresh', { id: 9 });
  });

  it('setSetting("autostart"/"motion", true/false) šalje "on"/"off"; ostale postavke šalju tekst kakav jest', async () => {
    invokeMock.mockResolvedValue(undefined);
    const t = new TauriApi();
    await t.setSetting('autostart', true);
    expect(invokeMock).toHaveBeenNthCalledWith(1, 'set_setting', { key: 'autostart', value: 'on' });
    await t.setSetting('autostart', false);
    expect(invokeMock).toHaveBeenNthCalledWith(2, 'set_setting', { key: 'autostart', value: 'off' });
    await t.setSetting('motion', true);
    expect(invokeMock).toHaveBeenNthCalledWith(3, 'set_setting', { key: 'motion', value: 'on' });
    await t.setSetting('motion', false);
    expect(invokeMock).toHaveBeenNthCalledWith(4, 'set_setting', { key: 'motion', value: 'off' });
    await t.setSetting('theme', 'mint');
    expect(invokeMock).toHaveBeenNthCalledWith(5, 'set_setting', { key: 'theme', value: 'mint' });
  });

  it('onReportUpdated: listen uhvati handler, project_id iz tereta stiže do cb', () => {
    let handler: ((e: { payload: { project_id: number } }) => void) | undefined;
    listenMock.mockImplementation((_event: string, cb: typeof handler) => {
      handler = cb;
      return Promise.resolve(() => {});
    });
    const t = new TauriApi();
    const received: number[] = [];
    t.onReportUpdated((id) => received.push(id));
    expect(listenMock).toHaveBeenCalledWith('report_updated', expect.any(Function));
    handler?.({ payload: { project_id: 7 } });
    expect(received).toEqual([7]);
  });

  it('onSignalRaised: cijeli teret (project_id, rule, severity) stiže do cb', () => {
    let handler: ((e: { payload: { project_id: number; rule: string; severity: 'alert' } }) => void) | undefined;
    listenMock.mockImplementation((_event: string, cb: typeof handler) => {
      handler = cb;
      return Promise.resolve(() => {});
    });
    const t = new TauriApi();
    let received: unknown = null;
    t.onSignalRaised((e) => {
      received = e;
    });
    handler?.({ payload: { project_id: 5, rule: 'unmerged-branches', severity: 'alert' } });
    expect(received).toEqual({ project_id: 5, rule: 'unmerged-branches', severity: 'alert' });
  });

  it('odjava pozvana PRIJE nego se listen razriješi svejedno otkaže pretplatu', async () => {
    const fakeUnlisten = vi.fn();
    let resolveListen!: (fn: UnlistenFn) => void;
    listenMock.mockImplementation(() => new Promise<UnlistenFn>((resolve) => (resolveListen = resolve)));
    const t = new TauriApi();
    const off = t.onReportUpdated(() => {});
    off(); // odjava PRIJE nego se `listen()` razriješi
    expect(fakeUnlisten).not.toHaveBeenCalled();
    resolveListen(fakeUnlisten);
    await Promise.resolve();
    await Promise.resolve();
    expect(fakeUnlisten).toHaveBeenCalledTimes(1);
  });
});

describe('createApi', () => {
  it('u Nodeu (bez window) vraća MockApi i ne baca', () => {
    expect(() => createApi()).not.toThrow();
    expect(createApi()).toBeInstanceOf(MockApi);
  });
});

// ── state.svelte.ts: greška i "zadnji zahtjev pobjeđuje" ──────────────────────────────────────────
// `app`/`loadReport` uvezeni izravno (bez `vi.mock`) — singleton `api` (isti modul, gore NEMOKIRAN
// jer `vi.mock` gore cilja samo `@tauri-apps/api/*`) je u Nodeu `MockApi`; njegovu `getReport` ovdje
// privremeno zamijenimo lažnom da kontroliramo REDOSLIJED razrješenja dvaju poziva — isti obrazac
// kao `requestToken` u `Indicators.svelte` (dopuna T34, "zadnji zahtjev pobjeđuje").
function fakeReport(commits: number): Report {
  return {
    generated_at: 0,
    since: '2026-01-01',
    until: null,
    branch: 'main',
    scope: 'default',
    touched: { commits, lines: 0, files: 0, skipped_lines: 0, worktrees: 1, diaries: 1 },
    days: [],
    kinds: [],
    branches: [],
    commits: [],
    deliveries: [],
    indicators: [],
    phases: [],
    visions: [],
    vision_totals: [],
    docs: null,
    signals: [],
  };
}

describe('state.svelte — greška i utrka zahtjeva', () => {
  it('greška iz getReport (goli tekst, Rust Err(String)) završi u app.error', async () => {
    app.currentId = 1;
    const original = api.getReport;
    api.getReport = vi.fn().mockRejectedValue('repozitorij ne postoji');
    await loadReport();
    expect(app.error).toBe('repozitorij ne postoji');
    dismissError();
    expect(app.error).toBeNull();
    api.getReport = original;
  });

  it('zadnji zahtjev pobjeđuje: stariji odgovor koji stigne kasnije ne prepisuje noviji', async () => {
    app.currentId = 1;
    let resolveFirst!: (r: Report) => void;
    let resolveSecond!: (r: Report) => void;
    let call = 0;
    const original = api.getReport;
    api.getReport = vi.fn().mockImplementation(
      () =>
        new Promise<Report>((resolve) => {
          call += 1;
          if (call === 1) resolveFirst = resolve;
          else resolveSecond = resolve;
        }),
    );
    const firstLoad = loadReport(); // korisnik otvara raspon A
    await Promise.resolve();
    const secondLoad = loadReport(); // odmah mijenja na raspon B — drugi zahtjev je sad "najnoviji"
    await Promise.resolve();
    resolveSecond(fakeReport(2)); // B stigne prvi (brži odgovor)
    await secondLoad;
    resolveFirst(fakeReport(1)); // A stigne kasnije — MORA biti odbačen
    await firstLoad;
    expect(app.report?.touched.commits).toBe(2);
    api.getReport = original;
  });
});
