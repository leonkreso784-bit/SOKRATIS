<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/58 — nadzorna ploča projekta, S-034): jedna duga stranica, osam sekcija
  // iz JEDNOG popisa (`sections.ts`); skok-izbornik je `position: sticky` unutar `<main>` (koji
  // scrolla), sidra su `id="sec-…"` na `<h2>`. Sadržaj sekcija je preseljen iz bivših pogleda bez
  // promjene mjerenja — samo `<h1>` → `<h2>` i uvoz iz `./project/`.
  import { app } from '../lib/state.svelte';
  import { t } from '../lib/i18n/index.svelte';
  import { SECTIONS, sectionId, sectionKey } from './project/sections';
  import SummarySection from './project/SummarySection.svelte';
  import TempoSection from './project/TempoSection.svelte';
  import BranchesSection from './project/BranchesSection.svelte';
  import KindsSection from './project/KindsSection.svelte';
  import PhasesSection from './project/PhasesSection.svelte';
  import DeliveriesSection from './project/DeliveriesSection.svelte';
  import IndicatorsSection from './project/IndicatorsSection.svelte';
  import DocsSection from './project/DocsSection.svelte';
</script>

<div class="flex flex-col gap-8">
  <nav
    class="sticky top-0 z-10 -mx-4 -mt-4 flex flex-wrap gap-1 border-b border-line bg-surface-0/95 px-4 py-2 text-sm"
    aria-label={t('nav.project')}
  >
    {#each SECTIONS as s (s)}
      <a href="#{sectionId(s)}" class="rounded-md px-2 py-1 text-ink-1 hover:bg-surface-2">{t(sectionKey(s))}</a>
    {/each}
  </nav>
  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else}
    <SummarySection />
    <TempoSection />
    <BranchesSection />
    <KindsSection />
    <PhasesSection />
    <DeliveriesSection />
    <IndicatorsSection />
    <DocsSection />
  {/if}
</div>
