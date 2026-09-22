// ZAŠTO OVAKO (cigla M2/41 — stanje kartice s objašnjenjem kao Svelte 5 runa)
// `$state` izvan komponente treba nastavak `.svelte.ts` da ga kompajler prepozna kao rune-modul
// (isti obrazac kao `state.svelte.ts`/`i18n/index.svelte.ts`). Jedan objekt drži KOJI `id` je otvoren
// i NA ČEMU (`anchor`) da `ExplainCard.svelte` zna gdje se postaviti i kome vratiti fokus pri zatvaranju —
// bez toga bi svaki `Explainable` morao nositi svoju kopiju te odluke (S-010).
export const explain = $state({
  id: null as string | null,
  anchor: null as HTMLElement | null,
  extra: null as string | null,
});

export function openExplain(id: string, anchor: HTMLElement, extra?: string): void {
  explain.id = id;
  explain.anchor = anchor;
  explain.extra = extra ?? null;
}

// Pamti `anchor` PRIJE nego stanje nulira, jer `closeExplain` se zove i nakon promjene pogleda
// (dopuna T41, `$effect` u `ExplainCard.svelte`) — tada je `anchor` već izvan DOM-a, a `focus()` na
// odspojenom elementu je bezopasan no-op, ne baca.
export function closeExplain(): void {
  const anchor = explain.anchor;
  explain.id = null;
  explain.anchor = null;
  explain.extra = null;
  anchor?.focus();
}
