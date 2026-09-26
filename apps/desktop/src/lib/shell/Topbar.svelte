<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — gornja traka: znak, birač projekta, raspon, osvježi; dopunjeno M2/38 —
  // `ThemeSwitch`/`LangSwitch` sele u pogled Postavke, gornja traka ih više ne crta, R23)
  // `<Topbar>` čita/piše izravno globalnu runu `app` (`state.svelte.ts`) — bez callback-propsa kao
  // u starijem Svelteu: promjena `app.currentId` ovdje se odmah vidi u `<Sidebar>` i `<main>`.
  // Dopunjeno M2/58 (S-034): birač projekta (`<select>`) je OBRISAN — ulaz u projekt je sad SAMO
  // klik na karticu Pregleda (`enterProject`). Na njegovo mjesto dolazi gumb „‹ Pregled" (`leaveProject`)
  // koji se vidi kad je korisnik unutar ploče projekta ili Dnevnika/Vizija — ime projekta uz njega
  // čita se iz `app.projects` po `app.currentId` (isti popis koji je select ranije crtao kao opcije).
  import markUrl from '../../assets/intro/mark.webp';
  import { app, leaveProject, loadReport, setError } from '../state.svelte';
  import { api } from '../api';
  import { t } from '../i18n/index.svelte';
  import RangePicker from './RangePicker.svelte';

  const currentProjectName = $derived(app.projects.find((p) => p.id === app.currentId)?.name ?? '');

  async function refresh(): Promise<void> {
    try {
      await api.refresh(app.currentId ?? undefined);
      if (app.currentId !== null) await loadReport();
    } catch (e) {
      setError(e);
    }
  }
</script>

<header class="flex h-16 shrink-0 items-center gap-4 border-b border-line bg-surface-1 px-4">
  <!-- Lockup je ime proizvoda, ne natpis — ne ide kroz t() (isto vrijedi za "S"/"KRATIS" niže). -->
  <div class="flex items-center gap-1 font-display text-lg font-semibold text-ink-0" aria-label="Sokratis">
    <span>S</span>
    <img src={markUrl} alt="" class="h-4 w-4" />
    <span>KRATIS</span>
  </div>

  {#if app.view !== 'overview' && app.view !== 'settings' && app.currentId !== null}
    <button type="button" class="rounded-md px-2 py-1 text-sm text-ink-1 hover:bg-surface-2" onclick={leaveProject}>
      <!-- „‹" je interpunkcija (strelica natrag), ne natpis — ne ide kroz t(). -->
      ‹ {t('top.back')}
    </button>
    <span class="text-sm font-medium text-ink-0">{currentProjectName}</span>
  {/if}

  <div class="flex flex-1 items-center justify-end gap-4">
    <RangePicker />
    <button type="button" class="rounded-md px-2 py-1 text-xs text-ink-1 hover:bg-surface-2" onclick={() => void refresh()}>
      {t('top.refresh')}
    </button>
  </div>
</header>
