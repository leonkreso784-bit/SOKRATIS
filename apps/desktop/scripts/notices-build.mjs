// ZAŠTO OVAKO (cigla 1.0.1/T67 — obavijesti o licencama, I/O ljuska)
// Sav disk i procesi su ovdje; tekst slaže čista `renderNotices` iz `notices.mjs`. Procesi se zovu
// kroz `execFileSync` (argumenti kao niz, bez ljuske); cargo s `--locked --offline`, jer brana ne
// smije pomaknuti Cargo.lock ni otići na mrežu. Koji crateovi ULAZE u izvršnu datoteku kaže `cargo
// tree -e normal` (razrješava značajke kao pravi build), a licencu, autore i mapu paketa `cargo
// metadata` — sam graf iz `cargo metadata` uzima uniju značajki cijelog workspacea i broji crateove
// koji se ne prevode. Koji npm paketi ulaze u sučelje kaže build: Vite se pokreće u privremenu mapu
// (ne `dist/`) sa sourcemapama, a imena paketa daju `sources` tih mapa i licencni banneri u CSS-u.
// Build je bez minifikacije jer minifikator briše bannere; koje module uključuje odlučuje
// tree-shaking, isti s minifikacijom i bez nje. `--check` ne piše ništa: izlazni kod 1 kad commitana
// datoteka nije ono što bi skripta napisala.
import { execFileSync } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  isLicenseFile,
  packagesFromBanners,
  packagesFromSources,
  parseCargoTree,
  personNames,
  renderNotices,
} from './notices.mjs';

const TARGET = 'x86_64-pc-windows-msvc';
const desktop = fileURLToPath(new URL('..', import.meta.url));
const root = fileURLToPath(new URL('../../..', import.meta.url));
const outFile = join(root, 'THIRD-PARTY-NOTICES.txt');
const check = process.argv.includes('--check');
const NPM_CI = 'prvo `npm ci`';

const cargo = (args) =>
  execFileSync('cargo', [...args, '--locked', '--offline'], { cwd: root, encoding: 'utf8', maxBuffer: 256 * 1024 * 1024 });

/** Datoteke licence u mapi paketa (samo datoteke, ne mape), sortirano po imenu. */
function licenseFiles(dir, extra) {
  const names = readdirSync(dir, { withFileTypes: true })
    .filter((e) => e.isFile() && isLicenseFile(e.name))
    .map((e) => e.name);
  if (extra && !names.includes(extra) && existsSync(join(dir, extra))) names.push(extra);
  return names.sort().map((name) => ({ name, text: readFileSync(join(dir, name), 'utf8') }));
}

function crateComponents() {
  const meta = JSON.parse(cargo(['metadata', '--format-version', '1', '--filter-platform', TARGET]));
  const byKey = new Map();
  for (const p of meta.packages) {
    const key = `${p.name}@${p.version}`;
    if (byKey.has(key)) throw new Error(`notices: ${key} postoji iz dva izvora — ne znam čiju licencu uzeti`);
    byKey.set(key, p);
  }
  const used = parseCargoTree(cargo(['tree', '-p', 'sokratis-desktop', '-e', 'normal', '--target', TARGET, '--prefix', 'none']));
  return used.map((key) => {
    const p = byKey.get(key);
    if (!p) throw new Error(`notices: ${key} je u cargo tree, a nema ga u cargo metadata`);
    return {
      kind: 'crate',
      name: p.name,
      version: p.version,
      license: p.license ?? '',
      files: licenseFiles(dirname(p.manifest_path), p.license_file ?? undefined),
      holders: personNames(p.authors),
    };
  });
}

/** Sve datoteke ispod `dir` (rekurzivno), sortirano — izlaz builda ima podmapu `assets/`. */
function walk(dir) {
  return readdirSync(dir, { withFileTypes: true, recursive: true })
    .filter((e) => e.isFile())
    .map((e) => join(e.parentPath, e.name))
    .sort();
}

/** Paketi `{ name, path }` čiji kod ulazi u izgrađeno sučelje, izmjereno buildom u privremenu mapu. */
function bundledPackages() {
  const vite = join(desktop, 'node_modules', 'vite', 'bin', 'vite.js');
  if (!existsSync(vite)) throw new Error(`notices: nema node_modules/vite — ${NPM_CI}`);
  const out = mkdtempSync(join(tmpdir(), 'sokratis-notices-'));
  try {
    const args = ['build', '--sourcemap', '--minify', 'false', '--outDir', out, '--emptyOutDir', '--logLevel', 'error'];
    execFileSync(process.execPath, [vite, ...args], { cwd: desktop, stdio: ['ignore', 'ignore', 'inherit'] });
    const byPath = new Map();
    for (const file of walk(out)) {
      let found = [];
      if (file.endsWith('.map')) found = packagesFromSources(JSON.parse(readFileSync(file, 'utf8')).sources ?? []);
      else if (/\.(css|js)$/.test(file)) found = packagesFromBanners(readFileSync(file, 'utf8'));
      for (const p of found) byPath.set(p.path, p);
    }
    if (!byPath.size) throw new Error('notices: build sučelja nije imenovao nijedan npm paket — sourcemape nisu pročitane?');
    return [...byPath.values()];
  } finally {
    rmSync(out, { recursive: true, force: true });
  }
}

function npmComponents() {
  const lock = JSON.parse(readFileSync(join(desktop, 'package-lock.json'), 'utf8'));
  return bundledPackages().map(({ name, path }) => {
    const version = lock.packages?.[path]?.version;
    if (!version) throw new Error(`notices: ${path} je u izgrađenom sučelju, a package-lock.json ga ne zna`);
    const dir = join(desktop, path);
    if (!existsSync(join(dir, 'package.json'))) throw new Error(`notices: nema ${path} — ${NPM_CI}`);
    const pkg = JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8'));
    if (pkg.version !== version) throw new Error(`notices: ${path} je ${pkg.version}, lock kaže ${version} — ${NPM_CI}`);
    const holders = [...new Set([...personNames(pkg.author), ...personNames(pkg.contributors)])];
    return { kind: 'npm', name, version, license: pkg.license ?? '', files: licenseFiles(dir), holders };
  });
}

try {
  const text = renderNotices([...crateComponents(), ...npmComponents()]);
  if (check) {
    const current = existsSync(outFile) ? readFileSync(outFile, 'utf8') : '';
    if (current !== text) {
      console.error('notices:check — THIRD-PARTY-NOTICES.txt nije ono što bi skripta napisala; pokreni `npm run notices`');
      process.exit(1);
    }
    console.log('notices:check — THIRD-PARTY-NOTICES.txt odgovara lockovima i buildu sučelja');
  } else {
    writeFileSync(outFile, text, 'utf8');
    console.log(`notices — zapisano ${outFile} (${Buffer.byteLength(text)} bajtova)`);
  }
} catch (e) {
  console.error(e instanceof Error ? e.message : String(e));
  process.exit(1);
}
