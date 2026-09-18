<script lang="ts">
  // ZAŠTO OVAKO (cigla M2/1 — kostur; M2/25 ga zamjenjuje okvirom)
  // `$state` je Svelte 5 runa: promjena vrijednosti sama osvježi ono što je čita.
  let name = $state('Sokratis');
</script>

<main class="p-4"><h1 class="text-xl">{name}</h1></main>
