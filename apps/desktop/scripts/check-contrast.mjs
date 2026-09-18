// ZAŠTO OVAKO (cigla M2/20 — brana kontrasta u `npm run check`)
// Tanka izvršna ljuska oko `checkTokens`: čita pravu datoteku tokena, ispisuje nalaze čitljivo
// (tema · pravilo · token · ploha · izmjereno < prag) i vraća izlazni kod 1 kad ih ima, jer
// `npm run check` mora crveno stati čim boja padne ispod WCAG-a ili se stopi s markom (S-017).
import { checkTokens } from './contrast.mjs';
import { tokensCss } from './read-tokens.mjs';

const f = checkTokens(tokensCss);
if (f.length) {
  console.error('check:contrast — ' + f.length + ' nalaza');
  for (const x of f) console.error(`  ${x.theme.padEnd(9)} ${x.rule.padEnd(8)} ${x.token.padEnd(12)} na ${x.on.padEnd(10)} ${x.value} < ${x.min}`);
  process.exit(1);
}
console.log('check:contrast — sve četiri teme prolaze');
