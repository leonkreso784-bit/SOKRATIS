<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Dnevnik: override vrste rada u mjestu)
  // Jedini dosad izgrađen pogled koji PIŠE (S-012 dopušta pisanje kroz `Api`, izračun ostaje u
  // `core`): `<select>` po retku šalje `api.setOverride`, pa ODMAH `loadReport()` da tablica pokaže
  // STVARNO stanje koje je jezgra preračunala — ne optimističku lokalnu izmjenu koja bi mogla
  // pobjeći od onoga što je zapravo spremljeno u `overrides.json`. `writing` gasi SVE kontrole dok
  // poziv traje (Ruling brifa #6) da dvostruki klik ne pošalje dva zahtjeva odjednom.
  import { app, loadReport } from '../lib/state.svelte';
  import { api } from '../lib/api';
  import { getDict, getLang, t } from '../lib/i18n/index.svelte';
  import { kindLabel, subLabel, ymd } from '../lib/format';
  import { sortDiary } from './helpers';
  import type { CommitRow, WorkKind } from '../lib/types';

  // Isti pet vrsta koje `format.ts`/`kind.*` ključevi poznaju (S-021) — `<option>` se generira iz
  // ovog niza, pa je `e.currentTarget.value as WorkKind` niže siguran cast (vrijednost je uvijek
  // jedna od ovih pet, nikad proizvoljan tekst iz preglednika).
  const KINDS: readonly WorkKind[] = ['planning', 'documentation', 'execution', 'polish', 'debugging'];

  let writing = $state(false);

  const rows = $derived(sortDiary(app.report?.commits ?? []));

  async function onKindChange(commit: CommitRow, kind: WorkKind): Promise<void> {
    const id = app.currentId;
    if (id === null || writing) return;
    writing = true;
    try {
      await api.setOverride(id, commit.sha, kind);
      await loadReport();
    } finally {
      writing = false;
    }
  }

  async function onReset(commit: CommitRow): Promise<void> {
    const id = app.currentId;
    if (id === null || writing) return;
    writing = true;
    try {
      await api.setOverride(id, commit.sha, null);
      await loadReport();
    } finally {
      writing = false;
    }
  }
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.diary')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if rows.length === 0}
    <p class="text-sm text-ink-2">{t('diary.empty')}</p>
  {:else}
    <!-- `table-fixed` + stupci sa zadanom širinom drže tablicu unutar `main` (pouka T27, #9): stupac
         `naslov` je JEDINI bez zadane širine pa uzima sav preostali prostor (CSS `table-layout:fixed`
         raspodjeljuje ostatak na stupce bez `width`), dug naslov se REŽE elipsom uz `title` za cijeli
         tekst na hover. Zaglavlje je `sticky` unutar `main`-ovog vlastitog skrola (stotine redaka u
         snimci) — nema potrebe za drugim, ugniježđenim skrol-okvirom. -->
    <table class="w-full table-fixed text-left text-sm">
      <thead class="sticky top-0 z-10 bg-surface-1">
        <tr class="text-ink-2">
          <th scope="col" class="w-20 py-1 pl-1 pr-3">{t('diary.sha')}</th>
          <th scope="col" class="w-28 py-1 pr-3">{t('diary.date')}</th>
          <th scope="col" class="py-1 pr-3">{t('diary.subject')}</th>
          <th scope="col" class="w-44 py-1 pr-3">{t('diary.kind')}</th>
          <th scope="col" class="w-36 py-1 pr-3">{t('diary.sub')}</th>
          <th scope="col" class="w-40 py-1 pr-3">{t('diary.overridden')}</th>
        </tr>
      </thead>
      <tbody>
        {#each rows as commit (commit.sha)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pl-1 pr-3 font-mono text-xs">{commit.sha}</td>
            <td class="py-1 pr-3">{ymd(commit.date, getLang())}</td>
            <td class="truncate py-1 pr-3" title={commit.subject}>{commit.subject}</td>
            <td class="py-1 pr-3">
              <select
                class="w-full rounded-md border border-line bg-surface-1 px-1 py-0.5 text-sm text-ink-0"
                aria-label={`${t('diary.kind')}: ${commit.sha}`}
                value={commit.kind}
                disabled={writing}
                onchange={(e) => void onKindChange(commit, e.currentTarget.value as WorkKind)}
              >
                {#each KINDS as kind (kind)}
                  <option value={kind}>{kindLabel(kind, getDict())}</option>
                {/each}
              </select>
            </td>
            <td class="py-1 pr-3">{subLabel(commit.sub, getDict())}</td>
            <td class="py-1 pr-3">
              {#if commit.overridden}
                <span class="inline-flex items-center gap-2">
                  <span class="rounded-full bg-surface-2 px-2 py-0.5 text-xs text-ink-1">{t('diary.overridden')}</span>
                  <button
                    type="button"
                    class="text-xs text-ink-2 underline decoration-dotted hover:text-ink-0 disabled:opacity-50"
                    disabled={writing}
                    onclick={() => void onReset(commit)}
                  >
                    {t('diary.reset')}
                  </button>
                </span>
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
