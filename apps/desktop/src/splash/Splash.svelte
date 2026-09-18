<!-- ZAŠTO OVAKO (cigla M2/24 — splash): komponenta samo učitava dvije slike i pokreće -->
<!-- `startIntro`; klik/tipka prekida (S-019), kraj javlja `splash:done` koji DESKTOP T31 sluša. -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { startIntro } from './intro';
  import markUrl from '../assets/intro/mark.webp';
  import graphUrl from '../assets/intro/graph.webp';

  let canvas: HTMLCanvasElement;
  let intro: { skip(): void } | null = null;

  const load = (src: string) =>
    new Promise<HTMLImageElement>((ok, err) => {
      const i = new Image();
      i.onload = () => ok(i);
      i.onerror = err;
      i.src = src;
    });

  // `__TAURI_INTERNALS__` postoji samo unutar Tauri prozora; u pregledniku (npr. `npm run dev`)
  // se `emit` ne smije ni pozvati, pa dinamički uvoz `@tauri-apps/api/event` ide iza te straže.
  async function announce() {
    if (!('__TAURI_INTERNALS__' in window)) {
      console.info('splash:done (bez Taurija)');
      return;
    }
    const { emit } = await import('@tauri-apps/api/event');
    await emit('splash:done');
  }

  onMount(async () => {
    const [mark, graph] = await Promise.all([load(markUrl), load(graphUrl)]);
    intro = startIntro(
      canvas,
      { mark, graph },
      { onDone: announce, reducedMotion: matchMedia('(prefers-reduced-motion: reduce)').matches },
    );
  });
</script>

<svelte:window onkeydown={() => intro?.skip()} />
<div class="stage" onclick={() => intro?.skip()} role="presentation">
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
