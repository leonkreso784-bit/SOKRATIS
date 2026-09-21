<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — birač raspona kao diskriminirana unija)
  // `Range` je unija po `preset` (`types.ts`): TS suzi tip čim provjerimo `app.range.preset ===
  // 'custom'`, pa `since`/`until` postoje SAMO ondje gdje ih Rust stvarno očekuje — nema polja koja
  // "mogu, ali ne moraju" biti prisutna. Svaka promjena odmah traži nov izvještaj (`loadReport`).
  import { app, loadReport } from '../state.svelte';
  import { t } from '../i18n/index.svelte';
  import type { Range } from '../types';

  const presets: ReadonlyArray<'all' | '7d' | '30d' | 'month' | 'custom'> = ['all', '7d', '30d', 'month', 'custom'];

  function choosePreset(preset: (typeof presets)[number]): void {
    if (preset === 'custom') {
      // Prelazak na "vlastiti" sam po sebi ne traži izvještaj — čeka oba datuma (onchange ispod).
      if (app.range.preset !== 'custom') {
        app.range = { preset: 'custom', since: '', until: '' };
      }
      return;
    }
    app.range = { preset };
    void loadReport();
  }

  function setSince(value: string): void {
    const current = app.range;
    if (current.preset !== 'custom') return;
    const next: Range = { preset: 'custom', since: value, until: current.until };
    app.range = next;
    if (next.since && next.until) void loadReport();
  }

  function setUntil(value: string): void {
    const current = app.range;
    if (current.preset !== 'custom') return;
    const next: Range = { preset: 'custom', since: current.since, until: value };
    app.range = next;
    if (next.since && next.until) void loadReport();
  }
</script>

<div class="flex items-center gap-1" role="group" aria-label={t('top.range')}>
  {#each presets as preset (preset)}
    <button
      type="button"
      class="rounded-md px-2 py-1 text-xs {app.range.preset === preset
        ? 'bg-brand-500 text-on-brand'
        : 'text-ink-1 hover:bg-surface-2'}"
      aria-pressed={app.range.preset === preset}
      onclick={() => choosePreset(preset)}
    >
      {t(`range.${preset}`)}
    </button>
  {/each}
  {#if app.range.preset === 'custom'}
    {@const custom = app.range}
    <input
      type="date"
      class="rounded-md border border-line bg-surface-1 px-1 py-0.5 text-xs text-ink-0"
      aria-label={t('range.from')}
      value={custom.since}
      onchange={(e) => setSince(e.currentTarget.value)}
    />
    <input
      type="date"
      class="rounded-md border border-line bg-surface-1 px-1 py-0.5 text-xs text-ink-0"
      aria-label={t('range.to')}
      value={custom.until}
      onchange={(e) => setUntil(e.currentTarget.value)}
    />
  {/if}
</div>
