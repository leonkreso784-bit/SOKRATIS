# ARCHITECTURE — što je izgrađeno

**Status:** ✅ opisuje kod koji je u `main`-u (Milestone 1, verzija 0.1.0) · **Zadnja provjera:** 2026-09-17

> **Što ovaj dokument JEST:** opis sustava kakav stoji u `crates/` — granice između crateova, tok
> podataka, formati koje čita i ugovori prema korisniku CLI-ja. **Što NIJE:** kronologija (to su
> [records/CHANGELOG.md](../records/CHANGELOG.md) i [records/PROGRESS.md](../records/PROGRESS.md)),
> plan ([plan/ROADMAP.md](../plan/ROADMAP.md)) ni dom odluka
> ([records/DECISIONS.md](../records/DECISIONS.md), S-001…S-011). Spec po kojem je M1 građen je
> arhiviran: [archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md).
>
> **Izvor istine je kod.** Gdje se dokument i kod razilaze, kod je u pravu, a dokument je propust.
> Zato uz svaku tvrdnju stoji datoteka u kojoj se provjerava.

---

## 1 · Tri cratea i granica među njima

```
Cargo.toml                  # workspace; verzije ovisnosti na jednom mjestu
crates/
  sokratis-core/            # čisti Rust: model · parseri · metrike · docs-ocjena · pravila
  sokratis-io/              # git kroz proces · datoteke · profil · ručni podaci
  sokratis-cli/             # binarna `sokratis`: report · docs · signals
```

| crate | smije | ne smije | ulaz u kod |
|---|---|---|---|
| `sokratis-core` | računati nad tekstom i strukturama | otvoriti datoteku, pokrenuti proces, pitati koliko je sati | `src/lib.rs` |
| `sokratis-io` | `std::process::Command` za `git`, `std::fs`, `chrono::Local` za današnji datum | računati metrike | `src/git.rs`, `src/project.rs` |
| `sokratis-cli` | ispisati JSON ili tablicu i vratiti izlazni kod | računati bilo što | `src/main.rs`, `src/table.rs` |

**Granica S-002:** jezgra ne zna odakle su podaci došli. Sve što joj treba dolazi u jednoj strukturi
(`ReportInput`: `git_log` kao tekst, dnevnik i plan kao tekst, docs, grane, ručni podaci, `now`,
`today`, `since`, `branch`) i vraća se jedna struktura (`Report`). Zato se jezgra testira bez gita, i
zato će je Tauri u M2 koristiti bez ijedne izmjene.

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

1. `Patterns::compile` — regexi iz profila se kompiliraju jednom po izvještaju.
2. `parse_git_log` — tekst loga → `Vec<Commit>` + broj preskočenih redaka.
3. filtar `since`: ostaju commiti s `commit_date >= since` (S-011 — isti kriterij kao `git log --since`).
4. `parse_diary` — naslovi dnevnika → `Vec<Delivery>`; `parse_plan` — redovi plana → faze.
5. `hours_per_day` — sati po danu (sortirano po `author_time`, S-007).
6. `day_stats` — redak po danu s commitom; `kind_stats` — vrste rada (ručni override po SHA pregazi klasifikator).
7. faze: zatvorene se **broje** iz cijelog loga, aktivne iz plana i filtriranih commita.
8. `indicators` — 18 pokazatelja, svaki s `kind` (`measure` ili `proxy`) i formulom.
9. `docs_health` — ocjena i nalazi; `evaluate_all(default_rules())` — signali.
10. `Report` se sastavi i serializira (`serde`).

**Grana:** metrike se računaju nad `profile.default_branch` ako ta grana postoji, inače nad trenutnom
granom (`io/src/project.rs::input`). Ostale grane ulaze **samo u signale**.

## 3 · Što `Report` nosi

`core/src/model.rs`. Polja: `generated_at` · `since` · `branch` · `touched` · `days` · `kinds` ·
`indicators` · `phases` · `visions` · `docs` (`null` kad projekt nema mapu s dokumentacijom — nula
bi bila laž) · `signals`.

**`touched` je mjerač mjerača** — koliko je izvještaj stvarno dotaknuo (`core/src/report.rs`):

| polje | što je | pažnja |
|---|---|---|
| `commits` | broj commita nakon filtra `since` | |
| `lines` | Σ `added + deleted` po svim izmjenama datoteka | |
| `files` | **broj izmjena datoteka kroz commite**, ne broj različitih datoteka | datoteka dirnuta u 10 commita doda 10 |
| `skipped_lines` | redaka `numstat`-a koje parser nije razumio | preskočeno se broji, nikad tiho ne ispari |

## 4 · Profil projekta — sva polja i zadane vrijednosti

**Izvor je `crates/sokratis-core/src/profile.rs`** (`impl Default for Profile`); ova tablica prati
njega. Zadane vrijednosti **jesu** konvencije Sokrat Studyja (S-005): prvi korisnik radi bez ijedne
postavke. Profil je `#[serde(default, deny_unknown_fields)]` — polje koje nedostaje uzima zadano,
polje s tipfelerom je greška, ne tiho ignoriranje. Ovo je jedina tablica profila u dokumentaciji.

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
| `phase_tag` | regex (blok §5) | oznaka faze u opisu commita |
| `classifier` | 4 pravila (blok §5) | uređena lista `(vrsta, regex)`; **redoslijed je ugovor** |
| `gate_pattern` | regex (blok §5) | podvrsta „brana i mjerenje" |
| `deploy_pattern` | regex (blok §5) | podvrsta „deploy" |
| `ci_fix_pattern` | regex (blok §5) | pokazatelj „CI-padova popravljenih" |
| `owner_name` | `"leon"` | isporuka se smatra vlasnikovom ako mu je ime u naslovu |
| `test_path_prefixes` | `["tests/"]` | testna putanja: počinje s… |
| `test_path_contains` | `["/check-"]` | …ili sadrži… |
| `test_path_suffixes` | `[".test.js", ".spec.js"]` | …ili se završava na |
| `code_exclude_prefixes` | `["docs/"]` | putanja koja se NE smatra kodom (počinje s) |
| `code_exclude_suffixes` | `[".md"]` | putanja koja se NE smatra kodom (završava na) |
| `session_gap_hours` | `2.0` | razmak manji od toga = neprekinut rad |
| `session_start_hours` | `0.5` | fiksni dodatak za prvi commit nove sesije |
| `closed_phases` | 4 faze Sokrat Studyja | `{name, from, to, tag_pattern, note}`; broje se iz commita |
| `include_unmerged` | `false` | **deklarirano, jezgra ga u 0.1.0 još ne čita** (vidi §11) |
| `unmerged_warn_days` | `5` | grana starija od toga → Warn |
| `unmerged_alert_days` | `10` | …starija od toga → Alert |
| `unmerged_alert_count` | `3` | …ili više od toliko takvih grana → Alert |
| `docs_lag_warn_days` | `2` | dnevnik toliko dana iza koda → Warn |
| `docs_lag_alert_days` | `5` | …toliko → Alert |
| `docs_weights` | `dead_link 5` · `not_indexed 3` · `multiple_plans 15` · `no_active_plan 10` · `diary_in_definition 5` · `lag 10` · `key_file_budget 5` | koliko koji nalaz odbija od 100 |

`Profile::log_since()` uzima **najraniji** datum od `since` i početaka zatvorenih faza — git log mora
dovući i commite starije od `since` da bi se zatvorene faze mogle prebrojati.

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
`--format=@@%h|%at|%ct|%ad|%cd|%s` + `--numstat` + `--date=format:%Y-%m-%d` + `--reverse`. Unix-vremena
(`%at`, `%ct`) služe za razmake, lokalni datumi za dan u tablici (`%ad`, autorov — paritet s
`RAD.xlsx`) i za filtar `since` (`%cd`, commitov — kao git). `--since` se **uvijek** šalje sa satom
`00:00:00` (S-011): bez sata git uzima trenutno doba dana, pa bi isti datum davao različit broj
commita ovisno o tome kad se izvještaj pokreće.

## 6 · `.sokratis/` — ručni podaci u repou (S-004)

Tri datoteke u korijenu repoa, sve tri neobavezne, sve tri putuju kroz git s projektom.
Čita ih `io/src/project.rs`. „Datoteka ne postoji" je jedina greška koja tiho pada na zadano; svaka
druga (mapa umjesto datoteke, nema dozvole, pokvaren JSON) se javlja s putanjom.

**`.sokratis/profile.json`** — bilo koji podskup polja iz §4. Primjer je profil kojim Sokratis mjeri
sam sebe (dogfooding): vlastiti plan nema cigle ni faze, a testovi mu žive u `crates/*/tests/`.

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
| `sokratis report [putanja] [--since YYYY-MM-DD] [--json\|--table]` | cijeli `Report`; **bez zastavice je JSON**, `--table` daje tablicu s hrvatskim natpisima | 0 |
| `sokratis docs [putanja] [--json]` | ocjena, broj nalaza, kašnjenje; bez `docs_dir` poruka `docs: nema mape s dokumentacijom (n/a)` | 0 |
| `sokratis signals [putanja] [--json]` | signali s dokazom, ili `nema signala` | **0** nema · **1** Warn · **2** Alert |

Svaka greška okoline (nema `git`-a na PATH-u, putanja nije repozitorij, pokvaren profil) ispisuje se
na stderr kao `sokratis: <poruka>` i daje **izlazni kod 3**. `process::exit` je u `main` i nigdje
drugdje: izlazni kod je ugovor prema preflightu, a ne nuspojava.

`--since` postoji samo na `report`; `docs` i `signals` uzimaju `since` iz profila.

## 10 · Dvije zamke koje je otkrio dogfooding

1. **„Testni redak" se prepoznaje SAMO po putanji** (`test_path_prefixes` · `test_path_contains` ·
   `test_path_suffixes`, `Profile::is_test_path`). Posljedice koje treba znati prije čitanja brojke
   „udio testnih redaka": inline `#[cfg(test)] mod tests` unutar `src/*.rs` se **ne broji** kao test
   (živi u istoj datoteci kao produkcijski kod), a **sve** pod testnom putanjom se broji — i fixture
   datoteke pod `tests/`, koje nisu kod testa. Sokratis zato u svom profilu ima
   `test_path_contains: ["/tests/"]`: njegovi testovi žive u `crates/*/tests/`, što zadani prefiks
   `tests/` ne hvata.
2. **`touched.files` je broj izmjena datoteka, ne broj različitih datoteka** (§3). Ista datoteka
   dirnuta u deset commita doda deset. Brojka odgovara na „koliko je izmjena pročitano", ne na
   „koliko datoteka projekt ima".

## 11 · Što stoji u kodu, a još ne izlazi (0.1.0)

Uredno zapisani propusti, ne skrivene rupe (CLAUDE.md #4):

- **`include_unmerged`** je polje profila, ali ga jezgra još ne čita: metrike su uvijek samo nad
  zadanom granom, a nespojene grane ulaze isključivo u signale.
- **`classify_sub`** (podvrsta commita: cigla · brana/mjerenje · deploy · ostalo) se računa i testira,
  ali ne ulazi u `Report` — čeka pogled Dnevnik u M2.
- **`GitSource::worktrees`** je implementiran i testiran, ali izvještaj ga ne koristi: identitet
  projekta preko više radnih stabala je posao M2.
- **SQLite snimke, watcher i popis projekata** su M2 (S-009); 0.1.0 sve računa na zahtjev i ne piše
  ništa osim onoga što korisnik sam stavi u `.sokratis/`.
