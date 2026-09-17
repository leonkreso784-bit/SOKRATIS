# ARHITEKTURA + MILESTONE 1 — dizajn Sokratisa

**Status:** 🟩 AKTIVAN · **Otvoren:** 2026-09-17 · Odobren, plan izveden (T1–T21 u `main`); **čeka:**
T22 (§7 izlazni uvjet) pa seli u `archive/` — status i brojke: [ROADMAP.md](./ROADMAP.md),
[../records/CHANGELOG.md](../records/CHANGELOG.md)

> **Što ovaj dokument JEST:** arhitektura cijelog sustava (da odluke imaju jedno mjesto) i
> **precizan opseg Milestonea 1**, iz kojeg se piše prvi plan implementacije
> (`docs/superpowers/plans/`). Milestone 2 i 3 su ovdje samo ocrtani; svaki dobiva svoj spec kad
> dođe na red, a ovaj tada seli u `archive/`.
>
> **Što NIJE:** nije dom za odluke (te su u [DECISIONS.md](../records/DECISIONS.md), S-001…S-010)
> ni za definiciju proizvoda ([PRD.md](../product/PRD.md)). Ovdje su pointeri, ne kopije.
>
> **Odluke Leona 2026-09-17:** Rust · Tauri 2 · Svelte 5 s tokenima Sokrat Studyja · ručni podaci u
> `.sokratis/` u repou · prvi signali **nespojene grane** i **kašnjenje docs-a** · novi znak.

---

## 1 · Slika cjeline

```
sokratis/
  Cargo.toml                 # workspace
  crates/
    sokratis-core/           # ČISTI Rust, bez I/O-a: model · parseri · metrike · docs-ocjena · pravila
    sokratis-io/             # git (proces) · datoteke · profil · ručni podaci · [M2] SQLite · watcher
    sokratis-cli/            # binarna `sokratis`: report · docs · signals   (i preflight brana)
  apps/
    desktop/                 # [M2] Tauri 2 (src-tauri = tanke naredbe nad io/core) + Svelte 5
```

**Tok podataka (isti za CLI i desktop):**

```
putanja repoa
  → io: profil (.sokratis/profile.json ili zadani) · git log (tekst) · dnevnik i plan (tekst) · ručni podaci
  → core: parse → Model → metrike → docs-ocjena → pravila → Report (serde)
  → cli: JSON / tablica / izlazni kod        ili        desktop: Tauri naredba → UI · SQLite snimka
```

**Granica koja se ne prelazi (S-002):** `sokratis-core` ne otvara nijednu datoteku i ne poziva
nijedan proces. Prima `&str` i strukture, vraća strukture. Zato se testira bez gita i zato ga CLI i
Tauri koriste bez prilagodbe. *Zašto Rust ovako:* to je i najčišći teren za učenje ownershipa —
core radi s podacima koje posjeduje, bez lifetimeova prema vanjskom svijetu.

### 1.1 Identitet projekta, radna stabla, grane

- Projekt = zajednički git-direktorij. `git worktree list --porcelain` daje sva stabla; Sokrat Study
  je **jedan** projekt s pet stabala.
- **Metrike se računaju nad zadanom granom** (zadano `main`) — to je paritet s tablicom, koja je
  čitala log grane u kojoj se vrtjela.
- **Grane izvan zadane ulaze samo u signale** (`unmerged-branches`). Profil može uključiti i njih u
  metrike (`include_unmerged: true`), zadano ne.

---

## 2 · Jezgra (`sokratis-core`)

### 2.1 Model

```rust
struct Commit { sha, author_time: i64, commit_time: i64, date: String /* YYYY-MM-DD autora */,
                commit_date: String /* YYYY-MM-DD commita */, subject: String,
                files: Vec<FileChange /* path, added, deleted */> }
enum WorkKind { Planning, Documentation, Execution, Polish, Debugging }   // Leonovih pet
enum SubKind  { Brick, GateOrMeasure, Deploy, Other }
struct Delivery { date, model, title, kind: WorkKind, deploy: bool }      // iz dnevnika
struct Phase { id, name, state: Planned | Running | Closed, total_bricks, done_bricks, from, to }
struct Vision { title, source, state, percent: Option<u8>, note }         // ručno
struct Touched { commits, lines, files, skipped_lines }
struct Report { generated_at, since, branch, touched: Touched, days: Vec<DayStats>,
                kinds: Vec<KindStats>, indicators: Vec<Indicator>, phases: Vec<Phase>,
                visions: Vec<Vision>, docs: Option<DocsHealth>, signals: Vec<Signal> }
```

Identifikatori u kodu i JSON-u su **engleski i stabilni** (S-008): `work_kind = "debugging"`.
Hrvatske i engleske natpise daje sučelje. Jezgra je time objavljiva, sučelje dvojezično od početka.

### 2.2 Ulazni formati — sve je tekst; regexi žive u profilu, ovdje su ZADANE vrijednosti

| ulaz | zadani oblik | izvor u Sokrat Studyju |
|---|---|---|
| git log | `@@%h\|%at\|%ct\|%ad\|%cd\|%s` + `--numstat` uz `--date=format:%Y-%m-%d` — unix-vremena za razmake, lokalni datumi za dan (`%ad` = autor, kao tablica) i za `since` (`%cd` = commit, kao git) | `git` |
| `--since` prema `git log` | `io` šalje `<datum> 00:00:00`, ne goli datum — bez sata `git log --since` uzima trenutno doba dana (approxidate), pa isti dan daje različit broj commita ovisno kad se pokrene (S-011) | mjereno na `main`-u Sokrat Studyja |
| dnevnik | `^## (\d{4}-\d{2}-\d{2})(?:\s*\(([^)]*)\))?\s*[—-]+\s*(.+)$` | `docs/records/PROGRESS.md` |
| plan | cigla `^\| \*\*(F\d)/(\d+)\*\*\s*(✅?)` · faza `^### (F\d) · (.+)$` | `docs/plan/RASPORED.md` |
| oznaka faze u commitu | `^(F\d/\d\|C\d[ab]?(?:/\d\w*)?\|MREZA[- ]?[A-E]\d?\|R\d\|T\d\|ALAT-\d\|BUG-\d+\|U\d)` | opis commita |
| testni redci | `tests/**` · `**/check-*` · `*.test.js` · `*.spec.js` | putanje u numstatu |
| klasifikator vrste | uređena lista `(vrsta, regex)`; **redoslijed je bitan:** planiranje > dokumentacija > debugging > poliranje > izvođenje | regexi iz `rad-xlsx.py`, preneseni 1:1 |
| podvrsta | deploy (`🚀`, „na produkciji", „deploy") > brane i mjerenje (`check:`, brana, gate, test, sonda, probe, mjera) > cigla (oznaka faze) > ostalo | `rad-xlsx.py` |

Override po SHA (ručni podatak) pregazi klasifikator. Redak koji se ne da parsirati se **preskače
i broji** u `touched.skipped_lines`, nikad tiho.

### 2.3 Metrike — paritet s `RAD.xlsx`

- **Tempo po danu:** commiti · kumulativno · redaka ± · sati (proxy) · isporuke · deployi · testni redci.
- **Sati (git-hours proxy, S-007):** commiti se **sortiraju po `author_time` uzlazno**, ne po
  redoslijedu git loga; razmak < 2 h = rad, novi niz = + 0,5 h. Nakon sortiranja negativan razmak je
  nemoguć. Ovo **ispravlja** poznati kvar tablice (−144 h zbog cherry-pickova `5233a0a`, `5da7119`,
  `1e2d157`, čiji je `author_time` stariji od prethodnog commita u logu). Prvi test u jezgri.
  Pragovi (2 h, 0,5 h) su u profilu.
- **Vrste rada:** broj, udio, redci po vrsti; podvrsta po commitu.
- **Pokazatelji kvalitete i brzine:** svih 18 iz lista Sažetak, svaki s `kind: measure | proxy` i
  formulom u JSON-u: radni dani · commiti · commiti/dan · isporuke · isporuke/dan · sati · commiti/sat ·
  redaka ± · testnih redaka · udio testnih · deploya · debugging commita · udio debugginga · udio
  dokumentacije · CI-padova popravljenih · isporuka pokrenutih Leonovim nalazom · zatvorenih faza ·
  prosječno trajanje zatvorene faze.
- **Faze:** zatvorene = konstante iz profila (ime, od, do, regex) i **broje se** iz commita; aktivne
  = čitaju se iz plana (ukupno cigli, `✅` gotovo, stanje planirano / u tijeku / zatvoreno).
- **Vizije:** prolaze nepromijenjene + zbroj po stanju.

### 2.4 Čistoća dokumentacije (`DocsHealth`)

Generičke provjere; svaka vraća nalaz s dokazom (`datoteka:redak`). Projekt bez `docs/` dobiva
`None`, **ne nulu**.

| provjera | zadano | podrijetlo |
|---|---|---|
| mrtve relativne `.md` poveznice | uvijek | Sokrat `check:docs` §1 |
| `.md` koji nije u indeksu | ako indeks postoji (`docs/README.md`) | `check:docs` §4 |
| više od jednog aktivnog plana | mapa plana + oznaka `PAUZIRAN` iz profila | `check:docs` §2 |
| dnevnik u definiciji | > 3 datuma u `docs/product/**` | `check:docs` §3 |
| **kašnjenje docs-a** | dani između zadnjeg commita koda i zadnje promjene dnevnika/changeloga | novo |
| budžet ključne datoteke | bajtovi `CLAUDE.md` bez `\r`, prag iz profila | `check:docs` |
| udio dokumentacijskih commita | informativno, ne ocjenjuje se | `RAD.xlsx` |

Ocjena 0–100 je **zbroj težina iz profila**, ali se uvijek prikazuje i popis nalaza; brojka bez popisa
je zabranjena (PRD §6).

### 2.5 Signali smjera

```rust
trait Rule { fn id(&self) -> &'static str; fn evaluate(&self, ctx: &Context) -> Vec<Signal>; }
struct Signal { rule: &'static str, severity: Info | Warn | Alert, title_key: String,
                evidence: Vec<String>, since: Option<i64> }
```

`Context` je sve što pravilo smije vidjeti, i ništa više: `profile` · `now` · commiti zadane grane ·
`branches: Vec<BranchInfo { name, last_commit_time, ahead_of_default, merged }>` · `docs_files:
Vec<DocFile { path, last_change_time }>` · `last_code_commit: Option<Commit>`. **Commit koda** =
commit koji dira bar jednu datoteku izvan `docs/` koja nije `.md` (profil: `code_paths_exclude`).

*Zašto Rust ovako:* trait objekt po pravilu = svako pravilo je svoja datoteka sa svojim testom;
novo pravilo ne dira postojeća. Pragovi su u profilu sa zadanim vrijednostima. **Signal bez
`evidence` ne prolazi test.**

**M1 pravila (Leonov izbor):**

| pravilo | Warn | Alert | dokaz |
|---|---|---|---|
| `unmerged-branches` | grana izvan zadane starija od 5 dana | starija od 10 dana **ili** više od 3 takve grane | ime grane · dana od zadnjeg commita · commita ispred zadane |
| `docs-lag` | dnevnik/changelog 2+ dana iza zadnjeg commita koda | 5+ dana | SHA i datum zadnjeg commita koda · datum zadnje promjene dnevnika |

**Kasnije (M3, samo popis, u BACKLOG-u):** udio debugginga raste 3 dana zaredom · cigle/dan ispod
prosjeka zatvorenih faza · deploy stariji od 14 dana · udio testnih redaka pada · commiti bez unosa u
dnevniku · faza traje dulje od prosjeka zatvorenih.

---

## 3 · I/O sloj (`sokratis-io`)

- **`GitSource` trait** s jednom implementacijom u M1: **poziv `git` procesa** (`std::process::Command`,
  S-003). Radna stabla i identičan izlaz onome što Leon vidi u terminalu, bez nativnog builda; paritet
  sa skriptom je doslovan. Trait ostaje da `gix` može ući kasnije. Daje točno četiri stvari:
  log zadane grane (format iz §2.2) · popis lokalnih grana sa zadnjim `author_time`, brojem commita
  ispred zadane (`git rev-list --count main..grana`) i je li spojena (`git branch --merged`) · radna
  stabla (`git worktree list --porcelain`) · zadnju promjenu putanje (`git log -1 --format=%at -- <put>`).
- **Profil projekta:** `<repo>/.sokratis/profile.json` (serde), sva polja opcionalna, zadano =
  konvencije Sokrat Studyja (S-005). **Nepoznato polje = greška**, ne tiho ignoriranje.
- **Ručni podaci (S-004):** `<repo>/.sokratis/overrides.json` (`sha → work_kind`) ·
  `<repo>/.sokratis/visions.json`. Putuju s projektom kroz git, kao što danas putuje `RAD.xlsx`.
- **Popis projekata:** `%APPDATA%\sokratis\config.json` (putanje repoa, zadana grana po projektu).
- **Radna stabla:** `git worktree list --porcelain` → identitet = zajednički git-dir.
- **[M2, S-009]** SQLite `%LOCALAPPDATA%\sokratis\sokratis.db`: `snapshot(project, taken_at, report_json)`
  za povijest ocjena · `notify` watcher na `.git/` i mapu docs-a.

**Greške:** nema `git`-a na PATH-u → jasna poruka, izlaz 3 · nema profila → zadano · nema docs-a →
`docs: null` · pokvaren redak → preskoči i broji · nepoznato polje profila → greška s imenom polja.

---

## 4 · CLI (`sokratis-cli`)

| naredba | izlaz | izlazni kod |
|---|---|---|
| `sokratis report [putanja] [--since YYYY-MM-DD] [--json\|--table]` | cijeli `Report` | 0 |
| `sokratis docs [putanja]` | `DocsHealth` s nalazima | 0 |
| `sokratis signals [putanja]` | signali | 0 nema · 1 Warn · 2 Alert · 3 greška okoline |

`signals` s izlaznim kodom je **preflight brana** u Sokrat Studyju: prvi korisnik Sokratisa je
Sokrat Study, i to prije nego postoji ijedan ekran.

---

## 5 · Desktop i sučelje — M2, samo ocrt (vlastiti spec kad dođe na red)

Tauri 2 naredbe: `list_projects` · `add_project` · `get_report` · `get_signals` · `set_override` ·
`save_vision`; događaj `report_updated` iz watchera; tray · autostart · sistemske obavijesti za Alert.
Sučelje Svelte 5 + Tailwind v4 (Vite plugin ili CLI — odluka u M2 specu) + `tokens.css` prenesen iz
Sokrat Studyja (4 teme, zadana svijetla „Akademsko plavo"). Pogledi: **Pregled** (svi projekti +
signali) · **Projekt** (tempo · vrste · kvaliteta · faze · vizije · docs) · **Dnevnik** (commiti,
override vrste u mjestu). Znak Sokratisa je nov; Sokratov logo se ne dira.

---

## 6 · Testiranje (detalji: [workflow/TESTING.md](../workflow/TESTING.md))

- **core:** jedinični testovi po parseru i metrici · **test negativnih sati** s tri stvarna commita iz
  nalaza · **test pariteta**: fixture = sirovi `git log` tekst Sokrat Studyja do 2026-09-16 + snimke
  `PROGRESS.md`/`RASPORED.md` + očekivane brojke iz `RAD.xlsx` od 2026-09-16 (list Sažetak). Očekivana
  razlika je **samo** u satima, i test to izričito tvrdi.
- **io:** integracijski testovi nad privremenim repoom (`git init` + commiti s `GIT_AUTHOR_DATE` i
  `GIT_COMMITTER_DATE`): grupiranje radnih stabala · nespojene grane · kašnjenje docs-a.
- **cli:** snapshot JSON izlaza nad privremenim repoom.
- **Brane prije commita:** `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` · `cargo test`.

---

## 7 · Izlazni uvjet Milestonea 1

Gotovo kad Leon može, iz terminala, nad `sokratstudy.dev`:

1. `sokratis report --json` → iste brojke kao u `RAD.xlsx` od istog dana (osim sati, koji su ispravni
   i to test dokazuje), s `touched` koji kaže koliko je commita i redaka pročitano;
2. `sokratis docs` → ocjena + popis nalaza s `datoteka:redak`;
3. `sokratis signals` → `unmerged-branches` i `docs-lag` s dokazom, i izlazni kod koji može stajati u preflightu;
4. sve to bez ijedne postavke (zadani profil), a s `.sokratis/overrides.json` override vrste preživi ponovno pokretanje;
5. `cargo test` zelen, clippy bez upozorenja, svaka cigla s „zašto Rust ovako" u zaglavlju.

Zatim: **STANI, javi se**, spec seli u `archive/`, nastaje `architecture/ARCHITECTURE.md`, piše se spec za M2.
