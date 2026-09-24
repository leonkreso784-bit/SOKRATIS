<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/45 — upit pri zatvaranju, S-036)
  // Vlastiti dijalog (ne `tauri-plugin-dialog::ask`) jer natpis, gumbi i boje idu kroz rječnik i
  // tokene kao ostatak sučelja; `role="dialog" aria-modal="true"` + fokus na „Odustani" pri otvaranju
  // (sigurniji gumb dobiva Enter), `Esc` = odustani. Ništa se ovdje ne odlučuje — roditelj (`App.svelte`)
  // zove `api.quit()` na potvrdu.
  import { t } from '../i18n/index.svelte';

  let { onconfirm, oncancel }: { onconfirm: () => void; oncancel: () => void } = $props();
  let cancelButton = $state<HTMLButtonElement | null>(null);

  $effect(() => {
    cancelButton?.focus();
  });

  function onKeydown(e: KeyboardEvent): void {
    if (e.key === 'Escape') oncancel();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div class="fixed inset-0 z-50 grid place-items-center bg-ink-0/40" role="presentation">
  <div
    role="dialog"
    aria-modal="true"
    aria-labelledby="quit-title"
    class="w-[420px] max-w-[90vw] rounded-lg border border-line bg-surface-1 p-5 shadow-e2"
  >
    <h2 id="quit-title" class="text-lg font-semibold text-ink-0">{t('quit.title')}</h2>
    <p class="mt-2 text-sm text-ink-1">{t('quit.body')}</p>
    <div class="mt-5 flex justify-end gap-2">
      <button
        type="button"
        bind:this={cancelButton}
        class="rounded-md px-3 py-1.5 text-sm text-ink-1 hover:bg-surface-2"
        onclick={oncancel}
      >
        {t('quit.cancel')}
      </button>
      <button
        type="button"
        class="rounded-md bg-brand-500 px-3 py-1.5 text-sm font-medium text-on-brand hover:bg-brand-600"
        onclick={onconfirm}
      >
        {t('quit.confirm')}
      </button>
    </div>
  </div>
</div>
