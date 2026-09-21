<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — HR/EN kao dva gumba, ne padajući izbornik)
  // Samo dvije vrijednosti postoje (S-021) — gumb po jeziku je jednostavniji za klik i čitljiviji
  // za a11y (`aria-pressed`) nego `<select>` s dvije stavke. `setLang` mijenja `document.documentElement
  // .lang` (u `i18n/index.svelte.ts`), a `app.settings.lang` prati istu vrijednost radi `getSettings`.
  import { app } from '../state.svelte';
  import { api } from '../api';
  import { setLang, t } from '../i18n/index.svelte';
  import type { Lang } from '../types';

  const langs: readonly Lang[] = ['hr', 'en'];

  async function choose(lang: Lang): Promise<void> {
    app.settings = { ...app.settings, lang };
    setLang(lang);
    await api.setSetting('lang', lang);
  }
</script>

<div class="flex items-center gap-1" role="group" aria-label={t('top.lang')}>
  {#each langs as lang (lang)}
    <button
      type="button"
      class="rounded-md px-2 py-1 text-xs {app.settings.lang === lang
        ? 'bg-brand-500 text-on-brand'
        : 'text-ink-1 hover:bg-surface-2'}"
      aria-pressed={app.settings.lang === lang}
      onclick={() => choose(lang)}
    >
      {t(`lang.${lang}`)}
    </button>
  {/each}
</div>
