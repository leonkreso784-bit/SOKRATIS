<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/28 — Vizije: zbroj po stanju, dodavanje/uređivanje kroz ISTI redak-obrazac,
  // brisanje s potvrdom; dopuna nakon prve recenzije — spec 6.2 doslovno traži "dodaj · uredi ·
  // promijeni stanje", prvi prolaz ove cigle je imao samo dodaj/obriši)
  // Svelte 5 `{#snippet}` je JEDAN blok markupa pozvan na DVA mjesta (iznad tablice za dodavanje,
  // UNUTAR retka za uređivanje) — isti obrazac, ne kopija (Ruling dopune #2). Jedan `formTarget`
  // (`'new' | indeks retka | null`) drži koji je obrazac otvoren: dodjela nove vrijednosti automatski
  // zatvori bilo koji drugi (Ruling dopune #2: istodobno najviše jedan obrazac otvoren) jer je to
  // JEDINA varijabla koja odlučuje što se prikazuje. `Vision` nema `id` u `types.ts` (ni u
  // `model.rs`: `title/source/state/percent/note`) — identitet retka je INDEKS u `report.visions`,
  // isti obrazac kao `confirmDeleteIndex`. `Vision.state` je slobodan tekst (`model.rs`: `state:
  // String`, ne enum) — korisnik ga upisuje u obrazac (i tako "promijeni stanje" iz spec-a 6.2 ide
  // kroz ISTO polje, bez posebnog brzog izbornika), sučelje ga NE prevodi kroz i18n. `saveVisions`
  // uvijek šalje CIJEL popis (ime parametra u brifu je "sve") — `replaceVisionAt`
  // (`views/helpers.ts`) čisto zamjenjuje jedan redak bez mutacije ulaza. Brisanje NE koristi
  // `window.confirm` (Ruling brifa #6 — WebView2/Playwright ga loše podnose): dvokoračni gumb u
  // retku ("Obriši" → "Potvrdi"/"Odustani") ostaje unutar iste tablice.
  import { app, loadReport } from '../lib/state.svelte';
  import { api } from '../lib/api';
  import { getLang, t } from '../lib/i18n/index.svelte';
  import { num, percent } from '../lib/format';
  import { replaceVisionAt } from './helpers';
  import type { Vision } from '../lib/types';

  // `null` = nijedan obrazac otvoren; `'new'` = dodavanje; broj = indeks retka koji se uređuje.
  let formTarget = $state<'new' | number | null>(null);
  let formTitle = $state('');
  let formSource = $state('');
  let formState = $state('');
  let formPercent = $state('');
  let formNote = $state('');
  let writing = $state(false);
  let confirmDeleteIndex = $state<number | null>(null);

  function openAddForm(): void {
    formTarget = 'new';
    formTitle = '';
    formSource = '';
    formState = '';
    formPercent = '';
    formNote = '';
  }

  function openEditForm(index: number, vision: Vision): void {
    formTarget = index;
    formTitle = vision.title;
    formSource = vision.source;
    formState = vision.state;
    formPercent = vision.percent === null ? '' : String(vision.percent);
    formNote = vision.note;
  }

  function closeForm(): void {
    formTarget = null;
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

  // Jedna funkcija za OBA smjera obrasca: novi redak se dodaje na kraj, uređeni redak zamjenjuje
  // svoje mjesto preko `replaceVisionAt` — u oba slučaja `saveVisions` dobiva CIJEL popis (S-010).
  async function saveForm(): Promise<void> {
    const id = app.currentId;
    if (id === null || writing || formTarget === null || formTitle.trim() === '') return;
    const edited: Vision = {
      title: formTitle.trim(),
      source: formSource.trim(),
      state: formState.trim(),
      percent: parsePercent(formPercent),
      note: formNote.trim(),
    };
    const current = app.report?.visions ?? [];
    const next = formTarget === 'new' ? [...current, edited] : replaceVisionAt(current, formTarget, edited);
    writing = true;
    try {
      await api.saveVisions(id, next);
      await loadReport();
      formTarget = null;
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

{#snippet visionForm()}
  <!-- ŠIRINA (pravilo #9, Ruling dopune #5): `max-w-2xl` vrijedi ISTO za dodavanje i uređivanje jer
       je ovo JEDAN blok markupa — obrazac ne razvlači ni redak (kad je unutar `<td colspan>`) ni
       `main` vodoravno. -->
  <form
    class="flex max-w-2xl flex-col gap-3 rounded-lg border border-line bg-surface-1 p-3"
    onsubmit={(e) => {
      e.preventDefault();
      void saveForm();
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
        onclick={closeForm}
        disabled={writing}
      >
        {t('visions.cancel')}
      </button>
    </div>
  </form>
{/snippet}

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
      onclick={openAddForm}
      disabled={formTarget !== null}
    >
      {t('visions.add')}
    </button>

    {#if formTarget === 'new'}
      {@render visionForm()}
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
            <th scope="col" class="w-48 py-1 pr-3">
              <span class="sr-only">{t('visions.edit')} / {t('visions.delete')}</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {#each app.report.visions as vision, i (vision.title + '::' + i)}
            {#if formTarget === i}
              <!-- Uređivanje "u mjestu": ISTI `visionForm` snippet, sad unutar retka koji se uređuje
                   (Ruling dopune #1/#2) — `colspan` premošćuje sve stupce jer je obrazac vlastiti grid. -->
              <tr class="border-t border-line">
                <td colspan="6" class="py-2 pr-3">
                  {@render visionForm()}
                </td>
              </tr>
            {:else}
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
                    <span class="inline-flex items-center gap-3">
                      <button
                        type="button"
                        class="text-xs text-ink-blue underline decoration-dotted hover:text-ink-0 disabled:opacity-50"
                        disabled={formTarget !== null}
                        aria-label={`${t('visions.edit')}: ${vision.title}`}
                        onclick={() => openEditForm(i, vision)}
                      >
                        {t('visions.edit')}
                      </button>
                      <button
                        type="button"
                        class="text-xs text-danger-ink underline decoration-dotted hover:opacity-80 disabled:opacity-50"
                        disabled={formTarget !== null}
                        aria-label={`${t('visions.delete')}: ${vision.title}`}
                        onclick={() => askDelete(i)}
                      >
                        {t('visions.delete')}
                      </button>
                    </span>
                  {/if}
                </td>
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    {/if}
  {/if}
</div>
