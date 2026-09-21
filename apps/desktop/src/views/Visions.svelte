<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Vizije: zbroj po stanju, dodavanje kroz redak-obrazac, brisanje s potvrdom)
  // `Vision.state` je slobodan tekst (`model.rs`: `state: String`, ne enum) — korisnik ga upisuje u
  // obrazac, sučelje ga NE prevodi kroz i18n (S-012 duh: prikaz onoga što jezgra/korisnik stvarno dao,
  // ne izmišljanje zatvorenog popisa koji core ne poznaje). `saveVisions` uvijek šalje CIJEL popis
  // (ime parametra u brifu je "sve") — dodavanje šalje stari popis + novi redak, brisanje šalje stari
  // popis bez obrisanog. Brisanje NE koristi `window.confirm` (Ruling brifa #6 — WebView2/Playwright ga
  // loše podnose): dvokoračni gumb u retku ("Obriši" → "Potvrdi"/"Odustani") ostaje unutar iste tablice.
  import { app, loadReport } from '../lib/state.svelte';
  import { api } from '../lib/api';
  import { getLang, t } from '../lib/i18n/index.svelte';
  import { num, percent } from '../lib/format';
  import type { Vision } from '../lib/types';

  let showForm = $state(false);
  let formTitle = $state('');
  let formSource = $state('');
  let formState = $state('');
  let formPercent = $state('');
  let formNote = $state('');
  let writing = $state(false);
  let confirmDeleteIndex = $state<number | null>(null);

  function openForm(): void {
    showForm = true;
    formTitle = '';
    formSource = '';
    formState = '';
    formPercent = '';
    formNote = '';
  }

  function cancelForm(): void {
    showForm = false;
  }

  // `Vision.percent` je `Option<u8>` u jezgri — cijeli broj 0-100, ne razlomak 0-1. Prazno polje
  // znači "nepoznato" (`null`), ne nula.
  function parsePercent(text: string): number | null {
    const trimmed = text.trim();
    if (trimmed === '') return null;
    const n = Number(trimmed);
    return Number.isNaN(n) ? null : n;
  }

  // `percent()` (`format.ts`) očekuje razlomak 0-1 (isti obrazac kao `KindStats.share`); pretvorba
  // cijelog broja 0-100 u razlomak živi OVDJE (domenska odluka o obliku polja), samo ispis ostaje u
  // `format.ts` (S-012) — `null` mora ostati `null`, ne `0` (dijeljenje `null / 100` bi tiho dalo 0).
  function percentText(p: number | null): string {
    return percent(p === null ? null : p / 100, getLang());
  }

  async function saveNewVision(): Promise<void> {
    const id = app.currentId;
    if (id === null || writing || formTitle.trim() === '') return;
    const newVision: Vision = {
      title: formTitle.trim(),
      source: formSource.trim(),
      state: formState.trim(),
      percent: parsePercent(formPercent),
      note: formNote.trim(),
    };
    writing = true;
    try {
      await api.saveVisions(id, [...(app.report?.visions ?? []), newVision]);
      await loadReport();
      showForm = false;
    } finally {
      writing = false;
    }
  }

  function askDelete(index: number): void {
    confirmDeleteIndex = index;
  }

  function cancelDelete(): void {
    confirmDeleteIndex = null;
  }

  async function confirmDelete(index: number): Promise<void> {
    const id = app.currentId;
    if (id === null || writing) return;
    writing = true;
    try {
      const remaining = (app.report?.visions ?? []).filter((_, i) => i !== index);
      await api.saveVisions(id, remaining);
      await loadReport();
      confirmDeleteIndex = null;
    } finally {
      writing = false;
    }
  }
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.visions')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else}
    {#if app.report.vision_totals.length > 0}
      <div class="flex flex-wrap gap-2" role="list" aria-label={t('visions.totals')}>
        {#each app.report.vision_totals as vt (vt.state)}
          <span role="listitem" class="rounded-full border border-line bg-surface-1 px-3 py-1 text-xs text-ink-1">
            {vt.state}: {num(vt.count, getLang())}
          </span>
        {/each}
      </div>
    {/if}

    <button
      type="button"
      class="self-start rounded-md bg-brand-500 px-3 py-1.5 text-sm font-medium text-on-brand hover:bg-brand-600 disabled:opacity-50"
      onclick={openForm}
      disabled={showForm}
    >
      {t('visions.add')}
    </button>

    {#if showForm}
      <form
        class="flex max-w-2xl flex-col gap-3 rounded-lg border border-line bg-surface-1 p-3"
        onsubmit={(e) => {
          e.preventDefault();
          void saveNewVision();
        }}
      >
        <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
          <label class="flex flex-col gap-1 text-xs text-ink-2">
            {t('visions.title')}
            <input
              class="rounded-md border border-line bg-surface-0 px-2 py-1 text-sm text-ink-0"
              bind:value={formTitle}
              disabled={writing}
            />
          </label>
          <label class="flex flex-col gap-1 text-xs text-ink-2">
            {t('visions.source')}
            <input
              class="rounded-md border border-line bg-surface-0 px-2 py-1 text-sm text-ink-0"
              bind:value={formSource}
              disabled={writing}
            />
          </label>
          <label class="flex flex-col gap-1 text-xs text-ink-2">
            {t('visions.state')}
            <input
              class="rounded-md border border-line bg-surface-0 px-2 py-1 text-sm text-ink-0"
              bind:value={formState}
              disabled={writing}
            />
          </label>
          <label class="flex flex-col gap-1 text-xs text-ink-2">
            {t('visions.percent')}
            <!-- `value=`/`oninput=` umjesto `bind:value` NAMJERNO: Svelte za `type="number"` sam
                 pretvara vezanu varijablu u `number`, a `formPercent` ostaje `string` do parsiranja
                 (`parsePercent`) jer prazno polje mora ostati "nepoznato" (`null`), ne `0`. -->
            <input
              type="number"
              min="0"
              max="100"
              class="w-full max-w-32 rounded-md border border-line bg-surface-0 px-2 py-1 text-sm text-ink-0"
              value={formPercent}
              oninput={(e) => (formPercent = e.currentTarget.value)}
              disabled={writing}
            />
          </label>
          <label class="flex flex-col gap-1 text-xs text-ink-2 sm:col-span-2">
            {t('visions.note')}
            <input
              class="rounded-md border border-line bg-surface-0 px-2 py-1 text-sm text-ink-0"
              bind:value={formNote}
              disabled={writing}
            />
          </label>
        </div>
        <div class="flex gap-2">
          <button
            type="submit"
            class="rounded-md bg-brand-500 px-3 py-1.5 text-sm font-medium text-on-brand hover:bg-brand-600 disabled:opacity-50"
            disabled={writing || formTitle.trim() === ''}
          >
            {t('visions.save')}
          </button>
          <button
            type="button"
            class="rounded-md px-3 py-1.5 text-sm text-ink-1 hover:bg-surface-2"
            onclick={cancelForm}
            disabled={writing}
          >
            {t('visions.cancel')}
          </button>
        </div>
      </form>
    {/if}

    {#if app.report.visions.length === 0}
      <p class="text-sm text-ink-2">{t('visions.empty')}</p>
    {:else}
      <!-- `table-fixed` (#9): "bilješka" bez zadane širine uzima preostali prostor; dugačak tekst se
           reže elipsom uz `title` za cijeli sadržaj na hover. -->
      <table class="w-full table-fixed text-left text-sm">
        <thead>
          <tr class="text-ink-2">
            <th scope="col" class="w-48 py-1 pr-3">{t('visions.title')}</th>
            <th scope="col" class="w-32 py-1 pr-3">{t('visions.source')}</th>
            <th scope="col" class="w-28 py-1 pr-3">{t('visions.state')}</th>
            <th scope="col" class="w-20 py-1 pr-3">{t('visions.percent')}</th>
            <th scope="col" class="py-1 pr-3">{t('visions.note')}</th>
            <th scope="col" class="w-48 py-1 pr-3"><span class="sr-only">{t('visions.delete')}</span></th>
          </tr>
        </thead>
        <tbody>
          {#each app.report.visions as vision, i (vision.title + '::' + i)}
            <tr class="border-t border-line text-ink-1">
              <td class="truncate py-1 pr-3" title={vision.title}>{vision.title}</td>
              <td class="truncate py-1 pr-3" title={vision.source}>{vision.source}</td>
              <td class="py-1 pr-3">{vision.state}</td>
              <td class="py-1 pr-3">{percentText(vision.percent)}</td>
              <td class="truncate py-1 pr-3" title={vision.note}>{vision.note}</td>
              <td class="py-1 pr-3">
                {#if confirmDeleteIndex === i}
                  <span class="inline-flex items-center gap-2">
                    <span class="text-xs text-ink-2">{t('visions.deleteConfirm')}</span>
                    <button
                      type="button"
                      class="text-xs text-danger-ink underline decoration-dotted disabled:opacity-50"
                      disabled={writing}
                      onclick={() => void confirmDelete(i)}
                    >
                      {t('common.confirm')}
                    </button>
                    <button
                      type="button"
                      class="text-xs text-ink-2 underline decoration-dotted disabled:opacity-50"
                      disabled={writing}
                      onclick={cancelDelete}
                    >
                      {t('visions.cancel')}
                    </button>
                  </span>
                {:else}
                  <button
                    type="button"
                    class="text-xs text-danger-ink underline decoration-dotted hover:opacity-80"
                    onclick={() => askDelete(i)}
                  >
                    {t('visions.delete')}
                  </button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  {/if}
</div>
