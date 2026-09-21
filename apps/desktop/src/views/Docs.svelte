<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Dokumentacija: ocjena, kašnjenje, nalazi s kopiranjem putanje)
  // `docs === null` znači da projekt nema mapu dokumentacije koju profil prepoznaje — jezgra je to
  // već utvrdila (S-012, sučelje ne pogađa) i pogled to MORA reći umjesto praznih brojki (spec 6.2,
  // Ruling brifa #2 drži ovu granu dostižnom i kad mock daje primjer). `copyText`/`joinRepoPath`
  // (`views/helpers.ts`) su jedini put do clipboarda i do apsolutne putanje — ništa se ne lijepi
  // izravno ovdje. Velika ocjena koristi rem-veličinu fonta (ne postotak širine, pouka #9).
  import { app } from '../lib/state.svelte';
  import { getLang, t } from '../lib/i18n/index.svelte';
  import { num } from '../lib/format';
  import { copyText, joinRepoPath } from './helpers';
  import type { Finding, ProjectSummary } from '../lib/types';

  let copiedPath = $state<string | null>(null);
  let copyTimeout: ReturnType<typeof setTimeout> | null = null;

  const docs = $derived(app.report?.docs ?? null);
  const currentProject = $derived<ProjectSummary | null>(
    app.projects.find((p) => p.id === app.currentId) ?? null,
  );

  async function onCopyPath(finding: Finding): Promise<void> {
    if (!currentProject) return;
    const fullPath = joinRepoPath(currentProject.root_path, finding.path);
    const ok = await copyText(fullPath);
    if (!ok) return;
    copiedPath = finding.path;
    if (copyTimeout) clearTimeout(copyTimeout);
    // Poruka "kopirano" ne smije trajno ostati na ekranu — dva sekunde je dovoljno da je vidi i onaj
    // tko baš u tom trenutku gleda na ekran (isti duh kao `signals.count` sažetak: kratko i jasno).
    copyTimeout = setTimeout(() => {
      copiedPath = null;
    }, 2000);
  }
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.docs')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if !docs}
    <p class="text-sm text-ink-2">{t('docs.na')}</p>
  {:else}
    <div class="flex flex-wrap items-center gap-6">
      <div class="flex flex-col items-start gap-1 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
        <span class="text-xs text-ink-2">{t('docs.score')}</span>
        <p class="text-5xl font-semibold leading-none text-ink-0">
          {num(docs.score, getLang())}<span class="text-lg text-ink-2">/100</span>
        </p>
      </div>
      {#if docs.lag_days !== null}
        <p class="text-sm text-warn-ink">{t('docs.lag', { n: docs.lag_days })}</p>
      {/if}
    </div>

    <section class="flex flex-col gap-2">
      <h2 class="text-lg font-semibold text-ink-0">{t('docs.findings')}</h2>
      {#if docs.findings.length === 0}
        <p class="text-sm text-ink-2">{t('docs.none')}</p>
      {:else}
        <ul class="flex flex-col">
          {#each docs.findings as finding, i (finding.check + '::' + finding.path + '::' + i)}
            <li class="flex flex-col gap-1 border-t border-line py-2 text-sm first:border-t-0">
              <div class="flex flex-wrap items-center gap-2">
                <span class="rounded bg-surface-2 px-1.5 py-0.5 text-xs text-ink-2">{finding.check}</span>
                <button
                  type="button"
                  class="max-w-full truncate text-left font-mono text-xs text-ink-blue underline decoration-dotted hover:text-ink-0 disabled:cursor-default disabled:no-underline disabled:opacity-60"
                  title={finding.path}
                  aria-label={`${t('docs.copy')}: ${finding.path}`}
                  disabled={!currentProject}
                  onclick={() => void onCopyPath(finding)}
                >
                  {finding.path}{finding.line !== null ? ':' + finding.line : ''}
                </button>
              </div>
              <p class="break-words text-ink-1">{finding.message}</p>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <!-- Region uvijek postoji u DOM-u (i prazan) da čitač ekrana pouzdano najavi promjenu teksta —
         element koji se tek DODA s `aria-live` ponekad se ne najavi (WebView2/Chromium rub). -->
    <p class="text-xs text-ink-2" aria-live="polite">{copiedPath ? t('docs.copied') : ''}</p>
  {/if}
</div>
