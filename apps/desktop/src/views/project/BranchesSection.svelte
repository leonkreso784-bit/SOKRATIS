<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/58 — sekcija Grane na ploči projekta, S-034): T58 crta SAMO tablicu iz
  // `Report.branches` (`BranchStats`, S-032) — jezgra je već izračunala commite/redke/sate po grani,
  // sučelje ih samo ispisuje (S-012). Graf (`HBars`) dolazi u T59; ova sekcija ne uvozi ništa iz
  // `lib/charts` da ne preduhitri tu ciglu. Kad je `Report.scope` „default" ili postoji samo jedna
  // grana, dodatna rečenica javlja da mjerenje NIJE obuhvatilo sve grane (spec §3.2) — bez nje bi
  // prazna/kratka tablica izgledala kao da projekt stvarno ima samo jednu granu.
  import { app } from '../../lib/state.svelte';
  import { getLang, t } from '../../lib/i18n/index.svelte';
  import { hours, num } from '../../lib/format';
  import { sectionId, sectionKey } from './sections';

  const branches = $derived(app.report?.branches ?? []);
  const defaultOnly = $derived(app.report ? app.report.scope === 'default' || app.report.branches.length <= 1 : false);
</script>

<div class="flex flex-col gap-4">
  <h2 id={sectionId('branches')} class="text-xl font-semibold text-ink-0">{t(sectionKey('branches'))}</h2>

  <table class="w-full text-left text-sm">
    <thead>
      <tr class="text-ink-2">
        <th scope="col" class="py-1 pr-3">{t('branches.name')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.commits')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.hours')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.lines')}</th>
        <th scope="col" class="py-1 pr-3">{t('branches.merged')}</th>
      </tr>
    </thead>
    <tbody>
      {#each branches as b (b.name)}
        <tr class="border-t border-line text-ink-1">
          <td class="py-1 pr-3">{b.name}</td>
          <td class="py-1 pr-3">{num(b.commits, getLang())}</td>
          <td class="py-1 pr-3">{hours(b.hours, getLang())}</td>
          <td class="py-1 pr-3">{num(b.lines, getLang())}</td>
          <td class="py-1 pr-3">{b.merged ? t('branches.yes') : t('branches.no')}</td>
        </tr>
      {/each}
    </tbody>
  </table>

  {#if defaultOnly}
    <p class="text-sm text-ink-2">{t('branches.default_only')}</p>
  {/if}
</div>
