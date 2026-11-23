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
// (cigla mac/T76) `cargo tree --target` ovisnosti proc-makroa rješava za DOMAĆINA, pa na Macu donese
// i crateove samo za macOS (`swift-rs` → `base64`). Zato se njegov popis presijeca s paketima iz
// `cargo metadata` za cilj (`targetCrates`). Na Macu je popis danas isti kao na Windowsu (izmjereno
// 2026-11-23), ali proc-makro ovisnost samo za Windows domaćina na Macu bi izostala — mjerodavan je
// `notices:check` na Windowsu. Preskočeni crateovi idu jednim retkom na stderr, da novi bude vidljiv.
// (cigla mac/T87 — popis po platformi, S-043 O6) Svaki paket nosi popis onoga što je u njemu: Windows u
// THIRD-PARTY-NOTICES.txt, macOS (aarch64-apple-darwin) u THIRD-PARTY-NOTICES-macos.txt. Windows popis
// se mjeri na svakom domaćinu (na Macu izmjeren isti), Mac popis SAMO na Macu: drugdje bi `cargo tree`
// propustio Mac-only ovisnosti proc-makroa i tiho napisao drukčiji popis. Zato se tamo preskače jednim
// retkom na stderr i izlaznim kodom 0 — brana ne pada, ali i ne tvrdi ono što nije izmjerila.
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
  planTargets,
  renderNotices,
  targetCrates,
} from './notices.mjs';

const desktop = fileURLToPath(new URL('..', import.meta.url));
const root = fileURLToPath(new URL('../../..', import.meta.url));
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

/** Crateovi prevedeni u izvršnu datoteku za cilj `triple` (Rust target triple). */
function crateComponents(triple) {
  const meta = JSON.parse(cargo(['metadata', '--format-version', '1', '--filter-platform', triple]));
  const byKey = new Map();
  for (const p of meta.packages) {
    const key = `${p.name}@${p.version}`;
    if (byKey.has(key)) throw new Error(`notices: ${key} postoji iz dva izvora — ne znam čiju licencu uzeti`);
    byKey.set(key, p);
  }
  const tree = parseCargoTree(cargo(['tree', '-p', 'sokratis-desktop', '-e', 'normal', '--target', triple, '--prefix', 'none']));
  const used = targetCrates(tree, byKey.keys());
  const skipped = tree.filter((key) => !used.includes(key));
  if (skipped.length) console.error(`notices: ${triple}: preskočeno kao ovisnost domaćina: ${skipped.join(', ')}`);
  return used.map((key) => {
    const p = byKey.get(key);
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
  const plan = planTargets(process.platform);
  for (const t of plan.skip) console.error(`notices: preskočeno: popis za ${t.name} (${t.file}) se mjeri samo na ${t.name} domaćinu`);
  // Sučelje je isto na svakom cilju (Vite ne zna za platformu) — gradi se jednom za sve popise.
  const npm = npmComponents();
  let stale = false;
  for (const t of plan.measure) {
    const outFile = join(root, t.file);
    const text = renderNotices([...crateComponents(t.triple), ...npm], t.triple);
    if (check) {
      const current = existsSync(outFile) ? readFileSync(outFile, 'utf8') : '';
      if (current !== text) {
        console.error(`notices:check — ${t.file} nije ono što bi skripta napisala; pokreni \`npm run notices\``);
        stale = true;
      } else console.log(`notices:check — ${t.file} odgovara lockovima i buildu sučelja`);
    } else {
      writeFileSync(outFile, text, 'utf8');
      console.log(`notices — zapisano ${outFile} (${Buffer.byteLength(text)} bajtova)`);
    }
  }
  if (stale) process.exit(1);
} catch (e) {
  console.error(e instanceof Error ? e.message : String(e));
  process.exit(1);
}
