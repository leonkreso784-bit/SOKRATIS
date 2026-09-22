// ZAŠTO OVAKO (cigla M2/38 — tip za read-motion.mjs)
// Isti razlog kao `read-tokens.d.mts` (M2/20): `.mjs` bez `allowJs` u tsconfigu (izvan vlasništva
// ove cigle) ne nosi tipove — svelte-check bez ovoga baca TS7016 na uvoz iz `read-motion.mjs`.
export const motionCss: string;
