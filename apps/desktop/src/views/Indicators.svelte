<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Pokazatelji: 18 kartica + sparkline trenda bez 18 poziva po iscrtavanju)
  // `$effect` dolazi u obzir baš ovdje jer treba nuspojavu (async poziv `api.getTrend`) koja se ne
  // smije ponoviti pri svakom iscrtavanju, nego SAMO kad se promijeni izvještaj/raspon — Svelte 5 to
  // prati preko čitanja `app.report`/`app.currentId`/`app.range` unutar efekta. `requestToken` čuva
  // od utrke: ako korisnik promijeni raspon dok stari poziv još čeka, kasni odgovor se baci umjesto
  // da pregazi noviji (Ruling orkestratora M2/27 #6).
  // Dopunjeno M2/41 — prekidač formule (`<details>`) je zamijenjen karticom s objašnjenjem: vrijednost
  // svake kartice je `Explainable`, a `Indicator.formula` (polje koje jezgra izračuna) ide u karticu
  // kao dodatni redak, ne kao vlastiti prikaz.
  import { app, setError } from '../lib/state.svelte';
  import { api } from '../lib/api';
  import { getLang, t } from '../lib/i18n/index.svelte';
  import { hours, num, percent } from '../lib/format';
  import Sparkline from '../lib/charts/Sparkline.svelte';
  import Explainable from '../lib/explain/Explainable.svelte';
  import type { Indicator, Range, TrendPoint } from '../lib/types';

  let trends = $state<Record<string, TrendPoint[]>>({});
  let requestToken = 0;

  $effect(() => {
    const report = app.report;
    const projectId = app.currentId;
    const range = app.range;
    if (!report || projectId === null) {
      trends = {};
      return;
    }
    const ids = report.indicators.map((ind) => ind.id);
    void loadTrends(projectId, range, ids);
  });

  async function loadTrends(projectId: number, range: Range, ids: string[]): Promise<void> {
    const token = ++requestToken;
    try {
      const results = await Promise.all(ids.map((id) => api.getTrend(projectId, id, range)));
      if (token !== requestToken) return; // stigao je noviji zahtjev u međuvremenu — ovaj odgovor je star
      const next: Record<string, TrendPoint[]> = {};
      ids.forEach((id, i) => {
        next[id] = results[i] ?? [];
      });
      trends = next;
    } catch (e) {
      if (token === requestToken) setError(e);
    }
  }

  // Jedini izbor OBLIKA prikaza — `Indicator.value` je onakav kakav jezgra izračuna, sučelje ga
  // NE preračunava (S-012). Redoslijed provjere prati brief M2/27: `*_share` → postotak, `hours` →
  // sati, `*_per_*`/`avg` → broj s jednom decimalom, inače cijeli broj.
  function indicatorValueText(ind: Indicator): string {
    if (ind.id.endsWith('_share')) return percent(ind.value, getLang());
    if (ind.id === 'hours') return hours(ind.value, getLang());
    if (ind.id.includes('_per_') || ind.id.includes('avg')) return num(ind.value, getLang(), 1);
    return num(ind.value, getLang(), 0);
  }
</script>

<div class="flex flex-col gap-4">
  <h1 class="text-xl font-semibold text-ink-0">{t('nav.indicators')}</h1>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {#each app.report.indicators as ind (ind.id)}
        <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
          <div class="flex items-start justify-between gap-2">
            <h2 class="text-sm font-semibold text-ink-0">{t('ind.' + ind.id)}</h2>
            <span class="shrink-0 text-xs text-ink-2">{t('ind.' + ind.kind)}</span>
          </div>
          <p class="text-2xl font-semibold text-ink-0">
            <Explainable id={`ind.${ind.id}`} extra={ind.formula}>{indicatorValueText(ind)}</Explainable>
          </p>
          <Sparkline
            points={(trends[ind.id] ?? []).map((p) => ({ value: p.value, marked: p.profile_changed }))}
            label={t('ind.trend') + ': ' + t('ind.' + ind.id)}
          />
        </div>
      {/each}
    </div>
  {/if}
</div>
