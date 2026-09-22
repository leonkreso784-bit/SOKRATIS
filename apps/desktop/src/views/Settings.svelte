<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/38 — pogled Postavke: tema, jezik, autostart, animacije, spec §13.2, S-028)
  // Deseti, globalni pogled (`types.ts`): NE čita `app.report`, radi i bez odabranog projekta i bez
  // izvještaja (R22). `ThemeSwitch`/`LangSwitch` sele OVAMO bez promjene ponašanja (R23) — samo
  // mijenjaju roditelja, gornja traka ih više ne crta. `writing` gasi autostart/animacije dok upis
  // traje (isti obrazac kao `Diary.svelte`, T28) da dvostruki klik ne pošalje dva zahtjeva odjednom;
  // `ThemeSwitch`/`LangSwitch` imaju vlastiti try/catch po kliku, ne dijele ovaj `writing`.
  import { app, setError, syncMotion } from '../lib/state.svelte';
  import { api } from '../lib/api';
  import { t } from '../lib/i18n/index.svelte';
  import ThemeSwitch from '../lib/shell/ThemeSwitch.svelte';
  import LangSwitch from '../lib/shell/LangSwitch.svelte';

  let writing = $state(false);

  async function setAutostart(value: boolean): Promise<void> {
    if (writing) return;
    writing = true;
    app.settings = { ...app.settings, autostart: value };
    try {
      await api.setSetting('autostart', value);
    } catch (e) {
      setError(e);
    } finally {
      writing = false;
    }
  }

  // Nakon upisa se poziva `syncMotion()` (R24): ista odluka koju `App.svelte` primjenjuje pri
  // pokretanju i na promjenu `prefers-reduced-motion`, ovdje samo ponovno pokrenuta jer se `motion`
  // promijenio.
  async function setMotion(value: boolean): Promise<void> {
    if (writing) return;
    writing = true;
    app.settings = { ...app.settings, motion: value };
    try {
      await api.setSetting('motion', value);
      syncMotion();
    } catch (e) {
      setError(e);
    } finally {
      writing = false;
    }
  }
</script>

<div class="flex max-w-xl flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('settings.title')}</h1>

  <div class="flex items-center justify-between gap-4 border-b border-line py-3">
    <span class="text-sm text-ink-1">{t('settings.theme')}</span>
    <ThemeSwitch />
  </div>

  <div class="flex items-center justify-between gap-4 border-b border-line py-3">
    <span class="text-sm text-ink-1">{t('settings.lang')}</span>
    <LangSwitch />
  </div>

  <div class="flex items-center justify-between gap-4 border-b border-line py-3">
    <div>
      <p class="text-sm text-ink-1">{t('settings.autostart')}</p>
      <p class="text-xs text-ink-2">{t('settings.autostart.hint')}</p>
    </div>
    <input
      type="checkbox"
      role="switch"
      class="h-4 w-4 accent-brand-500"
      aria-label={t('settings.autostart')}
      checked={app.settings.autostart}
      disabled={writing}
      onchange={(e) => void setAutostart(e.currentTarget.checked)}
    />
  </div>

  <div class="flex items-center justify-between gap-4 py-3">
    <div>
      <p class="text-sm text-ink-1">{t('settings.motion')}</p>
      <p class="text-xs text-ink-2">{t('settings.motion.hint')}</p>
    </div>
    <input
      type="checkbox"
      role="switch"
      class="h-4 w-4 accent-brand-500"
      aria-label={t('settings.motion')}
      checked={app.settings.motion}
      disabled={writing}
      onchange={(e) => void setMotion(e.currentTarget.checked)}
    />
  </div>
</div>
