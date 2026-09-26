<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/26 — Pregled: kartice projekata sa signalima; dopunjeno M2/34 — pretplata
  // na `report_updated` je preseljena u `App.svelte`, globalno za CIJELI okvir, ne samo ovaj pogled —
  // vidi zaglavlje ondje)
  // Prvi pogled (spec §6.1) čita `app.projects` izravno iz globalne rune (S-010: `state.svelte.ts`
  // je jedini izvor) i samo ga sortira/oblikuje za prikaz — `helpers.ts` nosi tu logiku odvojeno da
  // je vitest testira bez Svelte runtimea. `prompt()`/`confirm()` su WebView2-ovi ugrađeni dijalozi
  // (brief T26) — dovoljni dok pravi Tauri dijalog za preimenovanje/uklanjanje ne postane potreban.
  // Dopunjeno M2/42 — kartica projekta gubi ugniježđene gumbe (R26): gumb odabira sad omata SAMO
  // naslov, a tri crte (stabla · zadnji commit · signali) su mu sestre, svaka u vlastitom
  // `<Explainable>` — `<button>` unutar `<button>` je nevaljan HTML.
  // Dopunjeno M2/58 (S-034) — klik na naslov kartice sad zove `enterProject` (ne `selectProject`):
  // vodi izravno na ploču projekta, ne samo na odabir bez promjene pogleda.
  import { api } from '../lib/api';
  import { app, enterProject, loadProjects, setError } from '../lib/state.svelte';
  import { getDict, t } from '../lib/i18n/index.svelte';
  import { relative } from '../lib/format';
  import { severityClass, signalSummary, sortProjects } from './helpers';
  import Explainable from '../lib/explain/Explainable.svelte';
  import type { ProjectSummary } from '../lib/types';

  function lastCommitText(project: ProjectSummary): string {
    if (!project.last_commit) return t('overview.noReport');
    const when = relative(Date.now() / 1000, project.last_commit.time, getDict());
    return t('overview.lastCommit', { when });
  }

  async function addProject(): Promise<void> {
    try {
      const added = await api.addProject();
      if (added) await loadProjects();
    } catch (e) {
      setError(e);
    }
  }

  async function onRename(project: ProjectSummary): Promise<void> {
    const name = prompt(t('overview.renamePrompt', { name: project.name }), project.name);
    if (!name || !name.trim() || name.trim() === project.name) return;
    try {
      await api.renameProject(project.id, name.trim());
      await loadProjects();
    } catch (e) {
      setError(e);
    }
  }

  async function onRemove(project: ProjectSummary): Promise<void> {
    if (!confirm(t('overview.removeConfirm', { name: project.name }))) return;
    try {
      await api.removeProject(project.id);
      await loadProjects();
    } catch (e) {
      setError(e);
    }
  }
</script>

<div class="flex flex-col gap-4">
  <div class="flex items-center justify-between">
    <h1 class="text-xl font-semibold text-ink-0">{t('nav.overview')}</h1>
    <button
      type="button"
      class="rounded-md bg-brand-500 px-3 py-1.5 text-sm font-medium text-on-brand hover:bg-brand-600"
      onclick={() => void addProject()}
    >
      {t('overview.add')}
    </button>
  </div>

  {#if app.projects.length === 0}
    <p class="text-sm text-ink-2">{t('overview.empty')}</p>
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each sortProjects(app.projects) as project (project.id)}
        <div class="relative flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
          <button
            type="button"
            class="w-full pr-6 text-left"
            aria-label={project.name}
            onclick={() => void enterProject(project.id)}
          >
            <h2 class="text-lg font-semibold text-ink-0">{project.name}</h2>
          </button>
          <p class="text-sm text-ink-1">
            <Explainable id="overview.worktrees">{t('overview.worktrees', { n: project.worktrees })}</Explainable>
          </p>
          <p class="text-sm text-ink-2">
            <Explainable id="overview.last_commit">{lastCommitText(project)}</Explainable>
          </p>
          <p class="text-sm font-medium {severityClass(project.worst)}">
            <Explainable id="overview.signals">{signalSummary(project.signals, getDict())}</Explainable>
          </p>
          {#if project.error}
            <p class="text-sm text-danger-ink">{t('overview.error', { msg: project.error })}</p>
          {/if}

          <details class="absolute right-2 top-2">
            <summary
              class="grid h-8 w-8 cursor-pointer list-none place-items-center rounded-md text-ink-2 hover:bg-surface-2"
              aria-label={t('overview.menu', { name: project.name })}
            >
              …
            </summary>
            <div class="absolute right-0 z-10 mt-1 flex w-36 flex-col gap-1 rounded-md border border-line bg-surface-1 p-1 shadow-e2">
              <button
                type="button"
                class="rounded px-2 py-1 text-left text-sm text-ink-1 hover:bg-surface-2"
                onclick={() => void onRename(project)}
              >
                {t('overview.rename')}
              </button>
              <button
                type="button"
                class="rounded px-2 py-1 text-left text-sm text-danger-ink hover:bg-surface-2"
                onclick={() => void onRemove(project)}
              >
                {t('overview.remove')}
              </button>
            </div>
          </details>
        </div>
      {/each}
    </div>
  {/if}
</div>
