<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/26 — traka signala u zaglavlju odabranog projekta)
  // Spec §6.2: signali stoje uz projekt (traka s dokazom na klik), ne kao deseti pogled u izborniku —
  // traka čita `app.report` izravno (globalna runa, S-010) pa se sama osvježi čim `state.svelte.ts`
  // postavi nov izvještaj. `<details>` je izvorni HTML disclosure: dokaz se otvara/zatvara
  // tipkovnicom bez ručnog `aria-expanded` stanja koje bismo morali sami pamtiti.
  import { app } from '../state.svelte';
  import { t } from '../i18n/index.svelte';
  import { severityClass } from '../../views/helpers';
</script>

{#if app.currentId !== null && app.report}
  <div class="border-b border-line bg-surface-1 px-4 py-2">
    {#if app.report.signals.length === 0}
      <p class="text-sm text-ink-2">{t('signals.none')}</p>
    {:else}
      <ul class="flex flex-col gap-1" aria-label={t('signals.title')}>
        {#each app.report.signals as signal, i (signal.rule + '-' + i)}
          <li class="flex flex-wrap items-baseline gap-2">
            <span class="text-sm font-semibold {severityClass(signal.severity)}">
              {t('sev.' + signal.severity)}
            </span>
            <span class="text-sm text-ink-1">{t('rule.' + signal.rule)}</span>
            <details class="text-xs">
              <summary class="cursor-pointer list-none text-ink-2 underline decoration-dotted">
                {t('signals.evidence')}
              </summary>
              <ul class="mt-1 list-disc pl-5 text-ink-1">
                {#each signal.evidence as line, j (j)}
                  <li>{line}</li>
                {/each}
              </ul>
            </details>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
{/if}
