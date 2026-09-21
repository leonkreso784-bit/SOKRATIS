// ZAŠTO OVAKO (cigla M2/1 — kostur splash prozora; M2/24 ga puni animacijom)
// Svelte 5 `mount` veže `Splash` (canvas + `intro.ts`) na `#splash`, isti obrazac kao glavni
// prozor koji `mount`-a `App` na `#app`.
import { mount } from 'svelte';
import Splash from './Splash.svelte';

mount(Splash, { target: document.getElementById('splash')! });
