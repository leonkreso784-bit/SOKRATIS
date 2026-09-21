<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — gornja traka: znak, birač projekta, raspon, osvježi, tema, jezik)
  // `<Topbar>` čita/piše izravno globalnu runu `app` (`state.svelte.ts`) — bez callback-propsa kao
  // u starijem Svelteu: promjena `app.currentId` ovdje se odmah vidi u `<Sidebar>` i `<main>`.
  import markUrl from '../../assets/intro/mark.webp';
  import { app, loadReport, selectProject } from '../state.svelte';
  import { api } from '../api';
  import { t } from '../i18n/index.svelte';
  import RangePicker from './RangePicker.svelte';
  import ThemeSwitch from './ThemeSwitch.svelte';
  import LangSwitch from './LangSwitch.svelte';

  function onProjectChange(e: Event & { currentTarget: HTMLSelectElement }): void {
    const id = Number(e.currentTarget.value);
    if (!Number.isNaN(id)) void selectProject(id);
  }

  async function refresh(): Promise<void> {
    await api.refresh(app.currentId ?? undefined);
    if (app.currentId !== null) await loadReport();
  }
</script>

<header class="flex h-16 shrink-0 items-center gap-4 border-b border-line bg-surface-1 px-4">
  <!-- Lockup je ime proizvoda, ne natpis — ne ide kroz t() (isto vrijedi za "S"/"KRATIS" niže). -->
  <div class="flex items-center gap-1 font-display text-lg font-semibold text-ink-0" aria-label="Sokratis">
    <span>S</span>
    <img src={markUrl} alt="" class="h-4 w-4" />
    <span>KRATIS</span>
  </div>

  {#if app.projects.length > 0}
    <select
      class="rounded-md border border-line bg-surface-1 px-2 py-1 text-sm text-ink-0"
      aria-label={t('top.project')}
      value={app.currentId ?? ''}
      onchange={onProjectChange}
    >
      <option value="" disabled>{t('top.project')}</option>
      {#each app.projects as project (project.id)}
        <option value={project.id}>{project.name}</option>
      {/each}
    </select>
  {/if}

  <div class="flex flex-1 items-center justify-end gap-4">
    <RangePicker />
    <button type="button" class="rounded-md px-2 py-1 text-xs text-ink-1 hover:bg-surface-2" onclick={() => void refresh()}>
      {t('top.refresh')}
    </button>
    <ThemeSwitch />
    <LangSwitch />
  </div>
</header>
