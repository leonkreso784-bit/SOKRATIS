// ZAŠTO OVAKO (cigla M2/38 — čista odluka o pokretu, bez DOM-a)
// Jedna funkcija bez postraničnih učinaka: prima ono što `Settings` (korisnikov prekidač) i
// preglednik (`prefers-reduced-motion`) znaju, vraća `boolean` i ništa ne piše — zato je vitest
// testira izravno, bez lažnog DOM-a. `applyMotion`/`syncMotion` (`state.svelte.ts`) samo upisuju
// njezin rezultat u `document.documentElement.dataset.motion` (S-026).
export function motionOff(settingOn: boolean, prefersReduced: boolean): boolean {
  return !settingOn || prefersReduced;
}
