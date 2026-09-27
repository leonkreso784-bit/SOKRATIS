<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Faze: tri odjeljka po stanju, iz čistog razvrstavanja u helpers.ts)
  // `phaseRows` (`views/helpers.ts`) samo razvrstava `Report.phases` po `state` — jezgra već zna
  // stanje svake faze. `bricksPerDay` je JEDINI izračun ovdje koji jezgra još ne daje (dug prema
  // `core`, zapisano u helpers.ts i izvještaju M2/27 kao odstupanje od S-012).
  // Dopunjeno M2/42 — zaglavlja "dana", "commita", "cigle/dan" dobivaju vlastiti `<Explainable>`
  // (dopuna T42); "cigle" i raspon datuma nemaju id pa ostaju bez kartice.
  // Preseljeno M2/58 iz views/Phases.svelte — sadržaj nepromijenjen, samo <h1> → <h2 id> i putanje
  // uvoza (S-034).
  // Dopunjeno M2/60 — Gantt (spec §3.2, `layout.ts::ganttLayout`, T57) iznad tri tablice: traka po
  // fazi od-do, „danas" kao crta. `Report` nema polje „danas" (Ruling R72: `Report.until` ostaje
  // `null` kad je raspon „sve", jezgra ga NE upisuje) — `today` je pomoćna vrijednost SUČELJA, samo
  // za prikaz, izračunata jednom ovdje iz sata ovog računala.
  import { app } from '../../lib/state.svelte';
  import { getDict, getLang, t } from '../../lib/i18n/index.svelte';
  import { num, phaseState, ymd } from '../../lib/format';
  import { bricksPerDay, phaseRows } from '../helpers';
  import Gantt from '../../lib/charts/Gantt.svelte';
  import Explainable from '../../lib/explain/Explainable.svelte';
  import type { Phase, PhaseState } from '../../lib/types';
  import { sectionId, sectionKey } from './sections';

  // Redoslijed odjeljaka prati oblik koji `phaseRows` vraća (brief M2/27): zatvorene → u tijeku →
  // planirane.
  const SECTIONS: readonly PhaseState[] = ['closed', 'running', 'planned'];

  const rows = $derived(phaseRows(app.report?.phases ?? []));
  const phases = $derived(app.report?.phases ?? []);
  const today = new Date().toISOString().slice(0, 10);

  function sectionRows(state: PhaseState): Phase[] {
    return rows[state];
  }
</script>

<div class="flex flex-col gap-6">
  <h2 id={sectionId('phases')} class="text-xl font-semibold text-ink-0">{t(sectionKey('phases'))}</h2>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if app.report.phases.length === 0}
    <p class="text-sm text-ink-2">{t('phases.none')}</p>
  {:else}
    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t('phases.gantt')}</h3>
      <Explainable id="phases.gantt" block>
        <Gantt {phases} {today} label={t('phases.gantt')} />
      </Explainable>
    </div>

    {#each SECTIONS as state (state)}
      {@const list = sectionRows(state)}
      {#if list.length > 0}
        <section class="flex flex-col gap-2">
          <h2 class="text-lg font-semibold text-ink-0">{phaseState(state, getDict())}</h2>
          <table class="w-full text-left text-sm">
            <thead>
              <tr class="text-ink-2">
                <th scope="col" class="py-1 pr-3">{t('nav.phases')}</th>
                <th scope="col" class="py-1 pr-3">{t('phases.bricks')}</th>
                <th scope="col" class="py-1 pr-3"><Explainable id="phases.days">{t('phases.days')}</Explainable></th>
                <th scope="col" class="py-1 pr-3"><Explainable id="phases.commits">{t('phases.commits')}</Explainable></th>
                <th scope="col" class="py-1 pr-3">
                  <Explainable id="phases.bricks_per_day">{t('phases.bricks_per_day')}</Explainable>
                </th>
                <th scope="col" class="py-1 pr-3">{t('phases.from')}–{t('phases.to')}</th>
              </tr>
            </thead>
            <tbody>
              {#each list as phase (phase.id)}
                <tr class="border-t border-line text-ink-1">
                  <td class="py-1 pr-3">{phase.name}</td>
                  <td class="py-1 pr-3">{num(phase.done_bricks, getLang())}/{num(phase.total_bricks, getLang())}</td>
                  <td class="py-1 pr-3">{num(phase.days, getLang())}</td>
                  <td class="py-1 pr-3">{num(phase.commits, getLang())}</td>
                  <td class="py-1 pr-3">{num(bricksPerDay(phase), getLang(), 1)}</td>
                  <td class="py-1 pr-3">{ymd(phase.from, getLang())}–{ymd(phase.to, getLang())}</td>
                </tr>
              {/each}
            </tbody>
          </table>
        </section>
      {/if}
    {/each}
  {/if}
</div>
