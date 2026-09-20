# ARCHITECTURE — što je izgrađeno

**Status:** ✅ opisuje kod koji je u `main`-u — Milestone 1 (verzija 0.1.0, uključujući krug popravaka
nakon završne recenzije) **plus kostur M2, cijel** (`M2/1a`+`M2/1b`: ugovor tipova, crate
`sokratis-store`, `apps/desktop` s ikonama i `npm install`, desktop crate u workspaceu) **plus tok
JEZGRA, gotov** (M2/2…M2/7): snapshot ugovora `Report`-a, `until` kao gornja granica, zbroj vizija,
redci commita/isporuke u `Report`-u, aktivne faze preko `phase_tag` iz profila, `SnapshotMetrics`/
`diff`/`alerts_raised` **plus tok PROFIL, gotov** (M2/8, M2/9): ograda putanja i `test_path_exclude`
— što od kostura još stoji deklarirano bez ponašanja (ili bez potrošača) je u §11 ·
**Zadnja provjera:** 2026-09-18

> **Što ovaj dokument JEST:** opis sustava kakav stoji u `crates/` i `apps/` — granice između crateova, tok
> podataka, formati koje čita i ugovori prema korisniku CLI-ja. **Što NIJE:** kronologija (to su
> [records/CHANGELOG.md](../records/CHANGELOG.md) i [records/PROGRESS.md](../records/PROGRESS.md)),
> plan ([plan/ROADMAP.md](../plan/ROADMAP.md)) ni dom odluka
> ([records/DECISIONS.md](../records/DECISIONS.md), S-001…S-022). Spec po kojem je M1 građen je
> arhiviran: [archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md); aktivan spec M2 (što se tek
> gradi) je [plan/ARHITEKTURA_M2.md](../plan/ARHITEKTURA_M2.md).
>
> **Izvor istine je kod.** Gdje se dokument i kod razilaze, kod je u pravu, a dokument je propust.
> Zato uz svaku tvrdnju stoji datoteka u kojoj se provjerava.

---

## 1 · Četiri cratea i granica među njima

```
Cargo.toml                  # workspace; verzije ovisnosti na jednom mjestu
crates/
  sokratis-core/            # čisti Rust: model · parseri · metrike · docs-ocjena · pravila
  sokratis-io/              # git kroz proces · datoteke · profil · ručni podaci
  sokratis-store/           # SQLite (rusqlite) — od M2/1a: otvaranje baze i migracije, ništa više
  sokratis-cli/             # binarna `sokratis`: report · docs · signals
apps/desktop/               # ljuska M2: Svelte/Vite datoteke + crate `sokratis-desktop` (src-tauri)
```

| crate | smije | ne smije | ulaz u kod |
|---|---|---|---|
| `sokratis-core` | računati nad tekstom i strukturama | otvoriti datoteku, pokrenuti proces, pitati koliko je sati | `src/lib.rs` |
| `sokratis-io` | `std::process::Command` za `git`, `std::fs`, `chrono::Local` za današnji datum | računati metrike | `src/git.rs`, `src/project.rs` |
| `sokratis-store` | otvoriti SQLite bazu i primijeniti migracije; ovisi o `core` (tipovi), **ne o `io`** (S-013) | računati metrike, dirati git | `src/lib.rs`, `src/store.rs` |
| `sokratis-cli` | ispisati JSON ili tablicu i vratiti izlazni kod | računati bilo što | `src/main.rs`, `src/table.rs` |
| `sokratis-desktop` (`apps/desktop/src-tauri`) | pokrenuti Tauri ljusku: prozori, plugini, kasnije naredbe | držati išta što bi se htjelo testirati (S-013) | `src/main.rs`, `src/lib.rs` |

**Što `store` danas radi:** `Store::open(path)` / `Store::open_in_memory()` otvore vezu, primijene
migraciju `src/migrations/0001_init.sql` (sedam tablica: `project` · `project_worktree` · `setting` ·
`project_setting` · `profile_seen` · `snapshot` · `commit_cache`) i javljaju `schema_version()`.
Nijedan redak se još ne piše ni čita: registar projekata, postavke i snimke dolaze s ciglama
M2/15–M2/17, keš tek ako ga mjerenje zatraži (S-014). Baza će živjeti u `%LOCALAPPDATA%\sokratis\`;
0.1.0 je ne stvara — CLI `store` ne koristi.

**`sokratis-desktop` je u `[workspace] members` i builda se** (`M2/1b`, 2026-09-18): `npm install`
uz Leonov OK (80 paketa, 0 ranjivosti) i `npm run tauri icon` iz Leonova loga popunili su
`src-tauri/icons/`, pa `tauri-build` ima što tražiti. Prvi `cargo build -p sokratis-desktop` je
trajao ~3 min. `cargo test --workspace` dotiče crate; `apps/desktop` ima `node_modules` i `npm run
check`/`npm run build` rade (`dist/` s `index.html` i `splash.html`).

**Granica S-002:** jezgra ne zna odakle su podaci došli. Sve što joj treba dolazi u jednoj strukturi
(`ReportInput`: `git_log` kao tekst, dnevnik i plan kao tekst, docs, grane, ručni podaci, `now`,
`today`, `since`, `until`, `branch`) i vraća se jedna struktura (`Report`). Zato se jezgra testira bez
gita, i zato će je Tauri u M2 koristiti bez ijedne izmjene (S-012).

**Granica S-003:** git se čita **kroz proces**, ne kroz biblioteku. Poziv se gradi bez shella
(`Command::new("git").arg("-C")…`), pa nema escapinga. Iza traita `GitSource` (osam metoda, jedna
implementacija `GitCli`) stoji mjesto na koje kasnije može ući `gix` bez dizanja jezgre.

## 2 · Tok podataka

```
putanja repoa
  → io:  Project::open        .sokratis/profile.json ili Profile::default()   (S-005)
         Project::input        git log · dnevnik · plan · docs · grane · overridei · vizije
  → core: build_report         parse → metrike → docs-ocjena → pravila → Report
  → cli:  JSON | tablica | izlazni kod
```

Korak po korak, redoslijed je u `core/src/report.rs` (`build_report`) i nigdje drugdje:

1. **provjera ulaza:** `Patterns::compile` kompilira regexe iz profila jednom po izvještaju i traži
   da imaju grupe koje parser čita (`diary_heading` 3 · `plan_brick` 3 · `plan_phase_name` 2 ·
   `phase_tag` 1) → `ParseError::BadPattern`; `Profile::validate_dates` i provjera `input.since`
   traže oblik `YYYY-MM-DD` → `ParseError::BadDate { field, text }`. Valjan JSON s regexom bez grupe
   ili s tipfelerom u datumu je **greška s imenom polja** (izlaz 3), ne panika i ne tiha kriva brojka.
   Uz njih se od `M2/1a` zove i `Profile::validate_paths` (ograda putanja iz profila na korijen
   repoa): `inside_root` razlaže putanju kroz `std::path::Component` i s `matches!` odbija sve osim
   `Normal`/`CurDir` (dakle `..`, apsolutnu putanju, `C:\…`, `\\server\share`), bez ijednog diranja
   diska (S-002) → `ParseError::PathOutsideRoot { field, value }` imenuje prvo pogođeno od osam
   polja (M2/8, jezgreni dio duga I9). Io-dio ograde — provjera pri `Project::open`, prije nego se
   ijedna putanja pročita s diska — dolazi u T14 (§11).
2. `parse_git_log` — tekst loga → `Vec<Commit>` + broj preskočenih redaka.
3. filtar `since`: ostaju commiti s `commit_date >= since` (S-011 — isti kriterij kao `git log --since`).
4. `parse_diary` — naslovi dnevnika → `Vec<Delivery>`; `parse_plan` — redovi plana → faze.
5. `hours_per_day` — sati po danu (sortirano po `author_time`, S-007).
6. `commit_rows` — klasificira svaki commit (vrsta uz override, podvrsta) u redak za `Report.commits`
   (M2/5); `day_stats` — redak po danu s commitom; `kind_stats` broji **iz tih redaka** (posuđenih
   prije nego se pomaknu u `Report`), umjesto da klasifikaciju ponovi.
7. faze: zatvorene se **broje** iz cijelog loga; aktivne se na filtrirane commite vežu regexom
   `phase_tag` iz profila (M2/6, dug I3+M14 riješen), ne više tvrdim prefiksom `"{id}/"`. Zatvorena
   faza **bez ijednog pogođenog commita** ne ulazi ni u pokazatelje ni u tablicu — zatvorene faze
   dolaze iz profila, pa bi tuđi projekt sa zadanim profilom (S-005) dobio faze iz zraka.
8. `indicators` — 18 pokazatelja, svaki s `kind` (`measure` ili `proxy`) i formulom. `IndicatorInput`
   nosi i `since` (iz `ReportInput`, dakle `--since`): pokazatelj o zatvorenim fazama mjeri isto
   razdoblje kao ostatak izvještaja, ne `profile.since`. **Nedovršeno (§11):** nekoliko pokazatelja
   ovdje zove `effective_kind` odvojeno od koraka 6, pa se commit klasificira više od jednom po
   izvještaju — spec §3.2 traži jednom.
9. `docs_health` — ocjena i nalazi; `evaluate_all(default_rules())` — signali.
10. `Report` se sastavi i serializira (`serde`); `vision_totals` (zbroj vizija po stanju, M2/4) se
    računa u istom koraku iz `input.visions`.

**Grana:** metrike se računaju nad `profile.default_branch` ako ta grana postoji, inače nad trenutnom
granom (`io/src/project.rs::input`). Ostale grane ulaze **samo u signale**. Ako ni jedna ni druga ne
postoji kao referenca (repo nakon `git init`, bez commita), `io` vraća `IoError::NoCommits` →
`<putanja>: repozitorij nema commita` i izlaz 3; prije je korisnik dobivao gitov savjet o `--`.

## 3 · Što `Report` nosi

`core/src/model.rs`. Petnaest polja: `generated_at` · `since` · `until` · `branch` · `touched` ·
`days` · `kinds` · `commits` · `deliveries` · `indicators` · `phases` · `visions` · `vision_totals` ·
`docs` (`null` kad projekt nema mapu s dokumentacijom — nula bi bila laž) · `signals`.

**Tri polja koja je `M2/1a` deklarirala prazna sad jezgra puni** (M2/4, M2/5) — deklarirana su prije
potrošača da sučelje i snapshot ugovora (S-022) ne mijenjaju oblik svakom ciglom:

| polje | što nosi | puni ga |
|---|---|---|
| `commits` | redak po commitu (`CommitRow`: `sha` · `date` · `subject` · `kind` · `sub` · `overridden`) — ulaz za budući pogled Dnevnik | M2/5 |
| `deliveries` | redak po isporuci (`Delivery`: `date` · `model` · `title` · `kind` · `deploy`); M1 je isporuke iz dnevnika samo zbrajao po danu, sad postoji i popis — ulaz za budući pogled Isporuke | M2/5 |
| `vision_totals` | zbroj vizija po stanju (`VisionTotal { state, count }`, dug I6) | M2/4 |

**CLI-tablica (`cli/src/table.rs`) ova tri polja još ne ispisuje** — korisnik CLI-ja zato ne vidi
ništa novo; podaci postoje u JSON-u i čekaju sučelje M2 (§11).

**`until` (M2/3) je iznimka istog oblika — jezgra ga puni i njime filtrira, ali mu nitko još ne šalje
pravu vrijednost.** `ReportInput.until: Option<String>` prolazi istu provjeru oblika kao `since`
(`ParseError::BadDate { field: "until", .. }`, let-chain u `build_report`) i filtrira commite i
isporuke tako da ostane samo `commit_date`/`date <= until` (gornja granica uključuje cijeli dan,
S-011). `civil::next_day` postoji za rezervu zone, isto zrcalo `prev_day`-a. U svakom stvarnom pozivu
danas je vrijednost i dalje `null`/`None`: `io` ga tako šalje (`--until` prema gitu i ograda pri
učitavanju su IO T14), a CLI nema `--until` (T19) — tek ti potrošači daju prozoru stvarnu gornju
granicu. Snimka ugovora (`snapshot.rs`) zato i dalje ima `until: null`.

**`core/src/snapshot.rs` više nije prazan modul** (M2/7): `SnapshotMetrics::from_report`, `diff`,
`SignalCounts::from_signals`, `worst_severity`, `alerts_raised` postoje i imaju testove — pune se
tipovi `SignalCounts`, `MetricValue { id, value, kind }`, `SnapshotMetrics`, `MetricDelta` iz
`model.rs`. Potrošača (snimke, STORE M2/17; obavijesti na prijelaz u Alert, DESKTOP T29/T30) još
nema — detalji u §11.

**`touched` je mjerač mjerača** — koliko je izvještaj stvarno dotaknuo (`core/src/report.rs`):

| polje | što je | pažnja |
|---|---|---|
| `commits` | broj commita nakon filtra `since` | |
| `lines` | Σ `added + deleted` po svim izmjenama datoteka | |
| `files` | **broj izmjena datoteka kroz commite**, ne broj različitih datoteka | datoteka dirnuta u 10 commita doda 10 |
| `skipped_lines` | redaka `numstat`-a koje parser nije razumio | preskočeno se broji, nikad tiho ne ispari |

## 4 · Profil projekta — sva polja i zadane vrijednosti

**Izvor je `crates/sokratis-core/src/profile.rs`** (`impl Default for Profile`); ova tablica prati
njega i ima jednako redaka koliko struktura ima polja (**39** od `M2/1a`). Zadane vrijednosti **jesu**
konvencije Sokrat Studyja (S-005): prvi korisnik radi bez ijedne postavke. Profil je
`#[serde(default, deny_unknown_fields)]` — polje koje nedostaje uzima zadano, polje s tipfelerom je
greška, ne tiho ignoriranje (i to jedna poruka s putanjom, ne dvije). Ovo je jedina tablica profila
u dokumentaciji.

| polje | zadano | čemu služi |
|---|---|---|
| `default_branch` | `"main"` | grana nad kojom se računaju metrike |
| `since` | `"2026-08-29"` | od kada se mjeri kad `--since` nije dan |
| `diary_path` | `"docs/records/PROGRESS.md"` | dnevnik iz kojeg se čitaju isporuke |
| `changelog_path` | `"docs/records/CHANGELOG.md"` | drugi dokument koji smije „pokriti" kašnjenje docs-a |
| `plan_path` | `"docs/plan/RASPORED.md"` | plan iz kojeg se čitaju cigle i faze |
| `docs_dir` | `"docs"` | mapa koja se rekurzivno čita (`node_modules`, `.git`, `target` se preskaču) |
| `docs_index` | `"docs/README.md"` | indeks; dokument koji u njemu nije spomenut je nalaz |
| `plan_dir` | `"docs/plan"` | mapa u kojoj se traži točno jedan aktivan spec |
| `plan_dir_ignore` | `["ROADMAP.md"]` | datoteke u `plan_dir` koje nisu spec |
| `paused_marker` | regex (blok §5) | oznaka da spec ne sudjeluje u „koliko je planova aktivno" |
| `product_dir` | `"docs/product"` | mapa definicije proizvoda (ne smije biti dnevnik) |
| `key_file` | `"CLAUDE.md"` | datoteka kojoj se mjeri veličina |
| `key_file_budget_bytes` | `40000` | prag; mjeri se bez `\r` |
| `diary_heading` | regex (blok §5) | naslov unosa u dnevniku → datum, model, naslov |
| `diary_deploy_pattern` | regex (blok §5) | unos u dnevniku koji znači deploy |
| `plan_brick` | regex (blok §5) | redak cigle u planu; `✅` znači gotova |
| `plan_phase_name` | regex (blok §5) | naslov faze u planu |
| `phase_tag` | regex (blok §5) | oznaka aktivne faze u opisu commita; commit se veže na fazu (`ph.id` ili dijete `"{id}/"`) ako mu poklapa (dug I3+M14 riješen, M2/6) |
| `classifier` | 4 pravila (blok §5) | uređena lista `(vrsta, regex)`; **redoslijed je ugovor** |
| `gate_pattern` | regex (blok §5) | podvrsta „brana i mjerenje" |
| `deploy_pattern` | regex (blok §5) | podvrsta „deploy" |
| `ci_fix_pattern` | regex (blok §5) | pokazatelj „CI-padova popravljenih" |
| `owner_name` | `"leon"` | isporuka se smatra vlasnikovom ako mu je ime u naslovu |
| `test_path_prefixes` | `["tests/"]` | testna putanja: počinje s… |
| `test_path_contains` | `["/check-"]` | …ili sadrži… |
| `test_path_suffixes` | `[".test.js", ".spec.js"]` | …ili se završava na |
| `test_path_exclude` | `[]` | podputanje koje se **ne** broje kao test iako su pod testnom putanjom (npr. `fixtures/`); `is_test_path` ga provjerava prvo, kao stražarsku klauzulu (M2/9) — zadano `[]` čuva paritet, Sokratisov vlastiti profil ga postavlja tek u T35 |
| `code_exclude_prefixes` | `["docs/"]` | putanja koja se NE smatra kodom (počinje s) |
| `code_exclude_suffixes` | `[".md"]` | putanja koja se NE smatra kodom (završava na) |
| `session_gap_hours` | `2.0` | razmak manji od toga = neprekinut rad |
| `session_start_hours` | `0.5` | fiksni dodatak za prvi commit nove sesije |
| `closed_phases` | 4 faze Sokrat Studyja | `{name, from, to, tag_pattern, note}`; broje se iz commita |
| `include_unmerged` | `false` | **rezervirano — jezgra ga u 0.1.0 ne čita** (vidi §11) |
| `unmerged_warn_days` | `5` | grana starija od toga → Warn |
| `unmerged_alert_days` | `10` | …starija od toga → Alert |
| `unmerged_alert_count` | `3` | …ili više od toliko takvih grana → Alert |
| `docs_lag_warn_days` | `2` | dnevnik toliko dana iza koda → Warn |
| `docs_lag_alert_days` | `5` | …toliko → Alert |
| `docs_weights` | `dead_link 5` · `not_indexed 3` · `multiple_plans 15` · `no_active_plan 10` · `diary_in_definition 5` · `lag 10` · `key_file_budget 5` | koliko koji nalaz odbija od 100 |

`Profile::log_since()` uzima **najraniji** datum od `since` i početaka zatvorenih faza — git log mora
dovući i commite starije od `since` da bi se zatvorene faze mogle prebrojati. Prozor dovlačenja je
zatim `min(log_since(), --since)`: bez tog `min`-a je `--since` stariji od profila tiho dobivao
kraći log nego što `Report.since` tvrdi (zaglavlje i podaci moraju se odnositi na isto razdoblje).

## 5 · Zadani regexi i klasifikator — doslovno

Iz `profile.rs`; ovdje su izvan tablice jer sadrže znak `|`.

```jsonc
"paused_marker":        "^\\s*\\*\\*Status:\\*\\*\\s*⏸️\\s*PAUZIRAN\\b"
"diary_heading":        "^## (\\d{4}-\\d{2}-\\d{2})(?:\\s*\\(([^)]*)\\))?\\s*[—-]+\\s*(.+)$"
"diary_deploy_pattern": "deploy|main na `"
"plan_brick":           "^\\| \\*\\*(F\\d)/(\\d+)\\*\\*\\s*(✅?)"
"plan_phase_name":      "^### (F\\d) · (.+)$"
"phase_tag":            "^(F\\d/\\d|C\\d[ab]?(?:/\\d\\w*)?|MREZA[- ]?[A-E]\\d?(?: \\(\\d/\\d\\))?|R\\d(?:/[A-Z0-9+]+)?|T\\d|ALAT-\\d|BUG-\\d+|U\\d)"
"gate_pattern":         "check:|brana|gate|^alat|^test|sonda|probe|mjera"
"deploy_pattern":       "na produkciji|deploy"
"ci_fix_pattern":       "^ci:|popravak ci|job je otkazan"

// classifier — redoslijed je ugovor: prvi regex koji pogodi pobjeđuje,
// a commit koji ne pogodi nijedan je "execution" (izvođenje procesa).
1. "planning"      → "raspored|dobiva svoj spec|tracnice|…|zadatak za sljedecu"
2. "documentation" → "^docs|compact|revizija pred|^r\\d/docs|ishod --|deploy-zapis|zapis uz"
3. "debugging"     → "^fix|bug-\\d|^ci:|popravak ci|kvar|obara|^test:|lagati|…|više ne postoji"
4. "polish"        → "^c[4-7]|paleta|mrtv|selektorski|\\bvan\\b|audit|…|dug plaćen"
```

Regexi su preneseni 1:1 iz `rad-xlsx.py` Sokrat Studyja, zato su hrvatski i zato su takvi kakvi su.
Puni tekst svakog je u `profile.rs` — ovdje su skraćeni tri točkice tamo gdje je lista dugačka.

**Format git loga** koji `io` traži i jezgra razumije (`io/src/git.rs`, `core/src/parse/gitlog.rs`):
`--format=@@%h|%at|%ct|%ad|%cd|%s` + `--numstat` + `--date=format:%Y-%m-%d` + `--reverse` + završni
`--` (bez njega je ime grane dvosmisleno s datotekom istog imena). Unix-vremena (`%at`, `%ct`) služe
za razmake, lokalni datumi za dan u tablici (`%ad`, autorov — paritet s `RAD.xlsx`) i za filtar
`since` (`%cd`, commitov — kao git). `--since` se **uvijek** šalje sa satom `00:00:00` (S-011): bez
sata git uzima trenutno doba dana, pa bi isti datum davao različit broj commita ovisno o tome kad se
izvještaj pokreće.

**Rezerva od jednog dana:** `io` traži log od **dana prije** granice (`civil::prev_day`). Razlog je
zona: `--since … 00:00:00` je ponoć u zoni **stroja**, a datumi u logu su u zoni **commita**, pa bi
stroj zapadnije od pohranjenog pomaka odbacio commite koje jezgrin filtar zadržava (najosjetljiviji
je prvi dan zatvorene faze). Mjerodavan je i ostaje jezgrin filtar `commit_date >= since` nad
tekstom; rezerva zato ne mijenja ni jednu brojku, samo čini izvještaj neovisnim o zoni stroja
(dopuna S-011).

## 6 · `.sokratis/` — ručni podaci u repou (S-004)

Tri datoteke u korijenu repoa, sve tri neobavezne, sve tri putuju kroz git s projektom.
Čita ih `io/src/project.rs`. „Datoteka ne postoji" je jedina greška koja tiho pada na zadano; svaka
druga (mapa umjesto datoteke, nema dozvole, pokvaren JSON) se javlja s putanjom.

**`.sokratis/profile.json`** — bilo koji podskup polja iz §4. Primjer je profil kojim Sokratis mjeri
sam sebe (dogfooding): vlastiti plan nema cigle ni faze, a testovi mu žive u `crates/*/tests/`.
`phase_tag` je u njemu upisan i od M2/6 stvarno radi: aktivne faze se na commite vežu ovim regexom,
ne više tvrdo kodiranim prefiksom `"{id}/"`.

```json
{
  "since": "2026-09-17",
  "plan_path": "docs/plan/ROADMAP.md",
  "closed_phases": [],
  "phase_tag": "^(M\\d/\\d+)",
  "owner_name": "leon",
  "test_path_contains": ["/tests/"]
}
```

**`.sokratis/overrides.json`** — ručna vrsta rada po SHA-i commita; pregazi klasifikator. Ključ je
kratka SHA kakvu daje `%h`, vrijednost jedan od pet engleskih identifikatora (S-008): `planning` ·
`documentation` · `execution` · `polish` · `debugging`.

```json
{ "8de5866": "polish", "687db7a": "documentation" }
```

**`.sokratis/visions.json`** — vizije prolaze kroz izvještaj nepromijenjene (jezgra ih ne računa).

```json
[
  { "title": "Sokratis mjeri sam sebe", "source": "Leon", "state": "u tijeku",
    "percent": 80, "note": "dogfooding profil od T22" }
]
```

## 7 · Čistoća dokumentacije (`DocsHealth`)

`core/src/docs.rs`. Ulaz su svi `*.md` iz korijena repoa i rekurzivno iz `docs_dir`, sa sadržajem i
vremenom zadnje promjene iz gita. Projekt bez `docs_dir` dobiva `docs: null`.

| provjera | nalaz kad | dokaz |
|---|---|---|
| `dead-link` | relativna `.md` poveznica ne pokazuje na postojeću datoteku | `datoteka:redak` |
| `not-indexed` | `.md` pod `docs_dir` nije spomenut u `docs_index` | datoteka |
| `multiple-active-plans` | više od jednog nepauziranog spec-a u `plan_dir` | popis putanja |
| `no-active-plan` | postoje specovi, ali su svi pauzirani | mapa plana |
| `diary-in-definition` | više od tri datuma u datoteci pod `product_dir` | broj datuma |
| `docs-lag` | dnevnik/changelog kasni `docs_lag_warn_days`+ dana za zadnjim commitom koda | broj dana |
| `key-file-budget` | `key_file` je veći od budžeta | bajtovi i prag |

Ocjena je `100 − Σ težina nalaza` (`saturating_sub`, nikad ispod nule). **Brojka se nikad ne
prikazuje bez popisa nalaza.** Poveznica unutar ograde kôda (```` ``` ````) je primjer, ne poveznica —
provjera je namjerno preskače.

## 8 · Signali smjera

`core/src/rules/`. Jedno pravilo = jedna datoteka + jedan test; `trait Rule` je ugovor, a
`default_rules()` vraća `Vec<Box<dyn Rule>>` pa novo pravilo ne dira postojeća. **Signal bez dokaza
(`evidence`) ne prolazi test.**

| pravilo | Warn | Alert | dokaz |
|---|---|---|---|
| `unmerged-branches` | nespojena grana izvan zadane starija od `unmerged_warn_days` | najstarija starija od `unmerged_alert_days` **ili** više od `unmerged_alert_count` takvih grana | ime grane · dana od zadnjeg commita · commita ispred zadane |
| `docs-lag` | dnevnik/changelog `docs_lag_warn_days`+ dana iza zadnjeg commita koda | `docs_lag_alert_days`+ dana | SHA i datum zadnjeg commita koda · koliko dana kasni dnevnik |

**Commit koda** = commit koji dira bar jednu putanju koju profil ne isključuje
(`code_exclude_prefixes`/`code_exclude_suffixes`) — zadano: sve osim `docs/` i `*.md`.

## 9 · CLI — naredbe i izlazni kodovi

`cli/src/main.rs`; hrvatske natpise za tablicu daje `cli/src/table.rs`, identifikatori u JSON-u
ostaju engleski (S-008). Putanja je neobavezna; bez nje je to trenutna mapa. Radi i iz podmape i iz
radnog stabla — korijen se dobiva iz `git rev-parse --show-toplevel`.

| naredba | ispis | izlazni kod |
|---|---|---|
| `sokratis report [putanja] [--since YYYY-MM-DD] [--json\|--table]` | cijeli `Report`; **bez zastavice je JSON**, `--table` daje tablicu s hrvatskim natpisima; zastavice su **isključive** (`--json --table` je pogrešna uporaba, ne „zadnja pobjeđuje") | 0 |
| `sokratis docs [putanja] [--json]` | ocjena, broj nalaza, kašnjenje; bez `docs_dir` poruka `docs: nema mape s dokumentacijom (n/a)` | 0 |
| `sokratis signals [putanja] [--json]` | signali s dokazom, ili `nema signala` | **0** nema · **1** Warn · **2** Alert |

Svaka greška okoline (nema `git`-a na PATH-u, putanja nije repozitorij, repozitorij bez commita,
pokvaren profil, tipfeler u datumu ili regex bez grupe) ispisuje se na stderr kao
`sokratis: <poruka>` i daje **izlazni kod 3**. Isti kod dobiva i **pogrešna uporaba CLI-ja**
(nepoznata zastavica, `--json --table`): `clap` bi sam izašao s **2**, a 2 je rezerviran za Alert, pa
`main` koristi `try_parse` i razliku presuđuje po tome piše li `clap` na stderr — `--help` i
`--version` idu na stdout i daju **0**. `process::exit` je u `main` i nigdje drugdje: izlazni kod je
ugovor prema preflightu, a ne nuspojava.

`--since` postoji samo na `report`; `docs` i `signals` uzimaju `since` iz profila.

## 10 · Dvije zamke koje je otkrio dogfooding

1. **„Testni redak" se prepoznaje SAMO po putanji** (`test_path_prefixes` · `test_path_contains` ·
   `test_path_suffixes`, `Profile::is_test_path`). Posljedice koje treba znati prije čitanja brojke
   „udio testnih redaka": inline `#[cfg(test)] mod tests` unutar `src/*.rs` se **ne broji** kao test
   (živi u istoj datoteci kao produkcijski kod), a **sve** pod testnom putanjom se broji — i fixture
   datoteke pod `tests/`, koje nisu kod testa. Sokratis zato u svom profilu ima
   `test_path_contains: ["/tests/"]`: njegovi testovi žive u `crates/*/tests/`, što zadani prefiks
   `tests/` ne hvata. Posljedica koju treba znati pri čitanju **Sokratisova vlastitog** udjela
   testnih redaka: pod tom putanjom leže i fixture datoteke (snimka `PROGRESS.md` Sokrat Studyja ima
   767 kB), pa je većina njegovih „testnih redaka" fixture, ne kod testa. Polje koje to rješava
   (`test_path_exclude`) sad radi (M2/9), ali Sokratisov vlastiti profil ga još ne postavlja —
   `["/fixtures/"]` dolazi tek u T35, do tada fixture i dalje napuhuje njegov udio testnih redaka.
2. **`touched.files` je broj izmjena datoteka, ne broj različitih datoteka** (§3). Ista datoteka
   dirnuta u deset commita doda deset. Brojka odgovara na „koliko je izmjena pročitano", ne na
   „koliko datoteka projekt ima".

## 11 · Što stoji u kodu, a još ne izlazi ili ne radi (0.1.x)

Uredno zapisani propusti, ne skrivene rupe (CLAUDE.md #4). Ovo je **stanje koda**; što se od toga
planira uzeti i kada je u [`../records/BACKLOG.md`](../records/BACKLOG.md), a za stavke koje je
preuzeo M2 u [`../plan/ARHITEKTURA_M2.md`](../plan/ARHITEKTURA_M2.md) §8 i planu cigli
[`../superpowers/plans/2026-09-18-m2-desktop.md`](../superpowers/plans/2026-09-18-m2-desktop.md).

**Rezervirano polje profila** (deklarirano, jezgra ga ne čita):

- **`include_unmerged`** — metrike su uvijek samo nad zadanom granom, a nespojene grane ulaze
  isključivo u signale.

(`phase_tag` je do M2/6 bilo ovdje — sad radi, §4 i §6.)

**Izračunato, ali ne izlazi u CLI-tablicu** (JSON ga od M2/4–M2/5 nosi, §3):

- **`GitSource::worktrees`** je implementiran i testiran, ali izvještaj ga ne koristi: identitet
  projekta preko više radnih stabala je posao M2.
- **`Report.commits`/`deliveries`/`vision_totals`** postoje u JSON-u, ali `cli/src/table.rs` ih ne
  ispisuje — redak po commitu (uz `classify_sub`, sad `CommitRow.sub`), redak po isporuci i zbroj
  vizija po stanju čekaju pogled Dnevnik/Isporuke/Vizije u sučelju M2.

**Rubovi koje kod danas ne pokriva:**

- **Commit se klasificira više od jednom po izvještaju**, iako spec §3.2 traži jednom.
  `metrics/kinds.rs::commit_rows` (M2/5) klasificira svaki commit točno jednom za `Report.commits`, a
  `kind_stats` broji iz tih redaka — ali `metrics/indicators.rs:31` (`kind_count`, pokazatelji
  `debugging_commits` i `docs_share`) i dalje zove `effective_kind` odvojeno. Nalaz recenzije M2/5
  (2026-09-18), otvoren do završne recenzije M2: [`../records/BACKLOG.md`](../records/BACKLOG.md).
- **Putanje iz profila su ograđene u jezgri, ali ne još pri otvaranju projekta (I9, djelomično
  riješeno).** `Profile::validate_paths` (M2/8) sad odbija `..`, apsolutnu putanju i UNC/drive-prefiks
  za svih osam polja i `build_report` je zove prvi korak — pogrešan profil je greška s imenom polja
  (`ParseError::PathOutsideRoot`), ne tiha kriva putanja. Preostaje `io::Project::open` (T14): dok ta
  cigla ne uđe, `root.join(docs_dir)` u `io`-u i dalje prihvaća `..`/apsolutnu putanju (Rustov `join`
  apsolutnu **zamijeni**) prije nego jezgra uopće dobije priliku odbiti profil, pa `"docs_dir":
  "../.."` zna pročitati `.md` izvan repoa i tek onda pasti na gitu s porukom koja ne kaže što je
  krivo.
- **Detached HEAD:** `git branch --show-current` je prazan i kad repo ima commite, pa takav radni
  primjerak dobiva poruku `repozitorij nema commita` — kriva poruka u rubnom stanju koje CLI ne cilja.
- **Tablični ispis (`cli/src/table.rs`) nije dovršen kao sučelje:** udjeli su goli razlomci
  (`0.589`) dok VRSTE RADA imaju procente, stanje faze ide kroz `{:?}` pa u hrvatskoj tablici stoji
  `Closed`/`Running`/`Planned` bez prijevoda (S-008: natpise daje sučelje), prazan naslov „FAZE"
  ostaje bez ijednog retka, a ime faze dulje od 50 znakova prelije stupac. JSON je ugovor i on je
  točan; tablica je pomoć za terminal.
- **Cijena su procesi, ne parsiranje:** izvještaj nad Sokrat Studyjem traje ~3,2 s jer se zove
  `git log -1` za **svaku** `.md` datoteku (56) i `rev-list --count` za **svaku** granu (31) — uz to
  jezgra dio commita klasificira više puta (gore). Nad Sokratisom je to 0,7 s. Mjerljivo, ne
  pogađano. **Tok IO (M2/11, izvan `main`-a) ovaj git-trošak već mjeri i smanjuje**: 92 → 6 procesa,
  3318–3822 ms → 1479,71 ms nad Sokrat Studyjem (release, topli keš) — ulazi u `main` s tim tokom.
- **SQLite snimke, watcher i popis projekata** su M2 (S-009); 0.1.x sve računa na zahtjev i ne piše
  ništa osim onoga što korisnik sam stavi u `.sokratis/`.

**Kostur M2 (`M2/1a`+`M2/1b`) — deklarirano, bez ponašanja; tok JEZGRA (M2/2…M2/7) i tok PROFIL
(M2/8, M2/9) su ponašanje već dodali, oba gotova.** Cigla je namjerno unijela ugovor prije potrošača
(jedna zajednička točka, pa se tokovi poslije ne sudaraju); dio i dalje stoji u kodu bez potrošača ili
čeka drugi tok:

- **`until` validira i filtrira jezgra (M2/3), ali mu `io`/CLI još ne šalju stvarnu vrijednost**
  (T14/T19) — §3;
- **`Profile::validate_paths` i `test_path_exclude` više nisu stub/prazni** (M2/8, M2/9) — rade ono
  što ime kaže. Ostaje samo io-dio ograde: `Project::open` još ne zove `validate_paths` (I9
  djelomično, T14) — §2, §4, §11 gore;
- **`core/src/snapshot.rs` više nije prazan modul** (M2/7): `SnapshotMetrics::from_report`, `diff`,
  `SignalCounts::from_signals`, `worst_severity`, `alerts_raised` rade i imaju testove; `CommitRow` i
  `VisionTotal` već imaju potrošača (`Report.commits`/`vision_totals`, M2/4–M2/5), ali
  `SnapshotMetrics`/`MetricValue`/`MetricDelta`/`SignalCounts` još nemaju — čekaju STORE (snimke,
  M2/17) i DESKTOP (obavijest na prijelaz u Alert, T29/T30) — §1, §3;
- **`sokratis-store` zna otvoriti bazu i migrirati shemu, ali ga nitko ne zove**: ni CLI ni desktop
  nemaju tu ovisnost, pa baza nastaje samo u testu (registar M2/15, postavke M2/16, snimke M2/17) — §1;
- **ovisnost bez potrošača:** `notify` u `io` (watcher M2/13) — pinana u `M2/1a` s obrazloženjem
  ([`../workflow/RUST.md`](../workflow/RUST.md) §2), a ne nuspojava. `insta` više nije na ovom popisu:
  od M2/2 stvarno testira (`crates/sokratis-core/tests/snapshot.rs`, S-022);
- **`apps/desktop` se builda, ali ne radi ništa:** crate `sokratis-desktop` je u workspaceu i pokreće
  praznu Tauri ljusku (splash + glavni prozor bez sadržaja) — nijedna naredba prema jezgri ili
  `sokratis-store` još ne postoji, te dolaze s ciglama STORE/SUČELJE/DESKTOP — §1.
