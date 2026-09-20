<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Vrste rada: udio commita po vrsti kao prsten)
  // `Ring` (M2/23) ne oblikuje ništa samo — recenzija je iz njega izbacila svaki izračun (tooltip
  // ne zna postotak). Zato SVAKI segment ovdje dobiva GOTOV natpis (naziv vrste + postotak) kroz
  // `kindLabel`/`percent`; `Ring` ga samo iscrta (Ruling orkestratora M2/27 #1). Udio (`share`) je
  // `KindStats.share` KAKAV JEST — sučelje ga ne računa dijeljenjem `commits / ukupno` (S-012).
  import { app } from '../lib/state.svelte';
  import { getDict, getLang, t } from '../lib/i18n/index.svelte';
  import { kindLabel, num, percent } from '../lib/format';
  import Ring from '../lib/charts/Ring.svelte';
  import type { WorkKind } from '../lib/types';

  // Pet fiksnih tokena iz `tokens.css` (brief M2/27) — sve pet postoje pod ovim imenima, provjereno
  // prije uporabe; nijedna hex-boja u markupu (S-018).
  const KIND_COLORS: Record<WorkKind, string> = {
    planning: 'var(--color-brand-500)',
    documentation: 'var(--color-accent)',
    execution: 'var(--color-ink-green)',
    polish: 'var(--color-ink-amber)',
    debugging: 'var(--color-ink-red)',
  };

  const kinds = $derived(app.report?.kinds ?? []);
  const segments = $derived(
    kinds.map((k) => ({
      value: k.commits,
      label: `${kindLabel(k.kind, getDict())} — ${percent(k.share, getLang())}`,
      color: KIND_COLORS[k.kind],
    })),
  );
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.kinds')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if kinds.length === 0}
    <p class="text-sm text-ink-2">{t('kinds.empty')}</p>
  {:else}
    <Ring {segments} label={t('kinds.title')} />

    <table class="w-full text-left text-sm">
      <thead>
        <tr class="text-ink-2">
          <th scope="col" class="py-1 pr-3">{t('nav.kinds')}</th>
          <th scope="col" class="py-1 pr-3">{t('kinds.commits')}</th>
          <th scope="col" class="py-1 pr-3">{t('kinds.share')}</th>
          <th scope="col" class="py-1 pr-3">{t('kinds.lines')}</th>
        </tr>
      </thead>
      <tbody>
        {#each kinds as k (k.kind)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pr-3">{kindLabel(k.kind, getDict())}</td>
            <td class="py-1 pr-3">{num(k.commits, getLang())}</td>
            <td class="py-1 pr-3">{percent(k.share, getLang())}</td>
            <td class="py-1 pr-3">{num(k.lines, getLang())}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
