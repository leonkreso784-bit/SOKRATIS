<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Isporuke: samo prikaz, `Report.deliveries` kakav jest)
  // Za razliku od Dnevnika, ovaj pogled ništa ne piše (S-012) — `sortDeliveries` (`views/helpers.ts`)
  // dijeli komparator sa `sortDiary` (isti ISO-datum, isto "najnovije prvo") jer su oba niza istog
  // oblika ("date" prvo). Deploy je ikona s `aria-label` (T7 u a11y-pravilima brifa): 🚀 sam po sebi
  // ne govori ništa čitaču ekrana bez teksta.
  import { app } from '../lib/state.svelte';
  import { getDict, getLang, t } from '../lib/i18n/index.svelte';
  import { kindLabel, ymd } from '../lib/format';
  import { sortDeliveries } from './helpers';

  const rows = $derived(sortDeliveries(app.report?.deliveries ?? []));
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.deliveries')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if rows.length === 0}
    <p class="text-sm text-ink-2">{t('deliveries.empty')}</p>
  {:else}
    <!-- Isti obrazac kao Dnevnik (T27/T28, #9): `table-fixed`, "naslov" bez zadane širine uzima
         preostali prostor i dugačak tekst se reže elipsom (`truncate` + `title` za cijeli tekst). -->
    <table class="w-full table-fixed text-left text-sm">
      <thead>
        <tr class="text-ink-2">
          <th scope="col" class="w-28 py-1 pr-3">{t('deliveries.date')}</th>
          <th scope="col" class="w-40 py-1 pr-3">{t('deliveries.model')}</th>
          <th scope="col" class="py-1 pr-3">{t('deliveries.title')}</th>
          <th scope="col" class="w-40 py-1 pr-3">{t('nav.kinds')}</th>
          <th scope="col" class="w-20 py-1 pr-3">{t('deliveries.deploy')}</th>
        </tr>
      </thead>
      <tbody>
        {#each rows as d, i (d.date + '::' + i)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pr-3">{ymd(d.date, getLang())}</td>
            <td class="truncate py-1 pr-3" title={d.model}>{d.model || t('common.na')}</td>
            <td class="truncate py-1 pr-3" title={d.title}>{d.title}</td>
            <td class="py-1 pr-3">{kindLabel(d.kind, getDict())}</td>
            <td class="py-1 pr-3">
              {#if d.deploy}
                <span aria-label={t('deliveries.deploy')}>🚀</span>
              {:else}
                <span class="text-ink-2">{t('common.na')}</span>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
