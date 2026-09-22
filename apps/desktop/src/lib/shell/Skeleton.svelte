<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/40 — kostur pri prvom učitavanju projekta, spec §13.3)
  // Tri statična bloka umjesto crtanja stvarnih pogleda: prava visina naslova/grafa/tablice gasi
  // "skok" kad izvještaj stigne. `aria-busy`/`aria-label` javljaju čitaču ekrana da se nešto učitava
  // umjesto da mu tri prazna div-a prođu bez ikakvog značenja.
  import { t } from '../i18n/index.svelte';
</script>

<div class="flex flex-col gap-3" aria-busy="true" aria-label={t('common.loading')}>
  <div class="skeleton" style="height: 24px"></div>
  <div class="skeleton" style="height: 160px"></div>
  <div class="skeleton" style="height: 96px"></div>
</div>
