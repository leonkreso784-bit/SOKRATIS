<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Dnevnik: override vrste rada u mjestu; krug popravka 1 — mjerenje u
  // pregledniku na 960×600 je našlo stupac "naslov" od 17 px: fiksni stupci su pojeli 672 od 689 px)
  // Jedini dosad izgrađen pogled koji PIŠE (S-012 dopušta pisanje kroz `Api`, izračun ostaje u
  // `core`): `<select>` po retku šalje `api.setOverride`, pa ODMAH `loadReport()` da tablica pokaže
  // STVARNO stanje koje je jezgra preračunala — ne optimističku lokalnu izmjenu koja bi mogla
  // pobjeći od onoga što je zapravo spremljeno u `overrides.json`. `writing` gasi SVE kontrole dok
  // poziv traje (Ruling brifa #6) da dvostruki klik ne pošalje dva zahtjeva odjednom.
  // ŠIRINE (krug popravka 1, mjereno Playwrightom): `sha`/`datum`/`podvrsta` nemaju `truncate` pa se
  // PO DEFAULTU lome u dva retka umjesto da guraju tablicu vodoravno — smiju biti uski. "vrsta" nosi
  // `<select>` kojem treba PUNA najdulja oznaka (hr "vođenje dokumentacije") bez rezanja strelicom,
  // pa je širi na `lg:` (≥1024 px) nego na uskom prozoru gdje ionako nema mjesta. "ručno" je sad
  // ikona (●) + kratak gumb (↺) umjesto pune riječi + punog teksta gumba — isto značenje, manje px.
  // "naslov" je JEDINI stupac bez zadane širine (uzima cijeli ostatak, CSS `table-layout:fixed`) i
  // smije se lomiti u dva retka (`line-clamp-2`) umjesto da se odreže na jedan.
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
         raspodjeljuje ostatak na stupce bez `width`). Zaglavlje je `sticky` unutar `main`-ovog
         vlastitog skrola (stotine redaka u snimci) — nema omotača s `overflow` koji bi to pokvario. -->
    <table class="w-full table-fixed text-left text-sm">
      <thead class="sticky top-0 z-10 bg-surface-1">
        <tr class="text-ink-2">
          <th scope="col" class="w-[72px] py-1 pl-1 pr-3">{t('diary.sha')}</th>
          <th scope="col" class="w-[92px] py-1 pr-3">{t('diary.date')}</th>
          <th scope="col" class="py-1 pr-3">{t('diary.subject')}</th>
          <th scope="col" class="w-[140px] py-1 pr-3 lg:w-[224px]">{t('diary.kind')}</th>
          <th scope="col" class="w-[100px] py-1 pr-3 lg:w-[140px]">{t('diary.sub')}</th>
          <th scope="col" class="w-[64px] py-1 pr-3">{t('diary.overridden')}</th>
        </tr>
      </thead>
      <tbody>
        {#each rows as commit (commit.sha)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pl-1 pr-3 font-mono text-xs">{commit.sha}</td>
            <td class="py-1 pr-3">{ymd(commit.date, getLang())}</td>
            <td class="py-1 pr-3">
              <span class="line-clamp-2 break-words" title={commit.subject}>{commit.subject}</span>
            </td>
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
                <!-- Kompaktna zamjena za punu riječ + puni tekst gumba (krug popravka 1): ista
                     informacija (ručno klasificirano · vrati klasifikator), manje px. Oznaka je
                     ukras s `sr-only` tekstom (čitač ekrana ne oslanja se na `aria-label` običnog
                     `<span>`-a), gumb ima i `aria-label` i `title` = `diary.reset` kako traži ruling. -->
                <span class="inline-flex items-center gap-1">
                  <span
                    aria-hidden="true"
                    class="inline-flex h-5 w-5 items-center justify-center rounded-full bg-surface-2 text-[10px] leading-none text-ink-1"
                  >
                    ●
                  </span>
                  <span class="sr-only">{t('diary.overridden')}</span>
                  <button
                    type="button"
                    class="grid h-5 w-5 place-items-center rounded text-ink-2 hover:bg-surface-2 hover:text-ink-0 disabled:opacity-50"
                    aria-label={t('diary.reset')}
                    title={t('diary.reset')}
                    disabled={writing}
                    onclick={() => void onReset(commit)}
                  >
                    ↺
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
