<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/58 — kartica Sažetak na ploči projekta, S-034): šest kartica sa
  // POSTOJEĆIM `<Explainable>` id-jevima (`ind.commits`/`ind.hours`/`ind.working_days`/
  // `ind.deliveries`/`docs.score`/`signals.severity`) — ista objašnjenja koja Pokazatelji i
  // Dokumentacija već koriste, ne nova kopija teksta (S-010). Vrijednosti su onakve kakve jezgra
  // vrati (S-012): zbroj sati po danu i broj signala po težini su jedini izračun ovdje, oba čista
  // funkcija (`Array.reduce`, `signalCounts` iz `views/helpers.ts`), ne novo mjerenje.
  import { app } from '../../lib/state.svelte';
  import { getDict, getLang, t } from '../../lib/i18n/index.svelte';
  import { hours, num } from '../../lib/format';
  import { signalCounts, signalSummary } from '../helpers';
  import Explainable from '../../lib/explain/Explainable.svelte';
  import { sectionId, sectionKey } from './sections';

  const touchedCommits = $derived(app.report?.touched.commits ?? null);
  const totalHours = $derived(app.report ? app.report.days.reduce((sum, d) => sum + d.hours, 0) : null);
  const workingDays = $derived(app.report?.days.length ?? null);
  const deliveriesCount = $derived(app.report?.deliveries.length ?? null);
  const docsScore = $derived(app.report?.docs ? app.report.docs.score : null);
  const signals = $derived(signalCounts(app.report?.signals ?? []));
</script>

<div class="flex flex-col gap-4">
  <h2 id={sectionId('summary')} class="text-xl font-semibold text-ink-0">{t(sectionKey('summary'))}</h2>

  <div class="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-6">
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.commits')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="ind.commits">{num(touchedCommits, getLang())}</Explainable>
      </p>
    </div>
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.hours')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="ind.hours">{hours(totalHours, getLang())}</Explainable>
      </p>
    </div>
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.days')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="ind.working_days">{num(workingDays, getLang())}</Explainable>
      </p>
    </div>
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.deliveries')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="ind.deliveries">{num(deliveriesCount, getLang())}</Explainable>
      </p>
    </div>
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.docs')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="docs.score">{docsScore !== null ? num(docsScore, getLang()) : t('common.na')}</Explainable>
      </p>
    </div>
    <div class="flex flex-col gap-2 rounded-lg border border-line bg-surface-1 p-4 shadow-e1">
      <span class="text-xs text-ink-2">{t('summary.signals')}</span>
      <p class="text-2xl font-semibold text-ink-0">
        <Explainable id="signals.severity">{signalSummary(signals, getDict())}</Explainable>
      </p>
    </div>
  </div>
</div>
