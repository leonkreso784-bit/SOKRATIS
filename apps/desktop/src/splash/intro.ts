// ZAŠTO OVAKO (cigla M2/24 — splash)
// Vremenska crta je ČISTA funkcija `frame(ms)` (testira se bez platna), `paint` je doslovan port
// Leonova canvas-koda (sokratis-intro-clean-graph.html, 2026-09-18) — brojke se ne „poboljšavaju".
// Boje su HEKS namjerno (canvas ne čita tokene, iznimka od S-017): `#0b1017`/`#17212b`/`#00dce8`.
// `requestAnimationFrame` petlja stane na TOTAL_MS ili `skip()`; reduced-motion crta zadnji kadar.
export const TOTAL_MS = 4200;

// Krug popravka 1 (recenzija): `splash:done` mora stići TOČNO JEDNOM što god se dogodilo (kraj,
// klik, tipka, greška učitavanja) — `once` je zajednička brava koju `Splash.svelte` stavlja oko
// jedinog mjesta koje taj događaj šalje.
export function once(fn: () => void): () => void {
  let called = false;
  return () => {
    if (called) return;
    called = true;
    fn();
  };
}

// `clamp` čuva NaN (nedefinirano vrijeme) na 0 umjesto da ga propusti kroz Math.min/max, koji s NaN-om
// vraćaju NaN — izvan Leonova izvornika (koji nikad ne zove paint s NaN-om), dodano radi otpornosti.
const clamp = (x: number): number => (Number.isNaN(x) ? 0 : Math.max(0, Math.min(1, x)));
const ease = (x: number): number => 1 - (1 - clamp(x)) ** 3;

export type Frame = {
  bars: [number, number, number, number];
  wipe: number;
  wipeAlpha: number;
  whole: number;
  ring: number;
  portrait: number;
  settle: number;
};

export function frame(ms: number): Frame {
  const bars = [0, 1, 2, 3].map((i) => ease((ms - 160 - i * 110) / 650)) as Frame['bars'];
  const reveal = clamp((ms - 2200) / 950);
  return {
    bars,
    wipe: ease((ms - 750) / 750),
    wipeAlpha: clamp((ms - 750) / 420),
    whole: clamp((ms - 1400) / 400),
    ring: ease((ms - 1400) / 650),
    portrait: reveal * reveal * (3 - 2 * reveal),
    settle: ease((ms - 3400) / 650),
  };
}

const SLICES: [number, number, number][] = [
  [210, 395, 765],
  [405, 605, 551],
  [610, 800, 401],
  [815, 1020, 260],
];

export function paint(
  ctx: CanvasRenderingContext2D,
  img: { mark: HTMLImageElement; graph: HTMLImageElement },
  ms: number,
): void {
  const f = frame(ms);
  ctx.clearRect(0, 0, 1000, 560);
  const size = 320 - 190 * f.settle;
  const cx = 500 - 220 * f.settle;
  const cy = 280;
  ctx.save();
  ctx.translate(cx - size / 2, cy - size / 2);
  ctx.scale(size / 1160, size / 1160);
  ctx.translate(-47, -66);
  ctx.save();
  ctx.beginPath();
  ctx.arc(627, 646, 548, 0, Math.PI * 2);
  ctx.clip();
  ctx.fillStyle = '#17212b';
  ctx.fillRect(0, 0, 1280, 1280);
  const draw = () => {
    ctx.drawImage(img.graph, 0, 0, 1280, 1280);
    if (f.portrait > 0) {
      ctx.save();
      ctx.globalAlpha *= f.portrait;
      ctx.drawImage(img.mark, 0, 0, 1280, 1280);
      ctx.restore();
    }
  };
  // 1) stupci — svaki od četiri isječka otkriva `draw()` odozdo prema gore, jedan za drugim
  SLICES.forEach(([l, r, top], i) => {
    // `SLICES` i `f.bars` su uvijek iste duljine (4); `?? 0` čuva `noUncheckedIndexedAccess`
    // bez izjave da vrijednost sigurno postoji.
    const p = f.bars[i] ?? 0;
    ctx.save();
    ctx.beginPath();
    ctx.rect(l, 990 - (990 - top) * p, r - l, (990 - top) * p);
    ctx.clip();
    draw();
    ctx.restore();
  });
  // 2) brisanje — vodoravni "wipe" preko sredine, s vlastitom alfom neovisnom o širini reza
  ctx.save();
  ctx.beginPath();
  ctx.rect(240, 260, 790 * f.wipe, 530);
  ctx.clip();
  ctx.globalAlpha = f.wipeAlpha;
  draw();
  ctx.restore();
  // 3) cijeli znak — puni krug bez isjecanja, samo prosijava (alfa raste s `f.whole`)
  ctx.save();
  ctx.globalAlpha = f.whole;
  draw();
  ctx.restore();
  ctx.restore();
  // 4) prsten — kružni luk oko znaka koji raste s `f.ring`, isjecen na uski prsten (evenodd)
  if (f.ring > 0) {
    ctx.save();
    ctx.beginPath();
    ctx.moveTo(627, 646);
    ctx.arc(627, 646, 610, -Math.PI / 2, -Math.PI / 2 + Math.PI * 2 * f.ring);
    ctx.closePath();
    ctx.clip();
    ctx.beginPath();
    ctx.arc(627, 646, 592, 0, Math.PI * 2);
    ctx.arc(627, 646, 547, 0, Math.PI * 2, true);
    ctx.clip('evenodd');
    draw();
    ctx.restore();
  }
  ctx.restore();
  // 5) natpis — "S◍KRATIS" prelazi preko slova O tek kad se znak skupi (`f.settle`)
  ctx.save();
  ctx.globalAlpha = f.settle;
  ctx.fillStyle = '#00dce8';
  ctx.font = '800 100px system-ui, sans-serif';
  ctx.textBaseline = 'middle';
  ctx.fillText('S', 135, 286);
  ctx.fillText('KRATIS', 355, 286);
  ctx.restore();
}

export function startIntro(
  canvas: HTMLCanvasElement,
  img: { mark: HTMLImageElement; graph: HTMLImageElement },
  opts: { onDone: () => void; reducedMotion: boolean },
): { skip(): void } {
  const ctx = canvas.getContext('2d');
  if (!ctx) {
    opts.onDone();
    return { skip() {} };
  }
  let done = false;
  let raf = 0;
  const finish = () => {
    if (done) return;
    done = true;
    cancelAnimationFrame(raf);
    paint(ctx, img, TOTAL_MS);
    opts.onDone();
  };
  if (opts.reducedMotion) {
    finish();
    return { skip: finish };
  }
  const start = performance.now();
  const tick = (now: number) => {
    const t = now - start;
    paint(ctx, img, t);
    if (t < TOTAL_MS) {
      raf = requestAnimationFrame(tick);
    } else {
      finish();
    }
  };
  raf = requestAnimationFrame(tick);
  return { skip: finish };
}
