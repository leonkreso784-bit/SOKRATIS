// ZAŠTO OVAKO (cigla M2/23 — grafovi bez biblioteke)
// Deset redaka matematike umjesto ovisnosti koju bi trebalo pinati i vezati na tokene (S-018).
// Sve su funkcije čiste; komponente samo crtaju ono što one izračunaju — pa je test ovdje, ne u
// DOM-u. Svaka funkcija mora ostati konačna (bez NaN/Infinity) i za prazan niz ili nulti raspon,
// jer SVG s NaN u atributu tiho ne crta ništa.
export const linear =
  ([d0, d1]: [number, number], [r0, r1]: [number, number]) =>
  (x: number): number =>
    d1 === d0 ? r0 : r0 + ((x - d0) / (d1 - d0)) * (r1 - r0);

export function niceMax(max: number): number {
  // `<=` ne hvata NaN (`NaN <= 0` je `false`), a `Infinity <= 0` je isto `false` — bez ovog
  // stražara NaN/Infinity procure kroz `linear`/`ticks` u SVG atribut i graf tiho nestane.
  if (!Number.isFinite(max) || max <= 0) return 1;
  const p = 10 ** Math.floor(Math.log10(max));
  const f = max / p;
  const nice = f <= 1 ? 1 : f <= 1.5 ? 1.5 : f <= 2 ? 2 : f <= 3 ? 3 : f <= 5 ? 5 : f <= 8 ? 8 : 10;
  return nice * p;
}

export function ticks(max: number, count = 4): number[] {
  const top = niceMax(max);
  return Array.from({ length: count + 1 }, (_, i) => (top / count) * i);
}

export const cumulative = (v: number[]): number[] =>
  v.reduce<number[]>((acc, x) => [...acc, (acc.at(-1) ?? 0) + x], []);

// Domenu grafa treba računati SAMO nad konačnim vrijednostima — jedan `NaN` u nizu ne smije
// srušiti maksimum cijelog grafa na `NaN` (koristi ga svaka komponenta umjesto `Math.max(...)`).
export const finiteMax = (values: number[]): number => Math.max(0, ...values.filter(Number.isFinite));

export const linePath = (pts: { x: number; y: number }[]): string =>
  pts
    .filter((p) => Number.isFinite(p.x) && Number.isFinite(p.y))
    .map((p, i) => `${i ? 'L' : 'M'}${+p.x.toFixed(2)} ${+p.y.toFixed(2)}`)
    .join(' ');

export function arcPath(cx: number, cy: number, r: number, a0: number, a1: number): string {
  // Pun krug (a1 - a0 = 2π) ima istu početnu i završnu točku pa se SVG-luk ne bi nacrtao;
  // makni završetak za epsilon da luk ostane vidljiv, a ostali kutovi prolaze nepromijenjeni.
  const EPS = 1e-3;
  const end = a1 - a0 >= Math.PI * 2 - EPS ? a0 + Math.PI * 2 - EPS : a1;
  const pt = (a: number): [number, number] => [
    +(cx + r * Math.sin(a)).toFixed(2),
    +(cy - r * Math.cos(a)).toFixed(2),
  ];
  const [x0, y0] = pt(a0);
  const [x1, y1] = pt(end);
  const large = end - a0 > Math.PI ? 1 : 0;
  return `M${x0} ${y0} A${r} ${r} 0 ${large} 1 ${x1} ${y1}`;
}

export function ringSegments(values: number[]): { start: number; end: number; share: number }[] {
  const total = values.reduce((a, b) => a + b, 0);
  // `total > 0` (ne `total <= 0`) hvata i NaN: jedan NaN u `values` čini zbroj NaN-om, a
  // `NaN <= 0` je `false` pa bi stara provjera propustila NaN dalje u dijeljenje (isti uzrok kao u `niceMax`).
  if (!(total > 0)) return [];
  let a = 0;
  return values.map((v) => {
    const share = v / total;
    const seg = { start: a, end: a + share * Math.PI * 2, share };
    a = seg.end;
    return seg;
  });
}

// Graf S imenom je slika (`role="img"` + `aria-label`), graf BEZ imena je ukras (`aria-hidden`) —
// čitač ekrana inače najavi „slika" bez ičega kad `label` nedostaje (recenzija M2/23, krug 1).
export function svgA11y(name: string | undefined): { role: 'img'; 'aria-label': string } | { 'aria-hidden': 'true' } {
  return name && name.trim() !== '' ? { role: 'img', 'aria-label': name } : { 'aria-hidden': 'true' };
}
