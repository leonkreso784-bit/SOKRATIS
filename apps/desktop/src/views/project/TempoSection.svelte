<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/27 — Tempo: dani u razdoblju, commiti/sati, kumulativa)
  // Prekidač commiti/sati je dva `<button aria-pressed>` (natpis iz rječnika, S-021) koji samo BIRA
  // koji stupac ide u `Bars` — ništa se ovdje ne zbraja ni ne dijeli. Kumulativna linija crta
  // `d.commits_cumulative` izravno: to polje jezgra već izračuna (S-012), sučelje ga ne računa samo.
  // Dopunjeno M2/42 — oba grafa dobivaju `<Explainable block>` (S-027), a zaglavlje stupca sati u
  // tablici svoj `<Explainable>` (jedini stupac tablice s vlastitim mjerenjem koje pokazatelj ne
  // ponavlja negdje drugdje — ostala zaglavlja nemaju id pa se ne omataju, dopuna T42).
  // Dopunjeno M2/55 (S-035) — `Bars`/`Line` sad primaju `buckets`/`series` (temelji iz T54): stupci
  // i linija dobivaju stvarne datume na X-osi umjesto redoslijeda; prekidač dan/tjedan/mjesec dolazi
  // tek u T59 (PLOČA), ovdje je zrnatost fiksno `'day'` kao i dosad.
  // Preseljeno M2/58 iz views/Tempo.svelte — sadržaj nepromijenjen, samo <h1> → <h2 id> i putanje
  // uvoza (S-034).
  // Dopunjeno M2/59 — prekidač dan/tjedan/mjesec (`granularity`, S-035 nastavak T53): stupci i tablica
  // dijele iste dane, ali stupci se sad preslažu (`bucketDays`) po odabranoj zrnatosti dok tablica
  // OSTAJE po danu (točan popis, S-012). Toplinska karta (`Heatmap`) i doba dana (`Histogram`) su nova
  // dva grafa iz spec-a §3.2 — oba samo CRTAJU već izmjerene `DayStats`/`CommitRow` (jezgra ne zna za
  // "toplinsku kartu", to je samo drugi raspored istih brojeva).
  import { app } from '../../lib/state.svelte';
  import { getLang, t } from '../../lib/i18n/index.svelte';
  import { hours, num, ymd } from '../../lib/format';
  import { bucketDays, hourHistogram, type Granularity } from '../../lib/charts/bucket';
  import Bars from '../../lib/charts/Bars.svelte';
  import Line from '../../lib/charts/Line.svelte';
  import Heatmap from '../../lib/charts/Heatmap.svelte';
  import Histogram from '../../lib/charts/Histogram.svelte';
  import Explainable from '../../lib/explain/Explainable.svelte';
  import { sectionId, sectionKey } from './sections';

  let mode = $state<'commits' | 'hours'>('commits');
  let granularity = $state<Granularity>('day');
  const GRANULARITIES: readonly Granularity[] = ['day', 'week', 'month'];

  const days = $derived(app.report?.days ?? []);
  const buckets = $derived(
    bucketDays(days, granularity).map((b) => ({ start: b.start, values: { [mode]: mode === 'commits' ? b.commits : b.hours } })),
  );
  const valueText = (v: number): string => (mode === 'commits' ? num(v, getLang()) : hours(v, getLang()));
  // Ruling R72 (popravak 1) — ispravlja pogrešnu tvrdnju iz teksta plana: jezgra (`Report.until`,
  // `report.rs`) NE upisuje današnji datum kad je raspon „sve", ostaje `null`. `today` je pomoćna
  // vrijednost SUČELJA za gornju granicu toplinske karte u tom slučaju (prikaz, ne mjerenje).
  const today = new Date().toISOString().slice(0, 10);
</script>

<div class="flex flex-col gap-4">
  <h2 id={sectionId('tempo')} class="text-xl font-semibold text-ink-0">{t(sectionKey('tempo'))}</h2>

  {#if !app.report}
    <p class="text-sm text-ink-2">{t('common.loading')}</p>
  {:else if days.length === 0}
    <p class="text-sm text-ink-2">{t('tempo.empty')}</p>
  {:else}
    <div class="flex flex-wrap gap-4">
      <div class="flex gap-1" role="group" aria-label={t('nav.tempo')}>
        <button
          type="button"
          class="rounded-md px-3 py-1.5 text-sm {mode === 'commits' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
          aria-pressed={mode === 'commits'}
          onclick={() => (mode = 'commits')}
        >
          {t('tempo.commits')}
        </button>
        <button
          type="button"
          class="rounded-md px-3 py-1.5 text-sm {mode === 'hours' ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
          aria-pressed={mode === 'hours'}
          onclick={() => (mode = 'hours')}
        >
          {t('tempo.hours')}
        </button>
      </div>

      <div class="flex gap-1" role="group" aria-label={t('tempo.granularity')}>
        {#each GRANULARITIES as g (g)}
          <button
            type="button"
            class="rounded-md px-3 py-1.5 text-sm {granularity === g ? 'bg-brand-500 text-on-brand' : 'text-ink-1 hover:bg-surface-2'}"
            aria-pressed={granularity === g}
            onclick={() => (granularity = g)}
          >
            {t(`tempo.granularity.${g}`)}
          </button>
        {/each}
      </div>
    </div>

    <!-- Popravak 1 (V1, Ruling R70): svaki graf dobiva VIDLJIV naslov iznad sebe, isti izraz koji graf
         već prima kao `label` (bez novog ključa rječnika) — dosad je natpis bio SAMO `aria-label`,
         nevidljiv na ekranu, pa su četiri grafa zaredom izgledala bez ijednog naslova. Naslov je IZVAN
         `<Explainable>` da klik na naslov ne otvara karticu s objašnjenjem. -->
    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t(`tempo.${mode}`)}</h3>
      <Explainable id="tempo.bars" block>
        <Bars
          {buckets}
          series={[{ id: mode, label: t(`tempo.${mode}`), color: 'var(--color-brand-500)' }]}
          {granularity}
          label={t(`tempo.${mode}`)}
        />
      </Explainable>
    </div>
    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t('tempo.cumulative')}</h3>
      <Explainable id="tempo.cumulative" block>
        <Line
          series={[
            {
              id: 'cum',
              label: t('tempo.cumulative'),
              color: 'var(--color-brand-500)',
              points: days.map((d) => ({ date: d.date, value: d.commits_cumulative })),
            },
          ]}
          label={t('tempo.cumulative')}
        />
      </Explainable>
    </div>

    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t('tempo.heatmap')}</h3>
      <Explainable id="tempo.heatmap" block>
        <Heatmap
          days={days.map((d) => ({ date: d.date, value: mode === 'commits' ? d.commits : d.hours }))}
          range={[app.report.since, app.report.until ?? today]}
          label={t('tempo.heatmap')}
          {valueText}
        />
      </Explainable>
    </div>
    <div class="flex flex-col gap-2">
      <h3 class="text-sm font-semibold text-ink-1">{t('tempo.hoursOfDay')}</h3>
      <Explainable id="tempo.hours_of_day" block>
        <Histogram counts={hourHistogram(app.report.commits)} label={t('tempo.hoursOfDay')} />
      </Explainable>
    </div>

    <table class="w-full text-left text-sm">
      <thead>
        <tr class="text-ink-2">
          <th scope="col" class="py-1 pr-3">{t('tempo.day')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.commits')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.cumulative')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.lines')}</th>
          <th scope="col" class="py-1 pr-3"><Explainable id="tempo.hours">{t('tempo.hours')}</Explainable></th>
          <th scope="col" class="py-1 pr-3">{t('tempo.deliveries')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.deploys')}</th>
          <th scope="col" class="py-1 pr-3">{t('tempo.testLines')}</th>
        </tr>
      </thead>
      <tbody>
        {#each days as d (d.date)}
          <tr class="border-t border-line text-ink-1">
            <td class="py-1 pr-3">{ymd(d.date, getLang())}</td>
            <td class="py-1 pr-3">{num(d.commits, getLang())}</td>
            <td class="py-1 pr-3">{num(d.commits_cumulative, getLang())}</td>
            <td class="py-1 pr-3">{num(d.lines, getLang())}</td>
            <td class="py-1 pr-3">{hours(d.hours, getLang())}</td>
            <td class="py-1 pr-3">{num(d.deliveries, getLang())}</td>
            <td class="py-1 pr-3">{num(d.deploys, getLang())}</td>
            <td class="py-1 pr-3">{num(d.test_lines, getLang())}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>
