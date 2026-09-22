<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Tempo: dani u razdoblju, commiti/sati, kumulativa)
  // Prekidač commiti/sati je dva `<button aria-pressed>` (natpis iz rječnika, S-021) koji samo BIRA
  // koji stupac `DayStats` ide u `Bars` — ništa se ovdje ne zbraja ni ne dijeli. Kumulativna linija
  // crta `d.commits_cumulative` izravno: to polje jezgra već izračuna (S-012), sučelje ga NE
  // ponovno zbraja preko `cumulative()` iz `scale.ts` (Ruling orkestratora M2/27 #2).
  // Dopunjeno M2/42 — oba grafa dobivaju `<Explainable block>` (S-027), a zaglavlje stupca sati u
  // tablici svoj `<Explainable>` (jedini stupac tablice s vlastitim mjerenjem koje pokazatelj ne
  // ponavlja negdje drugdje — ostala zaglavlja nemaju id pa se ne omataju, dopuna T42).
  import { app } from '../lib/state.svelte';
  import { getLang, t } from '../lib/i18n/index.svelte';
  import { hours, num, ymd } from '../lib/format';
  import Bars from '../lib/charts/Bars.svelte';
  import Line from '../lib/charts/Line.svelte';
  import Explainable from '../lib/explain/Explainable.svelte';

  let mode = $state<'commits' | 'hours'>('commits');

  const days = $derived(app.report?.days ?? []);
  const barValues = $derived(days.map((d) => (mode === 'commits' ? d.commits : d.hours)));
  const barLabels = $derived(days.map((d) => ymd(d.date, getLang())));
  const cumulativeValues = $derived(days.map((d) => d.commits_cumulative));
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.tempo')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if days.length === 0}
    <p class="text-sm text-ink-2">{t('tempo.empty')}</p>
  {:else}
    <div class="flex gap-1" role="group" aria-label={t('nav.tempo')}>
      <button
        type="button"
        class="rounded-md px-3 py-1.5 text-sm {mode === 'commits' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
        aria-pressed={mode === 'commits'}
        onclick={() => (mode = 'commits')}
      >
        {t('tempo.commits')}
      </button>
      <button
        type="button"
        class="rounded-md px-3 py-1.5 text-sm {mode === 'hours' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
        aria-pressed={mode === 'hours'}
        onclick={() => (mode = 'hours')}
      >
        {t('tempo.hours')}
      </button>
    </div>

    <Explainable id="tempo.bars" block><Bars values={barValues} labels={barLabels} label={t(`tempo.${mode}`)} /></Explainable>
    <Explainable id="tempo.cumulative" block><Line values={cumulativeValues} label={t('tempo.cumulative')} /></Explainable>

    <table class="w-full text-left text-sm">
      <thead>
        <tr class="text-ink-2">
          <th scope="col" class="py-1 pr-3">{t('tempo.day')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.commits')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.cumulative')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.lines')}</th>
          <th scope="col" class="py-1 pr-3"><Explainable id="tempo.hours">{t('tempo.hours')}</Explainable></th>
          <th scope="col" class="py-1 pr-3">{t('tempo.deliveries')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.deploys')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.testLines')}</th>
        </tr>
      </thead>
      <tbody>
        {#each days as d (d.date)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pr-3">{ymd(d.date, getLang())}</td>
            <td class="py-1 pr-3">{num(d.commits, getLang())}</td>
            <td class="py-1 pr-3">{num(d.commits_cumulative, getLang())}</td>
            <td class="py-1 pr-3">{num(d.lines, getLang())}</td>
            <td class="py-1 pr-3">{hours(d.hours, getLang())}</td>
            <td class="py-1 pr-3">{num(d.deliveries, getLang())}</td>
            <td class="py-1 pr-3">{num(d.deploys, getLang())}</td>
            <td class="py-1 pr-3">{num(d.test_lines, getLang())}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
