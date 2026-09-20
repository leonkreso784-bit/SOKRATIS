# ARHITEKTURA + MILESTONE 2 — desktop Sokratisa

**Status:** 🟩 AKTIVAN SPEC — napisan 2026-09-18 iz brainstorminga s Leonom i **isti dan odobren**
(„Imaš moj OK", bez izmjena) · plan cigli iz njega:
[superpowers/plans/2026-09-18-m2-desktop.md](../superpowers/plans/2026-09-18-m2-desktop.md) ·
jedini aktivni spec u `plan/`

Što je od M1 stvarno izgrađeno opisuje [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md);
ispunjeni spec M1 je [archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md); status milestonea
[ROADMAP.md](./ROADMAP.md); dug koji M2 nasljeđuje [records/BACKLOG.md](../records/BACKLOG.md).

> **Što ovaj dokument JEST:** precizan opseg Milestonea 2 — desktop ljuska nad jezgrom iz M1 — iz
> kojeg se piše plan implementacije (`docs/superpowers/plans/`). Odjeljci su isti kao u specu M1:
> slika cjeline · model · formati · tok podataka · testovi · izlazni uvjet.
>
> **Što NIJE:** nije dom za odluke (S-012…S-022 su u [DECISIONS.md](../records/DECISIONS.md)) ni za
> definiciju proizvoda ([PRD.md](../product/PRD.md) §3 red „Desktop [M2]", §4 red M2). Ovdje su
> pointeri, ne kopije.
>
> **Odluke Leona 2026-09-18 (brainstorming, pitanje po pitanje):** aplikacija je **stalno otvorena,
> za proučavanje** · prvi ekran = **Pregled svih projekata sa signalima** · **svih 8 pogleda** s
> uređivanjem · SQLite = registar + snimke + keš · **watcher** na `.git` + gumb · tray minimizira,
> **autostart**, obavijest samo na prijelaz u Alert · projekt se dodaje odabirom mape, **stabla se
> grupiraju sama** · grafovi **vlastiti SVG** · sve **četiri teme**, `brand-*` iz loga i izmjeren ·
> birač raspona, zadano cijeli projekt · **HR/EN prekidač već u M2** · dug M1: **7 od 9** · ručni
> podaci se pišu u **glavno stablo** · animacija **jednom po pokretanju, prozor čeka**, preskočiva.

---

## 1 · Slika cjeline

```
sokratis/
  Cargo.toml                 # workspace: + crates/sokratis-store + apps/desktop/src-tauri
  crates/
    sokratis-core/           # NEPROMIJENJEN UGOVOR Report (S-002) + mjere koje su mjere, ne prikaz (§2)
    sokratis-io/             # git · profil · .sokratis · NOVO: pisanje ručnih podataka · watcher (§3)
    sokratis-store/          # NOV: SQLite — registar · postavke · snimke brojki · keš po SHA (§4)
    sokratis-cli/            # nepromijenjen + `--until` (da se svaka brojka s ekrana provjeri iz terminala)
  apps/
    desktop/
      src-tauri/             # NOV crate sokratis-desktop: naredbe · prozori · splash · tray — BEZ logike (§5)
      src/                   # Svelte 5 + Tailwind v4 (Vite plugin) + tokens.css · i18n · SVG grafovi (§6)
```

**Tok podataka (isti `Report` za CLI i desktop):**

```
watcher (.git / docs / .sokratis)   ili   gumb Osvježi   ili   ulazak u projekt   ili   birač raspona
  → io:      Project::open (glavno stablo) → profil → Project::input(range)
  → core:    build_report                                        nepromijenjeno (S-002)
  → store:   upsert snimke za danas                              samo prvi izračun tog dana (§4.3)
  → desktop: emit report_updated { project_id }
             usporedba signala s prethodnim izračunom → nov Alert? → emit signal_raised → obavijest OS-a
  → Svelte:  Report → pogledi (oblikovanje, ništa više)
```

**Granice koje se ne prelaze:**

- **`core` i dalje ne otvara datoteke i ne zove procese** (S-002). Nepromijenjeno.
- **`store` ovisi o `core` (tipovi), ne o `io`** — testira se nad `:memory:` bazom, bez gita.
- **`desktop` ne sadrži logiku** (S-013): naredbe su tanki omotači nad `io`/`store`/`core`. Sve što
  bi se htjelo testirati živi ispod njega.
- **Tauri naredbe vraćaju `Report` nepromijenjen** (S-012) — *nepromijenjen* znači **isti oblik koji
  CLI ispisuje**, bez preslagivanja u desktop crateu; dodaci iz §2 (`until`, `vision_totals`,
  `commits`) mijenjaju struct, ali oba potrošača odjednom. Sučelje i `sokratis report --json` čitaju
  isti JSON; kad se raziđu, zna se tko laže, a snapshot-test (§9) to hvata prije Leona.

### 1.1 Identitet projekta, radna stabla, gdje se piše

- **Projekt = zajednički git-direktorij** (`git rev-parse --path-format=absolute --git-common-dir`).
  Leon pokaže na *bilo koju* mapu — korijen ili radno stablo — Sokratis nađe zajednički repo i sva
  stabla (`GitSource::worktrees`, napisan u M1, prvi put korišten ovdje). Sokrat Study =
  `sokratstudy.dev` + `.f21 .f22 .f25 .f3` = **jedan** projekt s pet stabala.
- **Ime** se predlaže iz mape glavnog stabla, Leon ga može preimenovati (registar, §4).
- **Dodavanje stabla projekta koji već postoji** vraća poruku „ovo je stablo projekta ‹X›", ne duplikat.
- **Ručni podaci se pišu u glavno stablo** (S-015): uvijek `<glavno stablo>/.sokratis/overrides.json`
  i `visions.json`, neovisno o tome koje je stablo dodano. Upis je necommitana izmjena vidljiva u
  `git status`; **Sokratis nikad ne commita sam.**
- **Metrike se računaju nad zadanom granom** iz profila (paritet, nepromijenjeno iz M1); ostale grane
  ulaze samo u signale. `include_unmerged` ostaje rezervirano polje (BACKLOG).

---

## 2 · Jezgra (`sokratis-core`) — što se dodaje, i zašto baš ovdje

Pravilo: **mjerenje ide u `core`, oblikovanje u Svelte.** Agregacija koju treba test je mjerenje;
postotak s dvije decimale je oblikovanje.

| dodatak | u `Report`? | zašto u `core` | dug |
|---|---|---|---|
| `until` u `ReportInput` — filtar `commit_date <= until`, cijeli dan (kao `since`, S-011) | `until: Option<String>` | birač raspona traži gornju granicu; CLI dobiva `--until` da svaku brojku s ekrana Leon može ponoviti iz terminala | — |
| **zbroj vizija po stanju** | `vision_totals: Vec<{ state, count }>` | agregacija; spec M1 §2.3 ju je tražio, plan joj nije dao ciglu | I6 |
| **`Patterns.phase_tag` se počinje čitati** — aktivne faze se na commite vežu regexom iz profila, ne tvrdim prefiksom `"{id}/"` | oblik nepromijenjen, `from`/`days`/`commits` prestaju biti prazni bez poruke | polje i `classify::phase_tag()` prestaju biti mrtvi; Sokratisov vlastiti profil već nosi `"^(M\\d/\\d+)"` | I3 + M14 |
| **`inside_root(rel)`** — svaka putanja iz profila (`docs_dir`, `diary_path`, `plan_path`, …) mora ostati unutar korijena repoa; `..` i apsolutna putanja → `ParseError::PathOutsideRoot { field }` | — (greška, izlaz 3 / poruka u sučelju) | „Dodaj projekt" pokazuje na bilo koju mapu; alat koji čita izvan repoa koji mjeri nije svojstvo koje se objavljuje | I9 |
| **`SnapshotMetrics`** — iz `Report`-a izvedena tanka struktura: 18 pokazatelja · `docs.score` · broj signala po težini; i `diff(prev, cur)` | ne; ide u `store` | to je ono što snimka drži (§4) i što trend crta; usporedba dviju struktura je mjerenje, ne prikaz | — |
| **testni redak bez fixturea:** `is_test_path` dobiva i `test_path_exclude` (zadano `["fixtures/"]`) | oblik nepromijenjen, brojka „udio testnih redaka" prestaje varati | Sokratisov vlastiti udio danas je većinom 767 kB fixture-snimka | odg. 8 |

**Redoslijed je bitan:** snapshot `Report`-a (S-022, §9) je **prva cigla M2** i nastaje **prije** ovih
dodataka; svaki dodatak koji mijenja oblik (`until`, `vision_totals`) mijenja snapshot **namjerno**
(`cargo insta review`), s obrazloženjem u commitu. Tako nijedna promjena ugovora ne prolazi neopaženo.

**Model — što je novo (samo dodaci, ništa se ne preimenuje):**

```rust
// core
struct ReportInput { …, since: String, until: Option<String> }         // until = YYYY-MM-DD, cijeli dan
struct Report      { …, vision_totals: Vec<VisionTotal> }               // uz postojeći visions
struct VisionTotal { state: String, count: u32 }
struct SnapshotMetrics { indicators: Vec<(String, f64)>, docs_score: Option<u8>,
                         signals: SignalCounts /* info, warn, alert */ }
enum ParseError    { …, PathOutsideRoot { field: String, value: String } }
```

Identifikatori ostaju engleski i stabilni (S-008). Ono što se u M1 računalo a nije izlazilo
(`classify_sub` podvrsta commita) **izlazi u M2** kroz pogled Dnevnik: `DayStats` ne mijenja oblik,
ali `Report` dobiva `commits: Vec<CommitRow { sha, date, subject, kind, sub, overridden: bool }>` —
to je ono što Dnevnik prikazuje i gdje se vrsta mijenja u mjestu. (Nije nova mjera; jest nov izlaz,
pa i on mijenja snapshot namjerno.)

---

## 3 · I/O sloj (`sokratis-io`) — što se dodaje

### 3.1 Pisanje ručnih podataka (do sada je `io` samo čitao)

`Project::write_override(sha, Option<WorkKind>)` (None = ukloni) · `Project::write_visions(Vec<Vision>)`.
Uvijek u glavno stablo (§1.1). **Atomarno:** piše se privremena datoteka pa `rename` — pola JSON-a na
disku nakon pada nije prihvatljivo. Oblik je isti kakav Leon piše rukom (pretty JSON, 2 razmaka), da
`git diff` pokaže jedan redak, ne cijelu datoteku.

### 3.2 Performanse — M11, jer ih watcher plaća u petlji

Danas: ~90 `git` procesa po izvještaju nad Sokrat Studyjem, 3,2 s (`ARCHITECTURE.md` §11). Cilj:
**tri poziva** umjesto devedeset.

| danas | u M2 |
|---|---|
| `git log -1 --format=%at -- <put>` za **svaku** `.md` datoteku (56) | **jedan** `git log --name-only --format=@@%at` nad `docs/`; prva pojava putanje = zadnja promjena |
| `git rev-list --count main..<grana>` za **svaku** granu (31) + `git branch --merged` | **jedan** `git for-each-ref --format='%(refname:short) %(committerdate:unix) %(ahead-behind:main)'` |
| jezgra klasificira svaki commit više puta (dan, vrsta, faze, pravila) | klasifikacija **jednom** po izvještaju, rezultat se nosi uz `Commit` |

**Mjerenje, ne tvrdnja:** io-test nad privremenim repoom sa 60 `.md` datoteka i 30 grana broji
pozive procesa (`GitSource` dobiva brojač u testnom omotaču) i tvrdi `≤ 5`; ručno mjerenje nad
Sokrat Studyjem (`report` s toplim kešom **ispod 500 ms**) zapisuje se u izvještaj cigle jer CI nema
Sokrat Study. Pravilo #4: **prvo mjerenje s brojkom, onda popravak** — cigla M11 počinje testom koji
danas pada na 90.

### 3.3 Watcher (S-016)

Jedna struktura `Watcher` u `io` nad `notify` crateom; **ne zna za Tauri** — javlja kroz
`std::sync::mpsc` kanal `WatchEvent { project_id, reason: Git | Docs | Manual }`, `desktop` ga
prevodi u Tauri događaj. Zato se testira bez prozora.

| što nadzire (po projektu) | zašto |
|---|---|
| `<git_common_dir>/HEAD` · `refs/` · `logs/HEAD` · `packed-refs` | commit, prebacivanje grane, spajanje — **zajedničko svim stablima** |
| u svakom stablu: dnevnik · plan · `docs/` | docs-ocjena i isporuke se mijenjaju i bez commita |
| u glavnom stablu: `.sokratis/` | ručni podaci, i kad ih Leon promijeni izvan Sokratisa |

Tri zaštite, svaka sa svojim testom:

1. **Odgoda 600 ms i sažimanje rafala.** `git commit` dira `HEAD`, `logs/HEAD` i `refs/` — bez odgode
   je to tri izračuna za jedan commit. Test: tri promjene unutar 100 ms = **jedan** događaj.
2. **Vlastiti upisi se ne vraćaju kao događaj.** Kad `io` sam piše `overrides.json`, ta putanja je
   2 s potisnuta. Test: `write_override` = **nula** događaja.
3. **Izračun je serijski po projektu.** Dok jedan traje, novi događaj **zamjenjuje** čekanje, ne
   dodaje ga u red. Test: deset događaja za vrijeme izračuna = najviše **jedan** dodatni izračun.

### 3.4 Rubovi iz duga

- **Detached HEAD:** `git branch --show-current` je prazan i kad repo ima commite. `io` tada uzima
  `git rev-parse --short HEAD` i izvještaj nosi `branch: "HEAD@<sha>"` umjesto lažne poruke
  „repozitorij nema commita". Test nad privremenim repoom u detached stanju.
- **`--since` s rezervom jednog dana** ostaje (S-011 dopuna); `until` dobiva istu rezervu u drugom smjeru.

**Greške** (dopuna M1 popisa): `PathOutsideRoot` s imenom polja · mapa nije git-repo → poruka u
dijalogu, ne pad · stablo obrisano izvan Sokratisa (`git worktree remove`) → projekt ostaje, stablo
nestaje s popisa pri idućem osvježenju, glavno stablo se **ne** mijenja samo od sebe.

---

## 4 · Pohrana (`sokratis-store`) — SQLite u `%LOCALAPPDATA%\sokratis\sokratis.db`

Načelo PRD §6 („git je izvor istine; izvještaj je uvijek isti za isti git") se **ne razvodnjava**:
git je nepromjenjiv i izvještaj je determinističan, pa je sve što ovdje stoji ili **istina koju git
ne zna** (registar, postavke) ili **pogodnost koja se izvodi iznova** (snimke, keš). Nikad drugi
izvor istine (S-014). Zamjenjuje `%APPDATA%\sokratis\config.json` iz ocrta M1 — registar je u bazi.

### 4.1 Shema

| tablica | stupci | status |
|---|---|---|
| `schema_version` | `version` | migracije: jedna datoteka po verziji, `store` ih primjenjuje pri otvaranju |
| `project` | `id · name · root_path` (glavno stablo) `· git_common_dir · added_at · last_seen_at` | **istina** |
| `project_worktree` | `project_id · path · branch · seen_at` | izvedeno iz gita, osvježivo |
| `setting` | `key · value` — `theme` · `lang` · `autostart` · `window` (položaj i veličina prozora) | **istina** |
| `project_setting` | `project_id · key · value` — `range` · `last_view` | **istina** |
| `profile_seen` | `id · project_id · json` (kanonski JSON profila, `UNIQUE` po projektu) `· first_seen` | izvedeno |
| `snapshot` | `project_id · taken_on · profile_seen_id · metric · value · kind` — PK `(project_id, taken_on, metric)` | pogodnost |
| `commit_cache` | `project_id · sha · author_time · commit_time · date · commit_date · subject · files_json` — PK `(project_id, sha)` | pogodnost, **tek nakon mjerenja** (§4.4) |

### 4.2 Što snimka drži — i što namjerno ne drži

Snimka **ne drži cijeli `Report`**, nego `SnapshotMetrics` (§2): 18 pokazatelja + docs-ocjena + broj
signala po težini. Cijeli JSON bio bi suvišan — `sokratis report --since X --until Y` ga reproducira
iz gita. Uz svaku snimku stoji **`profile_seen_id`**: kad Leon promijeni regex u profilu, nova
snimka pokazuje na drugi zapis, i graf trenda **pokaže oznaku** na tom danu umjesto da nacrta lom
koji izgleda kao pad produktivnosti, a zapravo je promjena mjerila. Profil se pamti kao kanonski JSON
(sortirani ključevi), ne kao hash — `DefaultHasher` nije stabilan među verzijama Rusta, a nova
ovisnost samo za hash je pravilo #6 naopako.

### 4.3 Kad se snimka uzima

**Jednom dnevno po projektu**, pri prvom izračunu tog dana, ključ `(project_id, taken_on)` s upisom
preko starog (isti dan, novija brojka pobjeđuje). Nema zapisa po commitu — to bi bio „dnevni zadatak
koji bilježi", odbijen u BACKLOG-u. Trend čita `snapshot` izravno; nema izvedene tablice dok
mjerenje ne kaže da treba.

### 4.4 Keš po SHA — samo sirove činjenice, i samo ako mjerenje to traži

Keš drži **ono što je git rekao o commitu** (vremena, naslov, numstat), **nikad klasifikaciju** —
vrsta rada ovisi o regexima iz profila, pa bi keširana vrsta tiho preživjela promjenu profila.
Klasificiranje je jeftino; skupi su procesi. **Pravilo #4:** cigla keša dolazi **nakon** M11 (§3.2)
i mjerenja: ako izvještaj nad Sokrat Studyjem s tri git-poziva već stane ispod 500 ms, tablica
ostaje definirana a prazna i cigla se odgađa **s brojkom** u izvještaju; ako ne stane, keš ulazi i
`io` dovlači log inkrementalno (`--since` od najnovijeg keširanog `commit_date`).

---

## 5 · Desktop (`apps/desktop/src-tauri`, crate `sokratis-desktop`)

### 5.1 Naredbe i događaji — ugovor (S-012)

| naredba | vraća | radi |
|---|---|---|
| `list_projects()` | `Vec<ProjectSummary>` | registar + zadnji izračun po projektu |
| `add_project(path)` | `ProjectSummary` | `git-common-dir`, stabla, ime iz mape; duplikat → greška s imenom projekta |
| `rename_project(id, name)` · `remove_project(id)` | — | samo registar; `.sokratis/` u repou se ne dira |
| `get_report(id, range)` | **`Report` nepromijenjen** | `range = { since?, until? }` ili preset (`7d` · `30d` · `month` · `all`) |
| `get_trend(id, metric, range)` | `Vec<{ taken_on, value, profile_changed: bool }>` | iz `snapshot` |
| `set_override(id, sha, kind?)` · `save_visions(id, visions)` | — | `io` piše u glavno stablo (§3.1) → ponovni izračun |
| `refresh(id?)` | — | gumb; bez `id` = svi |
| `get_settings()` · `set_setting(key, value)` | — | tema · jezik · autostart (autostart mijenja i registraciju u OS-u) |
| `copy_path(id, rel)` | — | Dokumentacija: klik na nalaz kopira apsolutnu putanju u međuspremnik |

Događaji: **`report_updated { project_id }`** (nakon svakog izračuna) i **`signal_raised
{ project_id, rule, severity }`** — samo na **prijelaz** Info/Warn → Alert ili nov Alert, ne pri
svakom osvježavanju; ista nespojena grana ne javlja svakih par minuta.

**`ProjectSummary` je jedini nov oblik prema sučelju** i namjerno je tanak:
`id · name · root_path · worktrees: u32 · last_refresh: Option<i64> · last_commit: Option<{ sha, time,
subject }> · worst: Option<Severity> · signals: SignalCounts · error: Option<String>` (projekt čija
mapa više ne postoji ne ruši Pregled — nosi poruku).

### 5.2 Pokretanje, prozori, splash (S-019)

- Proces starta s **glavnim prozorom skrivenim** i **splash prozorom** (bez okvira, podloga `#0b1017`,
  omjer 1000:560). Za vrijeme animacije `io` učitava sve projekte.
- Glavni prozor se pokazuje kad su **oba** gotova: animacija odsvirala (4,2 s) **i** projekti učitani
  — što je kasnije; ako učitavanje traje dulje, zadnji kadar stoji i prsten se polako vrti. Klik ili
  tipka preskače na zadnji kadar (učitavanje se ne preskače, ali se čeka na zadnjem kadru).
- Animacija se vrti **jednom po pokretanju procesa** — otvaranje prozora iz traya je bez nje.
- `prefers-reduced-motion: reduce` → zadnji kadar odmah, kao što Leonov kod već radi.
- **Animacija se preuzima doslovno** iz Leonove datoteke
  (`C:\Users\leonk\Downloads\sokratis-intro-clean-graph.html`, 2026-09-18): dva WebP-a (`graphOnly` bez
  lika, `picture` s likom) izvučena iz base64 u `apps/desktop/src/assets/intro/`, i vremenska crta
  (stupci 160→1000 ms · brisanje 750→1500 · prsten 1400→2050 · lik 2200→3150 · skupljanje i natpis
  3400→4050, boja natpisa `#00dce8`, verzal `S ◍ KRATIS`). Portiranje u Svelte komponentu, canvas
  ostaje canvas.

### 5.3 Tray, obavijesti, autostart, jedna instanca (S-020)

- **X sakriva** prozor; izlaz je izričit: desni klik na tray → *Izađi*. Watcher radi i dok je prozor
  skriven — zato obavijest stiže i dok Leon ne gleda.
- Tray: lijevi klik = pokaži/podigni prozor; desni = *Otvori · Osvježi sve · Autostart ✓ · Izađi*.
- **Autostart** (registracija u OS-u kroz `tauri-plugin-autostart`) je postavka koju Leon može isključiti.
- **Obavijest OS-a** samo na `signal_raised`; tekst nosi projekt, pravilo i prvi redak dokaza; klik
  otvara taj projekt.
- **Jedna instanca** (`tauri-plugin-single-instance`): drugo pokretanje ne otvara drugu tray-ikonu
  nego podiže postojeći prozor. Bez toga autostart + ručni klik daju dva Sokratisa i dva watchera.

### 5.4 Ikone

`npm run tauri icon "C:\Users\leonk\Downloads\sokratis logo .png"` → `src-tauri/icons/` (`.ico`,
PNG set). **Tray dobiva pojednostavljen znak** (prsten + stupci, bez Sokratova lika) na 16 i 32 px —
na toj veličini lik i tri čvora se stope u mrlju. Provjera je **gledanjem**, zapisana u izvještaju
cigle sa snimkom zaslona. Znak ostaje rasterski (WebP/PNG iz Leonove datoteke); vektorizacija nije u
opsegu.

---

## 6 · Sučelje (`apps/desktop/src`, Svelte 5)

### 6.1 Okvir — kao Sokrat Study

Gornja traka + lijevi izbornik; uzor su `css/topbar.css` · `sidebar.css` · `study-chrome.css` Sokrat
Studyja (prenose se **mjere i ponašanje**, ne klase — Sokrat Study se ne dira). Gornja traka: mali
`S◍KRATIS` lockup · birač projekta · **birač raspona** (7 dana · 30 dana · ovaj mjesec · cijeli
projekt = `profile.since` · vlastiti; pamti se po projektu) · gumb Osvježi · tema · HR/EN. Lijevo:
pogledi projekta. Bez odabranog projekta sadržaj je Pregled.

### 6.2 Pogledi — svih 8 + Pregled

| pogled | pokazuje | piše |
|---|---|---|
| **Pregled** | kartica po projektu: ime · stabla · zadnji commit („prije 2 h") · najteži signal u boji · brojevi po težini · greška ako mapa ne postoji; „Dodaj projekt" (dijalog mape) | — |
| Tempo | stupci po danu (commiti / sati, prekidač) · kumulativna linija · tablica dana | — |
| Vrste rada | prsten udjela + tablica s **postocima** | — |
| Pokazatelji | 18 kartica: vrijednost · oznaka **mjera / proxy** · formula na klik · **sparkline** trenda iz snimki s oznakom promjene profila | — |
| Faze | zatvorene · aktivne · planirane; dani · commiti · cigle/dan; **„nema faza"** umjesto praznog naslova | — |
| Dnevnik | `CommitRow` po commitu: SHA · datum · naslov · vrsta · podvrsta; **vrsta se mijenja u mjestu**, override je označen | `overrides.json` |
| Isporuke | iz dnevnika (`Delivery`): datum · model · naslov · deploy | — |
| Vizije | popis po stanju + **zbroj po stanju** (`vision_totals`); dodaj · uredi · promijeni stanje | `visions.json` |
| Dokumentacija | ocjena · kašnjenje · nalazi `datoteka:redak` (klik kopira putanju; otvaranje u editoru je M3) | — |

Signali projekta stoje u **zaglavlju projekta** (traka s dokazom na klik), ne kao deseti pogled —
Pregled ih ionako nosi.

### 6.3 Oblikovanje — jedan modul, s testom (rješava dug M3)

`src/lib/format.ts`: postoci (`0.589` → `58,9 %`), sati na jednu decimalu, datumi po jeziku,
relativno vrijeme („prije 2 h"), stanja faza kroz i18n (`Closed` → „zatvorena"), prazna stanja.
**Nijedna komponenta ne oblikuje sama.** Test: `vitest` nad svakom funkcijom s rubovima (0, `null`,
negativno, > 100 %).

### 6.4 HR/EN (S-021)

`src/i18n/hr.json` + `en.json`, jedan ključ po natpisu, vlastiti `t(key)` bez biblioteke. **Test tvrdi
da obje datoteke imaju iste ključeve** (nedostaje li ključ, `npm run check` pada). Nijedan natpis u
komponenti. Jezgrini engleski identifikatori (S-008) su ključevi, natpisi su vrijednosti.

### 6.5 Teme i tokeni (S-017)

`tokens.css` Sokrat Studyja preuzima se **strukturno cijel**: `@theme static`, semantička imena
(`surface`/`ink`/`brand`/statusi), sve četiri palete (`:root[data-theme="chalk|academic|mint|carbon"]`,
zadano bez atributa = „Akademsko plavo"), brisanje Tailwindove zadane palete (`--color-*: initial`).
**Jedina izmjena je obitelj `brand-*`**, izvedena iz hue-a loga (`#00dce8`, ≈183°): neonski cijan
sam po sebi na bijeloj ima kontrast **≈1,7:1** i ne može biti tekst ni rub — pa se ljestvica izvodi
tako da `brand-600/700` prolaze **≥ 4,5:1 na sve tri plohe svake svijetle teme**, a neon ostaje za
tamne teme i za sam znak; ljubičasta iz čvorova (`#9b4df0`-ish) je naglasak. **Provjera je skripta**
(`npm run check:contrast`, po uzoru na Sokrat Studyjev `check:contrast`), ne oko: sve četiri teme ×
sve plohe × `ink`/`brand`/statusi. Tailwind kroz **`@tailwindcss/vite`** — Vite je ionako tu zbog
Sveltea; S-006 je taj izbor ostavio ovom specu.

### 6.6 Grafovi — vlastiti SVG (S-018)

Četiri komponente: `Bars` · `Line` · `Ring` · `Sparkline`. Boje **isključivo** `var(--color-*)`, pa
sve četiri teme rade bez retka dodatnog koda; tipografija i razmaci isti kao ostatak sučelja. Nema
ovisnosti koju treba pinati i obrazlagati. Tooltip je jedan `<title>`/`aria-label` po elementu.

---

## 7 · Ovisnosti — sve nabrojane, sve pinane (pravilo #6)

| gdje | crate / paket | uloga |
|---|---|---|
| `store` | `rusqlite` (`bundled`) | SQLite bez sustavske biblioteke; `bundled` jer tuđi stroj nema `sqlite3.dll` |
| `io` | `notify` | watcher datoteka; jedini kandidat već u M1 (`RUST.md` §1) |
| `core` (dev) | `insta` | snapshot `Report`-a — vraća se jednim retkom, kako je M1 najavio |
| `desktop` | `tauri` 2 · `tauri-plugin-dialog` · `tauri-plugin-notification` · `tauri-plugin-autostart` · `tauri-plugin-single-instance` | ljuska · odabir mape · obavijesti · autostart · jedna instanca |
| `apps/desktop` (npm) | `svelte` 5 · `vite` · `@sveltejs/vite-plugin-svelte` · `tailwindcss` 4 · `@tailwindcss/vite` · `@tauri-apps/api` · `@tauri-apps/cli` · `typescript` · `svelte-check` · `vitest` | sučelje · gradnja · tokeni · most prema naredbama · Tauri CLI **kao dev-ovisnost, ne globalna instalacija** · tipovi · testovi |

Svaka nova ovisnost = svoj commit s obrazloženjem; `Cargo.lock` i `package-lock.json` se commitaju;
`RUST.md` §2 dobiva redak po crateu kad cigla uđe. **Nema** biblioteke za grafove, i18n, state
management ni UI-komponente.

---

## 8 · Dug iz M1 koji M2 preuzima (7 od 9) i što ostaje

| stavka (BACKLOG „Iz završne recenzije M1") | gdje u ovom specu | kako se dokazuje |
|---|---|---|
| snapshot `Report`-a (I7) | §9, **prva cigla** | `insta` test zelen; svaka promjena oblika = namjeran `insta review` |
| performanse (M11) | §3.2 | io-test broji procese `≤ 5`; ručno < 500 ms nad Sokrat Studyjem u izvještaju cigle |
| ograda putanja (I9) | §2 `inside_root` | test: `docs_dir: "../.."` i apsolutna putanja → `PathOutsideRoot { field }` |
| zbroj vizija (I6) | §2 `vision_totals` | test nad fixtureom vizija |
| `phase_tag` (I3 + M14) | §2 | test: profil s `M1-3` oznakom dobiva `from`/`days`/`commits`, ne prazno |
| detached HEAD | §3.4 | io-test u detached stanju |
| testni redak koji fixture prevlada (odg. 8) | §2 `test_path_exclude` | test: Sokratisov profil, `fixtures/` izuzet |
| natpisi i jedinice **CLI-tablice** (M3) | **ostaje** (BACKLOG) | sučelje ih rješava u §6.3; tablica u terminalu je pomoć, JSON je ugovor |
| `include_unmerged` (odg. 7) | **ostaje** (BACKLOG) | metrike nad nespojenim granama nisu tražene ni u jednom pogledu |

---

## 9 · Testiranje (detalji ulaze u [workflow/TESTING.md](../workflow/TESTING.md) kad cigle uđu)

| sloj | što | kako |
|---|---|---|
| `core` | **snapshot `Report`-a nad fixtureom pariteta** (S-022, prva cigla) · `until` · `vision_totals` · `phase_tag` iz profila · `inside_root` odbija `..` i apsolutnu putanju **s imenom polja** · `test_path_exclude` · `SnapshotMetrics::from(&Report)` i `diff` | jedinični, `insta` za snapshot |
| `io` | grupiranje pet stabala u jedan projekt · `write_override`/`write_visions` atomarno i pretty · watcher: commit → događaj ≤ 2 s; rafal 3 = 1; vlastiti upis = 0; serijski · **M11 kao mjerenje**: brojač procesa `≤ 5` nad 60 `.md` + 30 grana · detached HEAD | integracijski, privremeni repo (`tempfile`) |
| `store` | migracije od prazne baze · registar CRUD · duplikat stabla → isti projekt · snimka jednom dnevno (upsert) · `profile_seen` različit = `profile_changed` u trendu · keš pogodak/promašaj (ako uđe, §4.4) | `:memory:` |
| `cli` | postojeći testovi izlaznih kodova · `--until` | postojeći + jedan |
| Svelte | `format.ts` rubovi · i18n ključevi jednaki · **kontrast sve četiri teme** · komponente grafova s praznim ulazom (0 točaka, 1 točka) | `vitest` + `check:contrast` |
| paritet | postojeći test **ostaje zelen**; Sokrat Study 190 / 70164 / 1801 nepromijenjeno | postojeći |
| `desktop` | nema logike → nema jediničnih; **ručna lista provjere** u izvještaju cigle: splash do kraja i preskočen · reduced-motion · tray klikovi · obavijest na Alert · drugo pokretanje · X sakriva | ručno, zapisano sa snimkama |

**Brane prije commita:** `cargo fmt --check` · `cargo clippy --all-targets -- -D warnings` ·
`cargo test` · **`npm run check`** (svelte-check · vitest · i18n · kontrast) u `apps/desktop`.
Crveno ne ide u commit.

---

## 10 · M0 za M2 — alati (stanje stroja 2026-09-18, izmjereno)

| alat | stanje | treba li što |
|---|---|---|
| Node 24.11.1 · npm 11.6.2 | ✅ | ne |
| WebView2 153.0.4234.32 (sustavski) | ✅ | ne |
| MSVC 14.44 (Build Tools 2022) · `link.exe` | ✅ | ne |
| Rust 1.98.1 (pinan) | ✅ | ne — Tauri 2 traži ≥ 1.77 |
| `cargo tauri` | ❌ | **ne treba globalno**: `@tauri-apps/cli` je pinana dev-ovisnost u `apps/desktop/package.json`, poziva se `npm run tauri …` |
| pnpm / yarn / bun | ❌ | ne; npm je dovoljan |
| NSIS / WiX (bundler) | ❌ | ne u M2 — instalater je M3 |

**Jedina instalacija je `npm install` u `apps/desktop`** (skida pinane pakete u `node_modules/`, ne
mijenja sustav) — i ona, kao u M0 za M1, čeka **Leonov OK** prije prve cigle koja je treba.

---

## 11 · Izlazni uvjet Milestonea 2 — verzija 0.2.0

Gotovo kad Leon može:

1. **pokrenuti Sokratis** (autostart ili klik), vidjeti animaciju do kraja, i dobiti **Pregled** sa
   Sokrat Studyjem (**pet stabala = jedan projekt**) i Sokratisom, sa signalima u boji;
2. ući u Sokrat Study i vidjeti **svih 8 pogleda** s brojkama **istim kao `sokratis report --json
   --since … --until …`** — jer čitaju isti JSON (S-012); snapshot-test čuva taj oblik, a ručna
   usporedba tri brojke (commiti · sati · docs-ocjena) stoji u izvještaju cigle;
3. commitati u `sokratstudy.f21` i vidjeti brojke osvježene **bez klika, unutar 2 s** (izračun ispod
   500 ms, izmjereno i zapisano);
4. promijeniti vrstu rada jednom commitu u Dnevniku i naći to u `sokratstudy.dev\.sokratis\overrides.json`
   kao necommitanu izmjenu od jednog retka — Sokratis nikad ne commita sam;
5. zatvoriti prozor, ostati u trayu, i dobiti **sistemsku obavijest** kad signal prijeđe u Alert;
   klik na nju otvara taj projekt; drugo pokretanje ne otvara drugu ikonu;
6. prebaciti temu (sve četiri prolaze `check:contrast`) i jezik (HR/EN, isti ključevi);
7. vidjeti **trend** docs-ocjene i tempa kroz dane iz snimki, s vidljivom oznakom gdje se profil mijenjao;
8. `cargo test` + `npm run check` zeleno, clippy čist, snapshot `Report`-a zelen, paritet zelen, svaka
   cigla s „zašto Rust ovako" u zaglavlju; **sedam stavki duga iz §8 zatvoreno s testom.**

Zatim: **STANI, javi se**, spec seli u `archive/`, `architecture/ARCHITECTURE.md` opisuje što je
izgrađeno, piše se spec za M3.

## 12 · Izvan opsega M2 (ostaje u BACKLOG-u, nijedno skriveno)

instalater (MSI/NSIS) · nova pravila signala · GitHub/Vercel adapteri · otvaranje datoteke u editoru
(samo kopiranje putanje) · natpisi CLI-tablice · `include_unmerged` · vektorizacija znaka · README EN,
licenca, objava · push/remote (čeka Leonov izričit OK; javna objava tek nakon I9, koji ovaj spec zatvara).
*(Pretečeno 2026-09-20: Leon je repo objavio javno prije kraja M2 — `DECISIONS.md` S-023; I9 i dalje zatvara ovaj spec, ciglom T14.)*
