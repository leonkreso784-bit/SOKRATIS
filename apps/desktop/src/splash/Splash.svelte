<!-- ZAŠTO OVAKO (cigla M2/24 — splash; krug popravka 1): splash se NIKAD ne smije zaglaviti — -->
<!-- `announce` ide kroz `once()` pa `splash:done` stigne točno jednom, što god se dogodi (kraj, -->
<!-- klik, tipka i prije i poslije učitavanja slika, ili greška učitavanja); DESKTOP T31 sluša. -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { once, startIntro } from './intro';
  import markUrl from '../assets/intro/mark.webp';
  import graphUrl from '../assets/intro/graph.webp';

  let canvas: HTMLCanvasElement;
  let intro: { skip(): void } | null = null;
  // Postavlja je `announce` (kroz `once`) — sprječava da `startIntro` krene NAKON što je
  // korisnik već preskočio dok su se slike još učitavale.
  let finished = false;

  const load = (src: string) =>
    new Promise<HTMLImageElement>((ok, err) => {
      const i = new Image();
      i.onload = () => ok(i);
      i.onerror = err;
      i.src = src;
    });

  // `__TAURI_INTERNALS__` postoji samo unutar Tauri prozora; u pregledniku (npr. `npm run dev`)
  // se `emit` ne smije ni pozvati, pa dinamički uvoz `@tauri-apps/api/event` ide iza te straže.
  // Greška u slanju (npr. `emit` odbije) ostaje ovdje — `announce` se svejedno smatra izvršenim.
  async function emitDone() {
    if (!('__TAURI_INTERNALS__' in window)) {
      console.info('splash:done (bez Taurija)');
      return;
    }
    try {
      const { emit } = await import('@tauri-apps/api/event');
      await emit('splash:done');
    } catch (e) {
      console.error('splash: slanje splash:done nije uspjelo', e);
    }
  }

  // JEDINO mjesto koje javlja kraj splasha — `once` jamči „točno jednom" bez obzira odakle se
  // pozove (kraj animacije, klik, tipka, greška učitavanja slike).
  const announce = once(() => {
    finished = true;
    void emitDone();
  });

  // Klik/tipka: dok slike još nisu učitane `intro` je `null`, pa preskakanje znači odmah javiti
  // kraj (S-019 — mora se moći preskočiti i prije nego što se ima što animirati).
  function skip() {
    if (intro) {
      intro.skip();
    } else {
      announce();
    }
  }

  onMount(async () => {
    try {
      const [mark, graph] = await Promise.all([load(markUrl), load(graphUrl)]);
      // Korisnik je mogao preskočiti dok smo čekali slike — tad se intro više ne smije pokrenuti.
      if (finished) return;
      intro = startIntro(
        canvas,
        { mark, graph },
        { onDone: announce, reducedMotion: matchMedia('(prefers-reduced-motion: reduce)').matches },
      );
    } catch (e) {
      console.error('splash: slike se nisu učitale, preskačem animaciju', e);
      announce();
    }
  });
</script>

<svelte:window onkeydown={skip} />
<div class="stage" onclick={skip} role="presentation">
  <!-- svelte-ignore a11y_no_interactive_element_to_noninteractive_role -->
  <!-- `<canvas>` je po HTML5 "interaktivan sadržaj" (može sadržavati fallback), ali ovaj nema
       nikakvu interakciju — role="img" + aria-label je standardni WAI-ARIA obrazac za canvas. -->
  <canvas bind:this={canvas} width="1000" height="560" role="img" aria-label="Sokratis"></canvas>
</div>

<style>
  .stage {
    background: #0b1017;
    width: 100vw;
    height: 100vh;
    display: grid;
    place-items: center;
    cursor: pointer;
  }
  canvas {
    width: 100%;
    height: auto;
  }
</style>
