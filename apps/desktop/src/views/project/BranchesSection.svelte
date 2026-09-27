<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/58 — sekcija Grane na ploči projekta, S-034): T58 crta SAMO tablicu iz
  // `Report.branches` (`BranchStats`, S-032) — jezgra je već izračunala commite/redke/sate po grani,
  // sučelje ih samo ispisuje (S-012). Graf (`HBars`) dolazi u T59; ova sekcija ne uvozi ništa iz
  // `lib/charts` da ne preduhitri tu ciglu. Kad je `Report.scope` „default" ili postoji samo jedna
  // grana, dodatna rečenica javlja da mjerenje NIJE obuhvatilo sve grane (spec §3.2) — bez nje bi
  // prazna/kratka tablica izgledala kao da projekt stvarno ima samo jednu granu.
  // Dopunjeno M2/59 — graf `HBars` (S-035, T57) iznad tablice: isti prekidač commiti/sati kao u
  // `TempoSection`, ali s POSTOJEĆIM ključevima `branches.commits`/`branches.hours` (nema novog
  // natpisa samo za prekidač). Bez ijedne grane (raspon bez commita) graf se uopće ne crta — prazna
  // tablica i rečenica ispod ostaju jedini prikaz (Review Focus #4).
  import { app } from '../../lib/state.svelte';
  import { getLang, t } from '../../lib/i18n/index.svelte';
  import { hours, num } from '../../lib/format';
  import HBars from '../../lib/charts/HBars.svelte';
  import Explainable from '../../lib/explain/Explainable.svelte';
  import { sectionId, sectionKey } from './sections';

  let mode = $state<'commits' | 'hours'>('commits');

  const branches = $derived(app.report?.branches ?? []);
  const defaultOnly = $derived(app.report ? app.report.scope === 'default' || app.report.branches.length <= 1 : false);
  const items = $derived(
    branches.map((b) => ({ label: b.name, value: mode === 'commits' ? b.commits : b.hours, merged: b.merged })),
  );
  const valueText = (v: number): string => (mode === 'commits' ? num(v, getLang()) : hours(v, getLang()));
</script>

<div class="flex flex-col gap-4">
  <h2 id={sectionId('branches')} class="text-xl font-semibold text-ink-0">{t(sectionKey('branches'))}</h2>

  {#if branches.length > 0}
    <div class="flex gap-1" role="group" aria-label={t(sectionKey('branches'))}>
      <button
        type="button"
        class="rounded-md px-3 py-1.5 text-sm {mode === 'commits' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
        aria-pressed={mode === 'commits'}
        onclick={() => (mode = 'commits')}
      >
        {t('branches.commits')}
      </button>
      <button
        type="button"
        class="rounded-md px-3 py-1.5 text-sm {mode === 'hours' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
        aria-pressed={mode === 'hours'}
        onclick={() => (mode = 'hours')}
      >
        {t('branches.hours')}
      </button>
    </div>

    <!-- Popravak 1 (V1, Ruling R70): vidljiv naslov iznad grafa, isti izraz kao `label` (bez novog
         ključa) — naslov je IZVAN `<Explainable>` da klik na naslov ne otvara karticu. -->
    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t('branches.chart')}</h3>
      <Explainable id="branches.bars" block>
        <HBars {items} label={t('branches.chart')} {valueText} mergedLabel={t('branches.merged')} />
      </Explainable>
    </div>
  {/if}

  <table class="w-full text-left text-sm">
    <thead>
      <tr class="text-ink-2">
        <th scope="col" class="py-1 pr-3">{t('branches.name')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.commits')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.hours')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.lines')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.merged')}</th>
      </tr>
    </thead>
    <tbody>
      {#each branches as b (b.name)}
        <tr class="border-t border-line text-ink-1">
          <td class="py-1 pr-3">{b.name}</td>
          <td class="py-1 pr-3">{num(b.commits, getLang())}</td>
          <td class="py-1 pr-3">{hours(b.hours, getLang())}</td>
          <td class="py-1 pr-3">{num(b.lines, getLang())}</td>
          <td class="py-1 pr-3">{b.merged ? t('branches.yes') : t('branches.no')}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if defaultOnly}
    <p class="text-sm text-ink-2">{t('branches.default_only')}</p>
  {/if}
</div>
