<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Vrste rada: udio commita po vrsti kao prsten; krug popravka 1 —
  // vizualna provjera je našla prsten razvučen na cijelu širinu `<main>` bez legende)
  // `Ring` (M2/23) ne oblikuje ništa samo — recenzija je iz njega izbacila svaki izračun (tooltip
  // ne zna postotak). Zato SVAKI segment ovdje dobiva GOTOV natpis (naziv vrste + postotak) kroz
  // `kindLabel`/`percent`; `Ring` ga samo iscrta (Ruling orkestratora M2/27 #1). Udio (`share`) je
  // `KindStats.share` KAKAV JEST — sučelje ga ne računa dijeljenjem `commits / ukupno` (S-012).
  // `Ring.svelte` crta `class="w-full"` (SVG bez vlastite širine), pa ga OGRANIČAVA omotač ovdje —
  // `Ring.svelte` se ne dira. `kindColor` (`views/helpers.ts`) je JEDNA mapa boja koju čitaju i
  // segment prstena i oznaka u retku tablice — legenda je tablica sama, ne druga kopija (S-010).
  import { app } from '../lib/state.svelte';
  import { getDict, getLang, t } from '../lib/i18n/index.svelte';
  import { kindLabel, num, percent } from '../lib/format';
  import { kindColor } from './helpers';
  import Ring from '../lib/charts/Ring.svelte';

  const kinds = $derived(app.report?.kinds ?? []);
  const segments = $derived(
    kinds.map((k) => ({
      value: k.commits,
      label: `${kindLabel(k.kind, getDict())} — ${percent(k.share, getLang())}`,
      color: kindColor(k.kind),
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
    <!-- Prsten uz tablicu na širokom prozoru (`lg:`), jedno ispod drugog na uskom — tablica mora
         ostati vidljiva bez pomicanja pri 1280×900 (vizualna provjera, krug popravka 1). -->
    <div class="flex flex-col gap-6 lg:flex-row lg:items-start">
      <div class="w-[260px] shrink-0">
        <Ring {segments} label={t('kinds.title')} />
      </div>

      <table class="min-w-0 flex-1 text-left text-sm">
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
              <td class="py-1 pr-3">
                <span class="flex items-center gap-2">
                  <!-- Oznaka je ukras (boja je već rečena imenom vrste uz nju) — čitač ekrana je preskače. -->
                  <span
                    aria-hidden="true"
                    class="inline-block h-3 w-3 shrink-0 rounded-full"
                    style="background: {kindColor(k.kind)}"
                  ></span>
                  {kindLabel(k.kind, getDict())}
                </span>
              </td>
              <td class="py-1 pr-3">{num(k.commits, getLang())}</td>
              <td class="py-1 pr-3">{percent(k.share, getLang())}</td>
              <td class="py-1 pr-3">{num(k.lines, getLang())}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
