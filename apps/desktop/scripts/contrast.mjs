// ZAŠTO OVAKO (cigla M2/20 — biblioteka mjerenja boja)
// Izvor istine je tokens.css — skripta ga PARSIRA, ne drži kopiju vrijednosti (kopija bi ostarjela).
// Pragovi: WCAG AA tekst 4.5, UI 3.0; hue-odvojenost „ok" od marke 25° je naše pravilo (Sokrat Study),
// jer je zeleni signal na ekranu ono što se ne smije stopiti s gumbom.
export const TEXT_MIN = 4.5;
export const UI_MIN = 3.0;
export const HUE_MIN = 25;
const SURFACES = ['surface-0', 'surface-1', 'surface-2'];
const AS_TEXT = ['ink-0', 'ink-1', 'ink-2', 'brand-500', 'accent', 'warn-ink', 'danger-ink', 'ink-red', 'ink-amber', 'ink-green', 'ink-cyan', 'ink-blue', 'ink-indigo', 'ink-violet', 'ink-pink'];
const AS_UI = ['brand-400', 'brand-600', 'ok', 'warn', 'danger', 'line-strong'];

export function parseHex(v) {
  const m = String(v).trim().match(/^#([0-9a-f]{3}|[0-9a-f]{6})$/i);
  if (!m) return null;
  let h = m[1];
  if (h.length === 3) h = h[0] + h[0] + h[1] + h[1] + h[2] + h[2];
  return [0, 2, 4].map((i) => parseInt(h.slice(i, i + 2), 16));
}
const lin = (c) => { const s = c / 255; return s <= 0.03928 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4; };
export const luminance = ([r, g, b]) => 0.2126 * lin(r) + 0.7152 * lin(g) + 0.0722 * lin(b);
export function contrast(a, b) {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}
export function hue([r, g, b]) {
  const max = Math.max(r, g, b), min = Math.min(r, g, b), d = max - min;
  if (d === 0) return 0;
  let h = max === r ? ((g - b) / d) % 6 : max === g ? (b - r) / d + 2 : (r - g) / d + 4;
  h *= 60;
  return h < 0 ? h + 360 : h;
}
const hueDistance = (a, b) => { const d = Math.abs(a - b) % 360; return d > 180 ? 360 - d : d; };
export const stripComments = (css) => String(css).replace(/\/\*[\s\S]*?\*\//g, '');

/** Vrati { academic: {token: hex}, chalk: {...}, ... } — svaka tema = zadani blok pregažen svojim. */
export function themes(css) {
  const src = stripComments(css);
  const vars = (block) => {
    const out = {};
    for (const m of block.matchAll(/--color-([a-z0-9-]+)\s*:\s*([^;]+);/g)) out[m[1]] = m[2].trim();
    return out;
  };
  const base = vars(src.match(/@theme\s+static\s*\{([\s\S]*?)\n\}/)?.[1] ?? '');
  const out = {};
  for (const m of src.matchAll(/:root\[data-theme="([a-z0-9-]+)"\]\s*\{([\s\S]*?)\n\}/g)) out[m[1]] = { ...base, ...vars(m[2]) };
  if (!Object.keys(out).length) throw new Error('tokens.css: nijedan :root[data-theme] blok — brana bi mjerila nula tema');
  return out;
}

/** Nalazi: [{ theme, rule, token, on, value, min }] — prazno = sve prolazi. */
export function checkTokens(css) {
  const failures = [];
  for (const [theme, t] of Object.entries(themes(css))) {
    const px = (name) => parseHex(t[name]);
    for (const s of SURFACES) {
      const bg = px(s);
      if (!bg) { failures.push({ theme, rule: 'missing', token: s, on: '', value: 0, min: 0 }); continue; }
      for (const name of AS_TEXT) { const c = px(name); if (!c) continue; const v = contrast(c, bg); if (v < TEXT_MIN) failures.push({ theme, rule: 'text', token: name, on: s, value: +v.toFixed(2), min: TEXT_MIN }); }
      for (const name of AS_UI) { const c = px(name); if (!c) continue; const v = contrast(c, bg); if (v < UI_MIN) failures.push({ theme, rule: 'ui', token: name, on: s, value: +v.toFixed(2), min: UI_MIN }); }
    }
    const brand = px('brand-500'), onBrand = px('on-brand'), ok = px('ok');
    if (brand && onBrand) { const v = contrast(onBrand, brand); if (v < TEXT_MIN) failures.push({ theme, rule: 'on-brand', token: 'on-brand', on: 'brand-500', value: +v.toFixed(2), min: TEXT_MIN }); }
    if (brand && ok) { const d = hueDistance(hue(ok), hue(brand)); if (d < HUE_MIN) failures.push({ theme, rule: 'hue', token: 'ok', on: 'brand-500', value: +d.toFixed(1), min: HUE_MIN }); }
  }
  return failures;
}
