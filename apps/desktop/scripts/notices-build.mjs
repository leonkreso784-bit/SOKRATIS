// ZAŠTO OVAKO (cigla 1.0.1/T67 — obavijesti o licencama, I/O ljuska)
// Sav disk i procesi su ovdje; tekst slaže čista `renderNotices` iz `notices.mjs`. Cargo se zove
// kroz `execFileSync` (argumenti kao niz, bez ljuske) s `--locked --offline`: brana ne smije
// pomaknuti Cargo.lock ni otići na mrežu. Koji crateovi ULAZE u izvršnu datoteku kaže `cargo tree
// -e normal` (razrješava značajke kao pravi build), a licencu i mapu paketa `cargo metadata` —
// sam graf iz `cargo metadata` uzima uniju značajki cijelog workspacea i broji crateove koji se ne
// prevode. `--check` ne piše ništa: izlazni kod 1 kad commitana datoteka nije ono što bi skripta napisala.
import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { isLicenseFile, npmPackages, parseCargoTree, renderNotices } from './notices.mjs';

const TARGET = 'x86_64-pc-windows-msvc';
const desktop = fileURLToPath(new URL('..', import.meta.url));
const root = fileURLToPath(new URL('../../..', import.meta.url));
const outFile = join(root, 'THIRD-PARTY-NOTICES.txt');
const check = process.argv.includes('--check');

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
    };
  });
}

function npmComponents() {
  const lock = JSON.parse(readFileSync(join(desktop, 'package-lock.json'), 'utf8'));
  return npmPackages(lock).map(({ name, version, path }) => {
    const dir = join(desktop, path);
    if (!existsSync(join(dir, 'package.json'))) throw new Error(`notices: nema ${path} — prvo \`npm ci\``);
    const pkg = JSON.parse(readFileSync(join(dir, 'package.json'), 'utf8'));
    if (pkg.version !== version) throw new Error(`notices: ${path} je ${pkg.version}, lock kaže ${version} — prvo \`npm ci\``);
    return { kind: 'npm', name, version, license: pkg.license ?? '', files: licenseFiles(dir) };
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
    console.log('notices:check — THIRD-PARTY-NOTICES.txt odgovara lockovima');
  } else {
    writeFileSync(outFile, text, 'utf8');
    console.log(`notices — zapisano ${outFile} (${Buffer.byteLength(text)} bajtova)`);
  }
} catch (e) {
  console.error(e instanceof Error ? e.message : String(e));
  process.exit(1);
}
