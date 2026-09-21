<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — četiri kruga, bojaju se iz tokena, ne iz Tailwind klasa)
  // `--theme-swatch-*` u `tokens.css` NISU u `--color-*` imenskom prostoru (namjerno — krug za
  // "chalk" mora ostati tamnog izgleda i dok je aktivna "academic" tema), pa Tailwind za njih ne
  // gradi utility-klase. `var(--theme-swatch-{tema}-bg)` je i dalje boja SAMO iz tokena (S-017),
  // samo primijenjena inline stilom jer se ime tokena mijenja po krugu u petlji.
  import { app, applyTheme, setError } from '../state.svelte';
  import { api } from '../api';
  import { t } from '../i18n/index.svelte';
  import type { Theme } from '../types';

  const themes: readonly Theme[] = ['academic', 'chalk', 'mint', 'carbon'];

  async function choose(theme: Theme): Promise<void> {
    app.settings = { ...app.settings, theme };
    applyTheme(theme);
    try {
      await api.setSetting('theme', theme);
    } catch (e) {
      setError(e);
    }
  }
</script>

<div class="flex items-center gap-1.5" role="group" aria-label={t('top.theme')}>
  {#each themes as theme (theme)}
    <button
      type="button"
      class="h-6 w-6 rounded-full border-2"
      style="background: var(--theme-swatch-{theme}-bg); border-color: var(--theme-swatch-{theme}-acc);
        {app.settings.theme === theme ? 'outline: 2px solid var(--color-brand-500); outline-offset: 2px;' : ''}"
      aria-label={t(`theme.${theme}`)}
      aria-pressed={app.settings.theme === theme}
      onclick={() => choose(theme)}
    ></button>
  {/each}
</div>
