<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/25 — okvir: gornja traka + lijevi izbornik + prostor za poglede;
  // dopunjeno M2/26 — Pregled je prvi stvaran pogled, traka signala sjedi između gornje trake i
  // retka Sidebar+main; dopunjeno M2/28 — svih devet pogleda iz `VIEWS` (`types.ts`) sada ima svoju
  // komponentu, pa je grana `{:else}` s `common.loading` UKLONJENA jer više nije dostižna ni za
  // jedan mogući `View`: `app.view` je zatvoren TS-enum, if/else-if lanac ovdje ga pokriva u cijelosti;
  // dopunjeno M2/34 — pretplata na `report_updated` je OVDJE, ne u `Overview.svelte`: promjena u
  // repou mora osvježiti pogled u kojem korisnik TRENUTNO stoji, ne samo Pregled; dopunjeno M2/38 —
  // `syncMotion` sluša `prefers-reduced-motion` odmah nakon `applyTheme`, a ruta `settings` je deseti,
  // globalni pogled koji `Sidebar` crta i bez odabranog projekta (R22); dopunjeno M2/39 — `{#key
  // app.epoch}` oko lanca pogleda tjera Svelte da poglede DEMONTIRA i ponovno MONTIRA kad `epoch`
  // poraste s 0 na 1, pa se ulazna animacija grafova (S-026) odigra tek tad, ne dok je glavni prozor
  // još skriven iza splasha (T31). Dva NEOVISNA čuvara javljaju taj trenutak jer nijedan sam nije
  // pouzdan u WebView2: `visibilitychange` prati `document.visibilityState`, koji Windows/WebView2
  // zna držati na "visible" i za skriveni HWND, pa se ne mora nikad promijeniti; `onFocusChanged`
  // prati stvarni fokus prozora, koji Rust postavlja u `splash.rs` (`main.show()` + `main.set_focus()`)
  // TOČNO kad se splash zatvori. Oba su idempotentna (`if (app.epoch === 0)`) pa koji god okine prvi
  // pobjeđuje, a drugi ne radi ništa.
  // `onMount` je Svelteov standardni "kad je komponenta u DOM-u" udarac — ovdje je to JEDINO mjesto
  // koje povlači početne postavke i popis projekata (S-010: jedno mjesto pokretanja, ne u svakoj
  // podkomponenti). Umotan u `try/catch` (dopuna T34): pad `getSettings` (npr. baza nedostupna) više
  // ne smije ostaviti prazan prozor bez ijedne poruke — greška ide u traku ispod.
  // Dopunjeno M2/40 — ključ `{#key}` sada nosi i `app.view`: promjena pogleda dobiva isti ulaz
  // (`view-enter`) kao promjena `epoch`, `is-loading` prigušuje stari sadržaj dok raspon učitava
  // novi, a `Skeleton` zamjenjuje poglede SAMO dok prvi izvještaj još nije stigao.
  // Dopunjeno M2/41 — `<ExplainCard />` sjedi IZVAN `{#key}`, na dnu korijenskog `<div>`: promjena
  // pogleda ne smije remontirati karticu s objašnjenjem dok se ona sama zatvara.
  // Dopunjeno M2/45 — X na prozoru više ne skriva tray (ukinut, S-036): `api.onCloseRequested`
  // postavlja `quitOpen`, `<ConfirmQuit />` pita, a potvrda zove `api.quit()`. Dijalog je IZA
  // `<ExplainCard />`, iz istog razloga kao ona — ništa se ne smije remontirati oko njega.
  // Dopunjeno M2/58 (S-034) — šest prikaznih pogleda (Tempo · Vrste rada · Pokazatelji · Faze ·
  // Isporuke · Dokumentacija) postaju sekcije JEDNE ploče, `views/Project.svelte`; lanac pogleda
  // ovdje ima sad PET grana: `overview | project | diary | visions | settings`.
  import { onDestroy, onMount } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { api } from './lib/api';
  import { app, applyTheme, dismissError, loadProjects, loadReport, setError, syncMotion } from './lib/state.svelte';
  import { setLang, t } from './lib/i18n/index.svelte';
  import Topbar from './lib/shell/Topbar.svelte';
  import Sidebar from './lib/shell/Sidebar.svelte';
  import SignalBar from './lib/shell/SignalBar.svelte';
  import Overview from './views/Overview.svelte';
  import Project from './views/Project.svelte';
  import Diary from './views/Diary.svelte';
  import Visions from './views/Visions.svelte';
  import Settings from './views/Settings.svelte';
  import Skeleton from './lib/shell/Skeleton.svelte';
  import ExplainCard from './lib/explain/ExplainCard.svelte';
  import ConfirmQuit from './lib/shell/ConfirmQuit.svelte';

  let unsubscribeReportUpdated: (() => void) | null = null;
  let unsubscribeClose: (() => void) | null = null;
  let motionQuery: MediaQueryList | undefined;
  let unlistenFocus: (() => void) | null = null;
  let quitOpen = $state(false);

  // Prvi čuvar (svugdje, uklj. preglednik): dokument je vidljiv od početka u `npm run dev`, pa
  // `epoch` odmah pređe na 1 i animacija se vidi pri montiranju — to je ispravno ondje gdje splash
  // ne postoji.
  const onVisible = () => {
    if (document.visibilityState === 'visible' && app.epoch === 0) app.epoch = 1;
  };

  onMount(async () => {
    try {
      app.settings = await api.getSettings();
      applyTheme(app.settings.theme);
      syncMotion();
      motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
      motionQuery.addEventListener('change', syncMotion);
      setLang(app.settings.lang);
      await loadProjects();
    } catch (e) {
      setError(e);
    }
    // Globalno, jednom (dopuna T34) — na svaku promjenu bilo kojeg projekta osvježi popis, a AKO je
    // to baš projekt koji korisnik trenutno gleda, učitaj i njegov izvještaj iznova.
    unsubscribeReportUpdated = api.onReportUpdated((id) => {
      void loadProjects();
      if (id === app.currentId) void loadReport();
    });

    unsubscribeClose = api.onCloseRequested(() => {
      quitOpen = true;
    });

    document.addEventListener('visibilitychange', onVisible);
    // Drugi čuvar, SAMO unutar Taurija (isti test kao `api.ts`/`Splash.svelte`): `onFocusChanged`
    // stiže sa `splash.rs` `main.show()` + `main.set_focus()`, neovisno o tome je li WebView2 uopće
    // okinuo `visibilitychange` za dotad skriveni prozor.
    if ('__TAURI_INTERNALS__' in window) {
      void getCurrentWindow()
        .onFocusChanged(({ payload }) => {
          if (payload && app.epoch === 0) app.epoch = 1;
        })
        .then((off) => {
          unlistenFocus = off;
        });
    }
  });

  onDestroy(() => {
    unsubscribeReportUpdated?.();
    unsubscribeClose?.();
    motionQuery?.removeEventListener('change', syncMotion);
    document.removeEventListener('visibilitychange', onVisible);
    unlistenFocus?.();
  });
</script>

<div class="flex h-screen flex-col bg-surface-0 text-ink-0">
  <Topbar />
  {#if app.error}
    <!-- Boje SAMO iz tokena (`text-danger-ink` već je dio brane kontrasta, dopuna T34) — dugačka
         poruka (npr. puna Windows putanja) se LOMI (`min-w-0` + `break-words` na fleksnom djetetu)
         umjesto da širi prozor ispod `minWidth` iz `tauri.conf.json` (960 px); ova cigla ne pokreće
         `tauri dev`, pa je stvarni izgled na 960 px na ručnoj listi (T34-dopuna, korak 2), ne ovdje. -->
    <div role="alert" class="flex items-center justify-between gap-3 border-b border-line bg-surface-1 px-4 py-2 text-sm">
      <span class="min-w-0 flex-1 break-words text-danger-ink">{t('common.error', { msg: app.error })}</span>
      <button
        type="button"
        class="shrink-0 rounded-md px-2 py-1 text-xs text-danger-ink hover:bg-surface-2"
        onclick={dismissError}
      >
        {t('common.dismiss')}
      </button>
    </div>
  {/if}
  <SignalBar />
  <div class="flex min-h-0 flex-1">
    <Sidebar />
    <main class="flex-1 overflow-auto p-4">
      {#key `${app.epoch}:${app.view}`}
        <div class="view-enter" class:is-loading={app.loading && app.report !== null}>
          {#if app.loading && app.report === null && app.currentId !== null}
            <Skeleton />
          {:else if app.view === 'overview'}
            <Overview />
          {:else if app.view === 'project'}
            <Project />
          {:else if app.view === 'diary'}
            <Diary />
          {:else if app.view === 'visions'}
            <Visions />
          {:else if app.view === 'settings'}
            <Settings />
          {/if}
        </div>
      {/key}
    </main>
  </div>
  <ExplainCard />
  {#if quitOpen}
    <ConfirmQuit onconfirm={() => void api.quit()} oncancel={() => (quitOpen = false)} />
  {/if}
</div>
