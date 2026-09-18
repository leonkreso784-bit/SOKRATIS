// ZAŠTO OVAKO (cigla M2/1 — ulaz glavnog prozora)
// Svelte 5 `mount` veže korijensku komponentu na `#app`; CSS se uvozi ovdje da Vite zna za njega.
import { mount } from 'svelte';
import App from './App.svelte';
import './app.css';

export default mount(App, { target: document.getElementById('app')! });
