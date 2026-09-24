# ARHITEKTURA 1.0.0 — grane · ploča · izlaz (drugi rez, nakon Leonovih nalaza)

**Status:** 🟩 AKTIVAN SPEC — napisan 2026-09-24 iz brainstorminga s Leonom nad
[product/NALAZI_LEON_2026-09-23.md](../product/NALAZI_LEON_2026-09-23.md) (pitanje po pitanje, odluke
**S-032…S-037** u [records/DECISIONS.md](../records/DECISIONS.md)) · jedini aktivni spec u `plan/` ·
**preuzima etapu 3 „izdanje"** iz speca M2 (§13.1, §13.8), koji je istoga dana arhiviran:
[archive/ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md) · plan cigli:
[superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md](../superpowers/plans/2026-09-24-1-0-0-grane-i-ploca.md)
(T44–T64, napisan 2026-09-24 nakon Leonova pregleda speca; T64 dodan 2026-09-25, §1.4).

Što je STVARNO izgrađeno do ovog speca opisuje
[architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md) (verzija `1.0.0-pre.1` u `main`-u);
status milestonea [ROADMAP.md](./ROADMAP.md); što ostaje za kasnije
[records/BACKLOG.md](../records/BACKLOG.md).

> **Što ovaj dokument JEST:** precizan opseg **do verzije 1.0.0** nakon što je Leon instalirao
> „1.0.0-pre" i zaključio da je mjerilo krivo za njegov način rada (dugotrajne grane u radnim
> stablima) i da je sučelje premalo pregledno. Odjeljci: zašto · grane · dnevnik po stablima · sučelje
> · desktop · testovi · ovisnosti · redoslijed · izlazni uvjet · izvan opsega.
>
> **Što NIJE:** dom odluka (S-032…S-037 su u `DECISIONS.md`, ovdje su pointeri) ni opis izgrađenog
> (`ARCHITECTURE.md`). Ništa iz speca M2 se ovdje ne prepisuje — što M2 kaže o `store`-u, watcheru,
> snimkama, splashu, temama, i18n i kartici s objašnjenjem **i dalje vrijedi** i piše u arhivi i u
> `ARCHITECTURE.md`.
>
> **Odluke Leona 2026-09-24 (brainstorming):** metrike nad **svim lokalnim granama** (S-032) · dnevnik
> = **unija svih stabala**, plan i docs iz **vodećeg stabla** (S-033) · Pregled → klik na karticu →
> **nadzorna ploča projekta**; zasebni ostaju samo Dnevnik i Vizije (S-034) · grafovi = **d3-matematika
> + naš Svelte SVG**, 4 nove npm ovisnosti uz Leonov OK (S-035) · **X = upit → izlaz, tray se
> uklanja**, autostart otvara prozor (S-036) · **sve prije 1.0.0**, jedno izdanje (S-037).

---

## 0 · Zašto ovaj spec, i što mijenja u odlukama

Šest Leonovih nalaza i njihova sudbina:

| # | nalaz | odgovor | odluka |
|---|---|---|---|
| 1 | metrike vide samo `main` → njegov rad od 13. 9. je nevidljiv | mjerenje nad svim lokalnim granama, oznaka grane po commitu, dnevnik iz svih stabala | **S-032**, **S-033** (dopuna S-005: zadano više nije „samo zadana grana") |
| 2 | premalo grafova, bez datuma na osima, nepregledno | zajednički temelji (osi s datumima, mreža, legenda, tooltip) + 7 novih grafova + prekidač dan/tjedan/mjesec | **S-035** (revizija S-018: matematika iz d3, crtanje i dalje naše) |
| 3 | X mora zatvoriti uz upit; animacija pri svakom pokretanju | X = upit → izlaz; tray se uklanja; svako pokretanje je nov proces pa S-019 sam daje animaciju | **S-036** (ukida S-020 u dijelu „X sakriva, tray") |
| 4 | konzolni prozor `git.exe` pri svakom osvježenju (**kvar**) | jedan pomoćnik `git_command()` s `CREATE_NO_WINDOW` | §4.2 |
| 5 | „koliko god projekata, praćenje usporedno" | već pokriveno: Pregled = neograničene kartice, klik = ulaz u projekt | S-034 (zajednički graf usporedbe → BACKLOG) |
| 6 | klik na karticu → ploča s grafovima i svim brojkama | ploča s 8 sekcija; 6 prikaznih pogleda postaju sekcije | **S-034** (mijenja M2 §6.1/§6.2) |

**Nepromijenjeno:** S-002 (jezgra bez I/O-a) · S-003 (git kroz proces) · S-007 (sati po `author_time`)
· S-012 (`Report` je ugovor — mijenja se **namjerno**, kroz snapshot) · S-014 (keš po SHA-i, snimke po
danu) · S-015 (projekt = `git-common-dir`, ručni podaci u glavno stablo) · S-016 (watcher) · S-019
(splash) · S-026/S-027/S-028 (animacije · kartica s objašnjenjem · Postavke).

---

## 1 · Grane: što se mjeri (S-032)

### 1.1 Skup grana i oznaka po commitu — `io`

- `GitSource::log` i `rev_list` umjesto `branch: &str` primaju **`Scope`**:
  `Scope::Branch(ime)` (danas) ili `Scope::AllBranches` → `git log --branches …` (git svaki commit
  ispiše **jednom**, bez obzira u koliko je grana; remote-tracking reference i tagovi ne ulaze).
- **Oznaka grane po commitu** dolazi iz **jednog dodatnog procesa** (nova metoda
  `commit_sources(default_branch, since, until) -> HashMap<sha, grana>`):
  `git log --branches --not <zadana> --format=%h|%S <prozor>` — samo commiti **izvan** zadane grane,
  sa `%S` = kratko ime reference kojom ih je git dosegao (`feat/f6-mcp`; provjereno nad Sokrat
  Studyjem). Sve što nije u karti je zadana grana. Commit dostižan iz dviju nespojenih grana (dijete
  odvojeno od roditelja) dobiva **jednu** od njih — koju, presuđuje gitov obilazak; oznaka je „jedna
  od grana koje ga sadrže", ne vječna: nakon spajanja commit **prelazi pod zadanu granu**. Kartica s
  objašnjenjem to kaže.
- **Keš commita (`store`) se ne dira:** oznaka nije svojstvo SHA-e, pa **ne ide u keš** — zato
  odvojena karta, a ne `%S` u formatu loga (odbijeni pristup A2). `cached_log` prima `Scope` i
  prosljeđuje ga `rev_list`-u; `log_commits` (po SHA-i) ostaje bez promjene.
- Prozor dovlačenja: `window_args` isti kao danas; nad više vrhova gitov `--since` ne reže obilazak
  identično kao nad jednom granom, ali **jezgra presuđuje po `commit_date`**, pa brojka ne ovisi o
  tome. Mjerenje nad Sokrat Studyjem (cigla u planu) to potvrđuje: broj commita `--branches` u prozoru
  ≥ zbroj po granama koje jezgra izbroji.
- Karta koristi **isti `ref_name` kao danas** (zadana grana → trenutna → `HEAD`, `project.rs::input_with`),
  pa detached HEAD i repo bez zadane grane rade kao prije. `branches()` (signal „nespojene grane") se
  ne mijenja u ovom odjeljku (dopuna §1.4 mu dodaje vrh i sadržanost); jedino `last_changes` dobiva
  referencu — gleda povijest **vodećeg stabla** (§2).

### 1.2 Jezgra — model i mjere

```rust
// core/src/model.rs — samo dodaci, ništa se ne preimenuje
pub enum BranchScope { #[serde(rename = "all")] AllBranches, #[serde(rename = "default")] DefaultBranch }
struct ReportInput { …, scope: BranchScope, commit_branches: HashMap<String, String> }
struct CommitRow   { …, branch: String }                     // ime grane (zadana ako nije u karti)
struct Report      { …, scope: BranchScope, branches: Vec<BranchStats> }
struct BranchStats { name: String, commits: u32, lines: u64, hours: f64, merged: bool }
```

- `branches` se računa **iz redaka commita** (kao `kind_stats`), sortirano po `commits` silazno; red
  zadane grane je uvijek prvi ako ima commita. `merged` dolazi iz `BranchInfo` (`branches()`); zadana
  grana je `merged = true`.
- **Sati po grani = sati dana podijeljeni po udjelu commita grane tog dana**
  (`Σ_dan dan.hours × commiti_grane_tog_dana / commiti_dana`). Zbroj po granama = ukupno; sesije S-007
  se ne računaju dvaput. To je **proxy** i tako piše na kartici. Sesije po grani (zbroj može premašiti
  ukupno) odbijene.
- `Report.branch` i dalje = ime zadane grane (prema njoj „spojeno" / „ispred"). `Report.scope` kaže
  je li mjereno sve ili samo ona.
- Pravila (`rules/`) nepromijenjena: `unmerged_branches` gleda `BranchInfo` kao dosad.
- **Paritet s `RAD.xlsx` ostaje isti test:** fixture je tekst loga `main`-a, jezgra ne zna odakle je.
- Snapshot `Report`-a (S-022) se mijenja **namjerno** (`cargo insta review`), jednom, s obrazloženjem.

### 1.3 Profil i CLI

- `profile.json` += `branch_scope: "all" | "default"`, **zadano `"all"`** (dopuna S-005: prvi korisnik
  radi u granama). Polje je isti enum `BranchScope` kao u `Report` (serde `rename`: `"all"` /
  `"default"`), pa drugi tekst odbije serde s popisom dopuštenih vrijednosti, kao za `kind` u
  klasifikatoru — `IoError::Profile` nosi putanju, redak i stupac.
- CLI: `sokratis report <putanja> --scope all|default` pregazi profil (isti obrazac kao `--since`).
  Tablica dobiva redak „grana/opseg". `signals`/`docs` bez promjene.
- `branch_scope` ulazi u kanonski JSON profila koji dnevna snimka pamti (`engine.rs::try_snapshot`,
  S-014), pa trend pokazatelja **dobiva oznaku „profil promijenjen"** na dan nadogradnje — brojke
  prije i poslije nisu usporedive i graf to kaže sam, bez posebnog koda.
- Sokrat Study ima lokalne grane `origin/content/*` (materijali, ne kod); u rasponu od 29. 8. daju ≤ 2
  commita — prihvaćeno, bez polja za isključivanje (YAGNI; ako zatreba: `git log --exclude=<glob>
  --branches` je jedan argument).

### 1.4 Nespojene grane su lanci (S-038, dopuna 2026-09-25 iz vanjske analize)

- **Nalaz:** nad Sokrat Studyjem pravilo `unmerged_branches` prijavljuje 8 prekršitelja i **Alert**
  (prag `unmerged_alert_count = 3`), a pet ih je sadržano u `feat/f6-mcp` (Leon svaku sesiju grana od
  prethodne), jedna je 1 commit odvojena; stvarno odvojene su dvije. Izlazni kod 2 je ugovor za
  pre-flight skripte — lažni Alert zaustavlja tuđi proces. Fixture pravila gradi samo nezavisne grane,
  pa je kvar bio nevidljiv testovima.
- **Model:** `BranchInfo` += `tip` (puni SHA vrha, iz istog `for-each-ref`) i `contained_in:
  Option<String>` (ime **vrha lanca** koji granu sadrži; `None` = grana je vrh). `ReportInput` +=
  `branch_graph: String` — tekst `git log --branches --not <zadana> --format=%H|%P` (bez prozora;
  **jedan** dodatni proces, i to samo kad postoji nespojena grana — Sokratis sam ne plaća ništa).
  Jezgra (`core/src/chains.rs`, čisto) iz tog teksta računa sadržanost: X je sadržana u Y ako je vrh
  X predak vrha Y unutar nespojenih commita; vrh se bira deterministički (više commita ispred, pa
  manje ime; dvije grane na istom commitu → manje ime). **`Report` se ne mijenja** — `BranchInfo` je
  ulaz pravila, ne izlaz.
- **Pravilo:** `offenders` = **vrhovi** stariji od `unmerged_warn_days`; sadržane grane nikad ne ulaze
  u brojanje ni u prag; dokaz po vrhu nosi sadržane (`+5 grana unutar: …`). Vrh mlađi od praga
  utišava lanac — rad je živ. Perf granica 8 ostaje (izmjereno 7 + 1). Cigla **T64** u planu.
- **Nusprodukti (BACKLOG, ne 1.0.0):** signal „živa grana predugo izvan zadane" (starost od točke
  grananja, ne od zadnjeg commita — `feat/f6-mcp` je 108 commita ispred tjednima) · pravilo docs-a
  „citirana brojka/verzija u `.md` slaže se s izvorom istine" (uhvatilo bi `README.md` s `pre.1`).

---

## 2 · Dnevnik, plan i `docs/` po stablima (S-033)

- **Dnevnik = unija.** `io` čita `diary_path` iz **svakog radnog stabla** na disku
  (`GitSource::worktrees`, postoji od M1) → `ReportInput.diaries: Vec<String>` (zamjenjuje
  `diary: Option<String>`; prazan vektor = nema dnevnika). Jezgra parsira svaki tekst istim
  `parse_diary` i **unira isporuke po `(date, title)`**, sortirano po datumu; isti naslov s drukčijim
  `model`/`deploy` uzima **najnoviji** — `io` predaje dnevnike **redom od vodećeg stabla** (najnoviji
  HEAD, niže) prema starijima, pa isporuka uređena u grani u kojoj se radi pobjeđuje onu s `main`-a. Necommitani
  unos u dnevniku je vidljiv (čita se disk, ne git-objekt). Isporuka čiji je naslov uređen u drugoj
  grani pojavi se dvaput — poznato, dokumentirano na kartici.
- **Vodeće stablo** = radno stablo čiji HEAD ima **najnoviji `author_time`**. `io`:
  `worktree list --porcelain` (putanja · HEAD · grana po stablu, nova metoda `worktree_heads()`) +
  jedan `git log --no-walk --format=%H|%at <sha…>` za vremena → iz vodećeg stabla se čitaju
  `plan_path`, `docs/` (ocjena, kašnjenje, nalazi) i `last_changes` (povijest te grane). Glavno stablo
  (uz `.git`) ostaje mjesto **pisanja** ručnih podataka (S-015) — čitanje i pisanje su namjerno
  razdvojeni.
- `Touched` += `worktrees: u32`, `diaries: u32` — mjerač kaže koliko je dotaknuo.
- Watcher (S-016) već nadzire dnevnik/plan/`docs/` u **svakom** stablu — bez promjene.
- Cijena: **+2 git-procesa** po izvještaju (danas ≈ 5 nad Sokrat Studyjem); `tests/perf.rs` dobiva
  novu gornju granicu, izmjerenu.

---

## 3 · Sučelje (S-034, S-035)

### 3.1 Tok — Pregled je jedini ulaz

- **Pregled** = mreža kartica, neograničen broj projekata (registar to već zna). Kartica: ime · stabla ·
  zadnji commit („prije 2 h") · najteži signal · brojevi po težini · greška ako mapa ne postoji ·
  „Dodaj projekt". **Klik na karticu → pogled Projekt** tog projekta.
- **Birač projekta u gornjoj traci se uklanja.** Gornja traka: lockup · „‹ Pregled" + ime projekta (u
  projektu) · birač raspona · Osvježi. Lijevi izbornik: **Pregled · Projekt · Dnevnik · Vizije ·
  Postavke**; Projekt/Dnevnik/Vizije su zasivljeni dok korisnik nije ušao u projekt. `app.view` dobiva
  `'project'`, gubi `'tempo' | 'kinds' | 'indicators' | 'phases' | 'deliveries' | 'docs'`.
- Stanje ostaje kako jest (`currentId`, raspon po projektu, `epoch`, `report_updated` pretplata).

### 3.2 Ploča — `views/Project.svelte`, osam sekcija

Ljepljiv skok-izbornik na vrhu (sidra po sekciji). Sadržaj šest bivših pogleda seli **bez gubitka**
(tablice, oznake mjera/proxy, nalazi `datoteka:redak`, kopiranje putanje).

| # | sekcija | grafovi (▲ = nov) | izvor u `Report`/naredbi | `explain` id |
|---|---|---|---|---|
| 1 | Sažetak | ključne brojke (commiti · sati · dani · isporuke · docs-ocjena) + traka signala | `days`, `deliveries`, `docs`, `signals` | postojeći |
| 2 | Tempo | stupci commiti/sati s **prekidačem dan/tjedan/mjesec** · kumulativna linija · ▲ **kalendarska toplinska karta** · ▲ **doba dana** | `days` (`bucket.ts`), `commits[].author_time` | `tempo.bars`, `tempo.cumulative`, ▲`tempo.heatmap`, ▲`tempo.hours_of_day` |
| 3 | Grane | ▲ **vodoravni stupci** commiti (i sati) po grani, spojene označene | `branches` | ▲`branches.bars` |
| 4 | Vrste rada | prsten · ▲ **naslagani stupci po tjednu** + tablica s postocima | `kinds`, `commits[].date+kind` (`bucket.ts`) | `kinds.ring`, ▲`kinds.over_time` |
| 5 | Faze | ▲ **Gantt** (traka po fazi `from`→`to`, stanje bojom, „danas" kao crta) + tablica | `phases` | ▲`phases.gantt` + postojeći |
| 6 | Isporuke | ▲ **stupci po tjednu**, deploy označen + tablica | `deliveries` (`bucket.ts`) | ▲`deliveries.weekly` + postojeći |
| 7 | Pokazatelji | 18 kartica sa sparklineom (kao danas) | `indicators`, `get_trend` | `ind.*` |
| 8 | Dokumentacija | ocjena · kašnjenje · ▲ **trend ocjene i signala** · nalazi | `docs`, `get_trend(docs_score, signals_warn, signals_alert)` | `docs.*`, ▲`docs.trend` |

**7 novih `explain` id-eva**, svaki s `what|how|read` u oba jezika (S-027); test pokrivenosti raste
sam. Sekcija **Grane** se prikazuje uvijek — s jednom granom (ili `scope = default`) ima jedan stupac
i natpis „mjerena samo zadana grana", da se vidi *zašto* je jedan. **Doba dana** računa sat iz `author_time` u **zoni ovog računala** — jezgra ne nosi sat autora, a
proširenje formata loga bi zastarjelo keš; kartica to kaže. **Trend** postoji tek od dana kad je
projekt dodan u Sokratis (dnevne snimke, S-014) — kartica to kaže.

### 3.3 Grafovi — d3-matematika + naš SVG (S-035, revizija S-018)

Težina grafova nije u pravokutnicima nego u **matematici osi** (datumski ticksi 7 dana → godina,
`nice()` skale, stack, binovi, lukovi). Ta matematika dolazi iz **d3-jezgrenih modula bez DOM-a**
(§6); Svelte i dalje posjeduje DOM: boje **isključivo** `var(--color-*)`, animacije kroz
`data-motion` (S-026), tooltip i legenda naši.

`lib/charts/` — **temelji** (dijele ih svi grafovi):

| datoteka | uloga |
|---|---|
| `scales.ts` | omotači nad `d3-scale` (`scaleTime`, `scaleLinear`, `scaleBand`), ticksi i oblikovanje datuma po jeziku (`d3-time-format` `timeFormatLocale`, HR/EN) — **čiste funkcije, vitest** |
| `bucket.ts` | dan/tjedan/mjesec nad `DayStats`, isporukama i redcima commita (ISO tjedan, mjesec) — preslagivanje već izmjerenih dana, **ne mjerenje** (dopuna S-012); vitest s rubovima (prazan raspon, prijelaz godine) |
| `Chart.svelte` | okvir: margine, responsivan `viewBox`, naslov, `aria-label`, prazno stanje |
| `Axis.svelte` | X (vrijeme · kategorije) i Y (brojke), ticksi iz `scales.ts`, oznake ne preklapaju (prorjeđivanje po širini) |
| `Grid.svelte` · `Legend.svelte` · `Tooltip.svelte` | mreža po Y-ticksima · legenda iz nizova (boja + natpis) · jedan tooltip po grafu, pokazivač → najbliža točka, tipkovnicom dostupan |

**Komponente** na temeljima (8): `Bars` (grupirani/naslagani, X = datumi ili kategorije) · `Line`
(više nizova) · `Ring` · `Sparkline` · `Heatmap` (kalendar) · `Gantt` · `HBars` · `Histogram`.
Postojeće četiri se **prepisuju na temelje** (isti props gdje je moguće, plus osi/legenda). Sučelje
nema DOM u vitestu (bez `jsdom`, S-018 obrazac: „komponente se ne testiraju renderiranjem"), pa svaki
graf dijeli posao na **čistu funkciju rasporeda** (`layout.ts`: iz podataka i okvira u koordinate,
ticksove i natpise — vitest) i **tanku komponentu** koja te koordinate samo iscrta; datumi na osima
se dokazuju testom rasporeda, a izgled dimnim testom (CDP snimka orkestratora).

### 3.4 Dijalog izlaza — `components/ConfirmQuit.svelte`

Modalni dijalog u temi (HR/EN), fokus u dijalogu, `Esc` = Odustani; test vitestom (otvori, potvrdi →
`api.quit()` pozvan; odustani → nije). Tekst: §4.1.

---

## 4 · Desktop (`apps/desktop/src-tauri`)

### 4.1 X = upit → izlaz; tray se uklanja; autostart otvara prozor (S-036)

- `lib.rs`: `CloseRequested` → `api.prevent_close()` → `emit("close_requested")` → sučelje pokazuje
  `ConfirmQuit` → naredba **`quit`** → `app.exit(0)`. Tekst upita: **„Zatvoriti Sokratis? Nadzor
  projekata i obavijesti staju dok ga ponovno ne pokreneš."** — gumbi **[Zatvori] [Odustani]**.
- `tray.rs` se **briše**; `set_autostart` seli u `autostart.rs`; Tauri feature `tray-icon` i
  tray-ikone van iz `Cargo.toml`/`tauri.conf.json`. `tauri-plugin-single-instance` ostaje (druga
  instanca fokusira prvu). Plugin `notification` ostaje: obavijest na prijelaz u Alert dok je prozor
  otvoren (motor radi dok proces živi).
- Splash (S-019) nepromijenjen — svako pokretanje je nov proces, pa i animacija.
- Autostart = pokreni **s prozorom** pri prijavi; natpis u Postavkama: „Sokratis se otvara pri prijavi."
- Naredbe: +`quit`; događaj +`close_requested`. Ostalo (jedanaest naredbi, `Range`, `Settings`) bez
  promjene — `get_report` vraća prošireni `Report` (S-012).

### 4.2 Konzolni prozor `git.exe` — kvar

`git.rs` dobiva **jedan** pomoćnik `git_command(repo) -> Command` (jedino mjesto `Command::new("git")`;
danas su dva: `run` i `run_with_stdin`) koji na Windowsu postavlja
`std::os::windows::process::CommandExt::creation_flags(CREATE_NO_WINDOW = 0x0800_0000)`. Instalirana
aplikacija je `windows_subsystem = "windows"`, pa bez zastavice svaki proces bljesne konzolu.
**Test:** zastavica se iz `Command`-a ne može pročitati natrag — test čita izvor (`include_str!`) i
tvrdi da se `Command::new` pojavljuje **samo unutar** `git_command`; pošteno ograničenje, zapisano u
zaglavlju. Pravilo u `RUST.md`: svaki budući `Command` u `io`/desktopu ide kroz pomoćnika.

---

## 5 · Testovi — test-prvo (pravilo #7)

| što | fixture → očekivano | gdje |
|---|---|---|
| oznaka grane po commitu, `BranchStats`, sati po udjelu | tekst loga s 3 grane + karta `sha→grana` → redci s `branch`, 3 reda `branches`, zbroj sati = ukupno | `core/tests/branches.rs` |
| `scope = DefaultBranch` s praznom kartom | isti log → svi redci = zadana grana, jedan red `branches` | isto |
| unija dnevnika | dva dnevnika s 2 preklopljene isporuke → 5 isporuka, prvi viđeni pobjeđuje | `core/tests/diary_union.rs` |
| paritet `RAD.xlsx` | **nepromijenjen** (fixture `main`-a) | `core/tests/parity.rs` |
| snapshot `Report`-a | jedna namjerna promjena (`scope`, `branches`, `branch` u redcima, `touched`) | `insta` |
| `Scope` → argumenti gita; `commit_sources` parse | lažni `GitSource` bilježi argumente; tekst `%h\|%S` → karta | `io/src/git.rs` (unit) |
| vodeće stablo | tri stabla s HEAD-ovima → najnoviji `author_time` pobjeđuje; jedno stablo → ono | `io/src/project.rs` (unit, lažni git) |
| `git_command` jedino mjesto `Command::new` | izvor `git.rs` | `io/src/git.rs` (unit) |
| profil `branch_scope` | `"all"`/`"default"` OK, `"x"` → greška s imenom polja; zadano `"all"` | `core/src/profile.rs` |
| CLI `--scope` | tablica/JSON nose opseg; nad pravim repoom (temp git s 2 grane) commiti obiju grana | `cli/tests/` |
| performanse | Sokrat Study: broj git-procesa po izvještaju ≤ 8; trajanje izmjereno u izvještaju cigle | `io/tests/perf.rs` |
| lanci grana (§1.4, T64) | fixture LANCA `a ⊂ b ⊂ c ⊂ d` + odvojena `e` → 2 vrha, **Warn ne Alert**, dokaz `d` nabraja `a, b, c`; tekst `%H\|%P` → `contained_in`; pravi repo s lancem od tri grane | `core/src/rules/unmerged_branches.rs`, `core/src/chains.rs`, `io/tests/git_cli.rs` |
| CLI zaglavlje imenuje mjereni doseg (T51, dopuna) | `Report { scope: all, 2 grane }` → zaglavlje sadrži `opseg: sve lokalne grane`, ne `grana main · N commita` | `cli/src/table.rs` (unit) |
| `bucket.ts` | 10 dana preko prijelaza godine → tjedni ISO, mjeseci; prazno → prazno | vitest |
| `scales.ts` | raspon 7 dana → dnevni ticksi, 6 mjeseci → mjesečni; HR nazivi mjeseci | vitest |
| `layout.ts` (raspored grafova) | svaki graf: iz podataka + okvira → koordinate, ticksovi s datumima, legenda; prazan ulaz → prazan raspored bez NaN | vitest |
| ploča | 8 sekcija u DOM-u, skok-izbornik, svaki `explain` id postoji u oba rječnika (postojeći test raste sam) | vitest |
| tok | klik na karticu → `app.view === 'project'`; izbornik zasivljen bez projekta | vitest |
| `ConfirmQuit` | potvrdi → `quit`; odustani/Esc → ne | vitest |
| dimni test | X → upit → izlaz procesa; ponovno pokretanje → animacija; osvježenje bez konzole; Sokrat Study pokazuje commite od 23. 9. | ručno, `TESTING.md` §5 |

Brane ostaju: `cargo fmt --check` · `clippy --workspace --all-targets -D warnings` · `cargo test` ·
`npm run check` (svelte-check · i18n · kontrast · vitest) · `npm run build` · `sokratis docs .` 100/100
· `signals .` nema.

---

## 6 · Ovisnosti — sve nabrojane, sve pinane (pravilo #6)

**npm, 4 izravne (Leonov OK 2026-09-24, S-035):** `d3-scale@4.0.2` · `d3-shape@3.2.0` ·
`d3-array@3.2.4` · `d3-time-format@4.1.0` (svi ISC, bez DOM-a; tranzitivno `d3-time`, `d3-format`,
`d3-interpolate`, `d3-color`, `d3-path`, `internmap` ≈ 10 paketa) + `@types/*` za njih kao dev.
Odbijeni: LayerChart 2.5 (30+ paketa, `-next` pre-release ovisnosti) · Observable Plot (cijeli d3) ·
ECharts 6 (canvas, vlastiti izgled, teme kroz runtime, ≈1 MB) · čisti vlastiti SVG (datumske osi bismo
pisali i debugirali sami).
**Rust:** nijedna nova (`creation_flags` je `std`). **Uklanja se:** Tauri feature `tray-icon`.

---

## 7 · Redoslijed — vrijednost prvo, instalater nakon svake sesije

Sesije su kratke (Leonov usage); zato **nakon svake sesije** orkestrator gradi instalater
„1.0.0-pre.N" (S-029) da Leon vidi gotovo odmah, iako je izdanje jedno (S-037).

| tok | cigle (okvirno; točno u planu) | Leon vidi nakon |
|---|---|---|
| **DESKTOP-2** | konzola (§4.2) · X/tray/autostart (§4.1, §3.4) | 1. sesije: bez bljeska konzole, X pita i izlazi, animacija svaki put |
| **JEZGRA-2** | model + `BranchStats` + sati po udjelu · unija dnevnika · profil `branch_scope` · snapshot | — (jezgra) |
| **IO-2** | `Scope` + `commit_sources` · `worktree_heads` + vodeće stablo + `diaries` · CLI `--scope` · mjerenje nad Sokrat Studyjem | 2. sesije: **njegov rad od 13. 9. je u brojkama** (Dnevnik s granom, Isporuke svježe) |
| **GRAFOVI** | ovisnosti + `scales.ts`/`bucket.ts` · temelji · prepis četiri postojeća · `Heatmap`+`Histogram` · `Gantt`+`HBars`+naslagani `Bars` | 3. sesije: datumi na osima, legenda, tooltip |
| **PLOČA** | tok Pregled→Projekt + izbornik + traka · sekcije 1–4 · sekcije 5–8 · 7 `explain` + i18n · dimni test | 4. sesije: ploča |
| **IZDANJE** | T35 (verzija 1.0.0, mjerenja — napola u `sokratis.rel`, ostaje parkirano do kraja) · završna recenzija (opus) · jedan krug popravaka · T43 čuvar (M2 §13.8 preuzet ovdje) · instalater 1.0.0 · **STANI**, tag uz Leonov OK | 5. sesije: 1.0.0 |

Ovisnosti: PLOČA traži GRAFOVE i IO-2 (sekcija Grane); DESKTOP-2 i JEZGRA-2 mogu usporedno; IO-2
nakon JEZGRE-2 (tipovi). Vlasništvo datoteka po toku piše plan.

---

## 8 · Izlazni uvjet 1.0.0 — gotovo kad

1. `sokratis report sokratstudy.dev --json` u zadanom profilu vrati commite **svih lokalnih grana** s
   `branch` po retku i `branches` po grani; `--scope default` vrati današnji rezultat; paritet zelen.
2. Isporuke u Sokrat Studyju uključuju unose iz dnevnika radnih stabala; ocjena docs-a iz vodećeg
   stabla; `touched.worktrees` = broj stabala.
3. Instalirana aplikacija: nijedan bljesak konzole pri osvježenju; X → upit → proces izlazi; svako
   pokretanje = animacija; tray-ikone nema; autostart otvara prozor.
4. Pregled → klik na karticu → ploča s **8 sekcija** i **11 grafova** (4 prepisana + 7 novih) s
   datumima na osima, mrežom, legendom i tooltipom; prekidač dan/tjedan/mjesec; svaki graf ima karticu s
   objašnjenjem; sve četiri teme i `data-motion="off"` rade.
5. Izbornik: Pregled · Projekt · Dnevnik · Vizije · Postavke; birača projekta u traci nema.
6. Sve brane iz §5 zelene; `tests/perf.rs` s novom gornjom granicom; `ARCHITECTURE.md` opisuje novo
   stanje tvrdnju po tvrdnju; ovaj spec i plan u `archive/`; `CLAUDE.md` „Stanje" u desetak redaka;
   README (EN) opisuje 1.0.0. Tag `v1.0.0` **tek uz Leonov izričit OK**.

---

## 9 · Izvan opsega (ostaje u `BACKLOG.md`)

Zajednički graf usporedbe projekata na Pregledu (linija po projektu) · isključivanje grana po uzorku
(`branch_ignore`) · sat autora u formatu loga (doba dana u zoni autora) · zoom/brush na grafovima ·
otvaranje nalaza u editoru · sve iz M2 §12 i Leonovih v2 želja (kalibracija sati, tjedni izvještaj,
čarobnjak profila, izvoz, streak, „Danas", prečaci, iOS-glossy izgled — grafovi su sad naš SVG, pa je
put otvoren).
