# BACKLOG — parkiralište

> **Nije plan i nije obećanje.** Stavka ovdje čeka milestone koji je uzme ili odluku koja je odbije.
> Plan ne nosi tuđe stavke; kad stavka uđe u spec, ovdje ostaje samo pointer.

## Čeka milestone

| stavka | za | bilješka |
|---|---|---|
| Pravila M3: udio debugginga raste 3 dana · cigle/dan ispod prosjeka zatvorenih faza · deploy > 14 dana · udio testnih pada · commiti bez unosa u dnevniku · faza dulja od prosjeka · **(Leon, 2026-09-22)** dokumentacija kasni za kodom po datotekama (npr. `ARCHITECTURE.md` nedirnut N dana dok `src/` ima M commita) · test-omjer po cigli (commit u `src/` bez pripadnog testa, pravilo #7) · faza bez zatvaranja dulje od procjene iz plana · nova `.rs`/`.ts` bez zaglavlja „zašto" (pravilo #5) · obećanje u dnevniku („sljedeća sesija: X") bez commita koji X dotiče u N dana | M3 | popis iz speca §2.5; svako pravilo = svoja datoteka + test |
| Profil za tuđe projekte: dokumentiran JSON format + primjer za repo bez `docs/`; **(Leon, 2026-09-22)** čarobnjak koji PREDLOŽI `profile.json` iz onoga što vidi u repou umjesto tihog Sokrat Studyjeva zadanog, i Conventional Commits kao drugi ponuđeni zadani profil. **Prilagodba korisnikovu načinu rada (kandidat, 2026-09-28):** (1) prepoznavanje konvencija iz repoa s dokazom (koji je naslov dnevnika, koji plan, koje testne putanje — i zašto); (2) izvor etapa i jedinica rada kao izbor u profilu (plan · tagovi · ručni rasponi · ništa); (3) natpisi po korisniku i skrivanje sekcija bez izvora | M3 | zadano = Sokrat Study (S-005); §13.7 (M2, gotovo) jamči da brojke nad neprepoznatim repoom ne lažu — čarobnjak je sljedeći korak, prijedlog umjesto tišine. Sloj (2) mijenja ugovor `Report` — namjerna promjena NAKON 1.0.0 (S-022); Sokrat Study ostaje prva predložak-postavka, paritet netaknut |
| GitHub adapter: CI status po grani, PR-ovi | M3 (opcionalno) | mreža; `gh` nije na stroju |
| Vercel adapter: deployi umjesto `🚀` u dnevniku | M3 (opcionalno) | mreža |
| HR/EN natpisi u **sučelju** | → spec M2 [§6.4](../archive/ARHITEKTURA_M2.md) (S-021) | samo pointer |
| HR/EN natpisi u **CLI tablici** | M3 | jezgra je već engleska (S-008); tablica je pomoć za terminal |
| GitHub Actions | M4 (objava) | README na engleskom je gotov 2026-09-21 (S-030); licenca je odlučena 2026-09-29 (S-039) |
| Obavijesti o licencama ugrađenih komponenti uz instalater (npr. `THIRD-PARTY-NOTICES`) | zakrpa 1.0.1 ili M4 — odluka | MIT · Apache-2.0 · BSD · MPL-2.0 traže da tekst licence i autorska obavijest putuju uz binarnu distribuciju; instalater ih danas ne nosi (popis je samo u `Cargo.lock` i `package-lock.json`). Alat koji to generira je nova razvojna ovisnost — namjerna radnja (pravilo #6) |
| Instalater NSIS samo za Leona | → spec M2 [§13.6](../archive/ARHITEKTURA_M2.md) (S-029) | samo pointer |
| Potpisan i objavljen instalater · MSI · automatsko ažuriranje | M4 (objava) | S-029 |
| `sokratis docs .` mjeri samo korijen i `docs_dir`; `.md` pod `apps/desktop` (npr. budući README sučelja) nitko ne provjerava | M3 (profil: više `docs_dir`-ova ili `extra_docs_dirs`) | nalaz čuvara 2026-09-18 |
| „Klasifikacija jednom" (spec §3.2) nije dovršena: `metrics/kinds.rs::commit_rows`/`kind_stats` (M2/5) klasificiraju svaki commit jednom za `Report.commits`, ali `metrics/indicators.rs` (`kind_count`) i dalje zove `effective_kind` odvojeno za `debugging_commits`/`docs_share` — commit se klasificira više od jednom | M3 (završna recenzija 1.0.0 ga nije uzela u krug) | nalaz recenzije M2/5 (2026-09-18); stanje koda: `ARCHITECTURE.md` §11 |
| `sokratis report --table` u zaglavlju ispisuje samo „od {since}" — `until` se u tabličnom ispisu ne vidi iako je prozor ograničen (JSON je točan) | M3 (završna recenzija 1.0.0: čeka) | nalaz recenzije M2/19 (2026-09-20); `table.rs` nema presedan za uvjetno dodavanje polja u zaglavlje |
| Keširani put (`cached_log`, M2/14b) ne broji `touched.skipped_lines` iz PRVOG čitanja commita — keš pamti `Commit`-e (strukturu), ne sirovi tekst git loga | M3 ili odluka da nije bitno | poznato ograničenje ugrađeno u zaglavlje `io/src/cache.rs`; ne utječe na commite/redak-brojke, samo na broj preskočenih redaka pri parsiranju |
| Ključ keša sirovih commita je kratki SHA (`%h`) — ako git jednog dana produlji zadanu duljinu kratice, keš se jednom puni iznova (jednokratni trošak, ne kvar) | M3 ili odluka da nije bitno | poznato ograničenje ugrađeno u zaglavlje `io/src/cache.rs`; točnost brojki ne strada |
| Zajednički graf usporedbe projekata na Pregledu (linija po projektu) | M3 | izvan opsega 1.0.0 — [ARHITEKTURA_1_0.md §9](../archive/ARHITEKTURA_1_0.md); srodno: „usporedba projekata na Pregledu" niže (Leon, 2026-09-22) je redak na kartici, ovo je zajednički graf |
| `branch_ignore` — isključivanje grana po uzorku iz metrika | M3 ili odluka | izvan opsega 1.0.0 — [ARHITEKTURA_1_0.md §9](../archive/ARHITEKTURA_1_0.md); S-032 zadano mjeri SVE lokalne grane, ovo bi bio izuzetak |
| Sat autora u formatu loga — doba dana prikazano u zoni autora, ne stroja | M3 ili odluka | izvan opsega 1.0.0 — [ARHITEKTURA_1_0.md §9](../archive/ARHITEKTURA_1_0.md) |
| **Agenti i tokeni** (kandidat, 2026-09-28): Sokratis pokazuje koliko je agenata radilo na projektu, koliko je tokena potrošeno i koliko se pametno troše | M3 | nov izvor podataka — lokalni transkripti sesija, IZVAN repoa, format bez jamstva — iza vlastitog traita u `io` (isti obrazac kao `GitSource`, S-003); prikaz u tokenima, ne u novcu; „savjet" samo kao pravilo s dokazom; prvo jednokratno mjerenje skriptom (pravilo #4). Polazna brojka iz sesije 5 drugog reza: 12 pokretanja podagenata ≈ 1,7 M tokena (čuvar 197 k · recenzenti 57–128 k · graditelji 103–227 k) |
| **Dnevnik iz više datoteka** (kandidat, 2026-09-28): `diary_path` kao popis datoteka, ili arhiva dnevnika koju mjerenje i dalje čita | M3 (nakon 1.0.0 — mijenja ugovor profila) | izmjereno na izdanju 1.0.0: seljenje M1 sesija iz `PROGRESS.md` u `docs/archive/PROGRESS_M1.md` bi mjerenju Sokratisa samog srušilo isporuke **18 → 16** (i `deliveries_per_day`, `owner_driven_deliveries`), jer `Profile::diary_path` je jedan `String` (`core/src/profile.rs`) a `io` čita samo tu datoteku po stablu. Do odluke M1 sesije ostaju u `PROGRESS.md`; odluka (ostaju · sele uz pad · sele uz pomak `since` · ova promjena koda) je Leonova |
| **Prvi vanjski korisnik nakon 1.0.0** (kandidat, 2026-09-28) | odluka prije M4 | otvoreno: (a) koja **licenca** (red „LICENCA" niže); (b) dobiva li vanjski korisnik instalater (S-029 danas kaže „instalater samo za vlasnika", nepotpisan je) ili samo poveznicu na repo; (c) **zadani `since` u profilu** (S-005: `2026-08-29`) nad tuđim repoom bez profila — „Sve" tada počinje 29. 8. 2026 i starija povijest izgleda kao da je nema (nalaz m7 završne recenzije 1.0.0; srodno: red 2 „Nakon 1.0.0 — vanjska lista") |

## Druga verzija (M3) — iz Leonova zapisa namjere 2026-09-21

Rez: S-025. Što je od zapisa namjere ušlo u 1.0.0 stoji u specu M2 §13; ovdje je ostatak. Druga verzija
dobiva **svoj spec** (brainstorming → spec → plan) tek kad je 1.0.0 u uporabi — Leon želi da prva
verzija nadgleda gradnju druge, pa se redoslijed ispod još može promijeniti iz stvarne uporabe.

| stavka | bilješka |
|---|---|
| **Ocjena projekta F− … A+** u boji (crveno → zeleno), vidljiva odmah na otvaranju; temelji se na izradi i cijelom radu projekta prema dokumentaciji | nova mjera u jezgri — nema uzora u `RAD.xlsx`, traži definiciju (od čega se sastoji, kako se važe) i test; docs-ocjena 0–100 već postoji i vjerojatno je jedan od ulaza |
| **Omjer popravaka** (popravci prema novom radu) | Leonova želja; vrste rada već postoje (`debugging` …), treba definirati omjer i po čemu se dijeli (dan · faza · projekt) |
| Još statistike i grafova | kandidati orkestratora, neodlučeno: udio commita s AI-koautorom (`Co-Authored-By`) · toplinska karta dan × sat · trajanje cigle |
| **Postavke s vlastitim dodacima** | Leon želi sve četvero; redoslijed po cijeni: vlastiti pragovi i pravila signala → vlastiti pokazatelj (formula nad postojećim brojkama) → slaganje Pregleda → pravi pluginovi (kod) zadnji |
| Kartica s objašnjenjem **s dokazom** — commiti ili dani iz kojih je broj nastao | 1.0.0 ima statičnu karticu (S-027) |
| Punjenje trenda iz git-povijesti | u 1.0.0 trend kreće prazan od dana instalacije (Leon, 2026-09-21); tempo i commiti se mogu izračunati unatrag (`--until` po danu), docs-ocjena ne bez skupog čitanja starih stabala |
| Izvoz — **(Leon, 2026-09-22)** MD/CSV/PDF, zamjena za `RAD.xlsx` | „bit će potrebno u novijim verzijama“; do tada je izvoz `sokratis report --json` |
| Izgled u iOS stilu, blago „glossy“ | Leon: „nije toliko bitno još“; tokeni (S-017) su mjesto gdje se to mijenja |
| **(Leon, 2026-09-22)** Kalibracija sati: „koliko si danas stvarno radio?" mijenja osobni prag razmaka između commita; opcionalni senzor mtimea datoteka u stablu (npr. „radio si 14:10–16:40, commitao 17:00") | sati ostaju git-proxy (PRD §5, „Štoperica za sate" je odbijena niže) — ovo je kalibracija proxyja, ne mjerenje uživo; odluka je Leonova kad dođe na red |
| **(Leon, 2026-09-22)** Tjedni izvještaj-obavijest ponedjeljkom („prošli tjedan: 14 h, 23 commita, docs 100→92, 1 nov signal") · usporedba dva razdoblja · usporedba projekata na Pregledu (redak s omjerom na karticama) | trend iz snimki (S-014) je preduvjet; obavijest isti mehanizam kao signal-Alert (S-020) |
| **(Leon, 2026-09-22)** Dnevni streak + heatmap kalendar (godišnji pregled, ne toplinska karta dan × sat gore) · pogled „Danas" · kratice tipkovnice | sitnice, niska cijena; heatmap dan × sat (red iznad) ostaje zaseban graf |
| **(Leon, 2026-09-22)** Klik na obavijest OS-a otvara projekt | poznat rub koda, ne nova ideja — plugin na Windowsu danas nema povratni poziv za klik (Ruling R7, `ARCHITECTURE.md` §11); Leon ga želi popravljenog |
| **(Leon, 2026-09-22)** CLI izlazni kodovi dokumentirani KAO NAMJENA za CI (preflight) | ugovor već postoji i testiran je (`0`/`1`/`2`/`3`, `TESTING.md` §1); ide uz GitHub Actions (M4, „Čeka milestone" gore) kao primjer uporabe, ne novo ponašanje |

## Kasnije — zasebnim planom

| stavka | bilješka |
|---|---|
| **Timovi:** razrada po autoru u jednom repou · dijeljeni rad preko korisnikova **vlastitog** Supabasea, servera ili čega drugog | sudara se s PRD §5 „sve lokalno“ i s retkom „Oblak / računi / Supabase“ u „Odbijeno“ niže — to odbijanje vrijedi za 1.0.0 i drugu verziju; timovi ga ponovno otvaraju tek vlastitom odlukom i specom |
| **Ne samo git:** mape bez gita · Issues/Linear · time-tracker kao izvor | `GitSource` je već trait (S-003) — drugi izvor je nova implementacija, ali `Report` danas pretpostavlja commite |
| Prodaja kao proizvod | Leon: „moguće kasnije, sad ne ulazi u priču“; licenca 1.0.0 (S-039) dopušta besplatnu uporabu i ne može se stegnuti za već objavljenu verziju |
| Treći i četvrti projekt pod nadzorom (dva Leonova projekta izvan ovog repoa; imena se ne zapisuju u javni repo) | ne zna se drže li se konvencija Sokrat Studyja → spec M2 §13.7 jamči da brojke ne lažu; profil za tuđe projekte je redak u „Čeka milestone“ |

## Iz završne recenzije M1 (nalazi koji nisu popravljeni u M1)

Izvor je izvještaj završne recenzije (`.superpowers/sdd/2026-09-17-m1-jezgra-i-cli/final-review-report.md`
— radni zapis izvan gita), zato je uz svaku stavku broj nalaza. **Stanje koda** (što danas ne radi)
opisuje [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11; ovdje stoji **plan**.

**Sedam od devet stavki preuzeo je spec M2** — [`../archive/ARHITEKTURA_M2.md`](../archive/ARHITEKTURA_M2.md)
§8 ih nabraja s mjestom u specu i testom koji ih dokazuje: snapshot `Report`-a (I7) · performanse
(M11) · ograda putanja (I9) · zbroj vizija (I6) · `phase_tag` (I3 + M14) · detached HEAD · testni redak
koji fixture prevlada (odgođena 8). **Svih sedam je sad riješeno u `main`-u.** **I7 je riješena ciglom
M2/2** (2026-09-18): snapshot-test postoji i pada na svaku nenamjernu promjenu oblika. **I9 je
zatvorena u cijelosti** (2026-09-20, tok IO): jezgra (`Profile::validate_paths`/`inside_root`, M2/8)
odbija putanju izvan repoa; io-dio — `io::Project::open` zove `validate_paths()` odmah nakon
učitavanja profila, prije nego se ijedna putanja pročita s diska (M2/14, merge `3ca068c`) — je ušao.
Repo je od 2026-09-20 javan (S-023); ova ograda je bila dug prema javnom kodu, sad zatvoren u
cijelosti. Licenca postoji od 2026-09-29 (S-039), pa iz te odluke više ništa nije otvoreno. Testni redak
koji fixture prevlada (odgođena 8) je riješen ciglom M2/9 (`test_path_exclude`).
**I6 je riješena ciglom M2/4**: `Report.vision_totals` zbraja vizije po stanju. **`phase_tag` (I3 +
M14) je riješen ciglom M2/6**: aktivne faze se na commit vežu regexom iz profila, ne tvrdim
prefiksom; zadani profil (Sokrat Study) daje iste brojke kao prije (diff snimke bajtno prazan).
**Performanse (M11) su izmjerene i popravljene ciglom M2/11** (tok IO, u `main`-u od 2026-09-20):
3318–3822 ms → 1479,71 ms nad Sokrat Studyjem (release, topli keš), git-procesa 92 → 6; krug popravka
(`--diff-merges=combined`, `-c core.quotepath=false`) potvrdio 0/56 neslaganja nad Sokrat Studyjem.
Mjerenje je ostalo ≥ prag 500 ms — zato je keš sirovih commita (M2/18) ušao u plan; sad je i on u
`main`-u (tok STORE). **Detached HEAD riješen ciglom M2/12**: `Report.branch` postaje `HEAD@<sha>`,
ne lažno „nema commita".
Stanje koda za sve gore: [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11.
Ovdje ostaju samo dvije stavke koje M2 **ne** uzima:

| stavka | za | bilješka |
|---|---|---|
| Jedinice i natpisi **CLI-tablice**: udjeli u %, prijevod `Closed/Running/Planned`, „nema faza" umjesto praznog naslova, širina stupca | M3 | M3; sučelje to rješava u specu M2 §6.3, tablica u terminalu je pomoć — JSON je ugovor i on je točan |
| `include_unmerged`: mrtvo polje profila — ulogu ima `branch_scope` (S-032), a ime mu proturječi ponašanju (zadano `false`, a nespojene grane se mjere) | M3 ili odluka | odgođena 7 M1 + nalaz m13 završne recenzije 1.0.0: brisanje lomi profile koji ga navode (`deny_unknown_fields`), pa traži odluku |

## Iz završne recenzije 1.0.0 (T63, 2026-09-28) — presuda „čeka"

Izvor: izvještaj završne recenzije `.superpowers/sdd/2026-09-24-1-0-0-grane-i-ploca/task-63-review.md`
(radni zapis izvan gita) — Minor nalazi m2–m17 i odjeljak „Trijaža odgođenih" (broj = redak ledgera).
Svi su **kandidati za M3**, ne odluke; nijedan ne gubi podatke ni ne laže u brojkama jezgre. Što od
toga danas vidi korisnik: [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11.
Stavke koje su već drugdje u ovom dokumentu ovdje se ne ponavljaju (`--table` bez `until`, klik na
obavijest, tekst grafova raste s prozorom, dani bez commita, `include_unmerged`).

| izvor | stavka | mjesto |
|---|---|---|
| m2 | pokazatelj zatvorenih faza u razdoblju gleda samo `since`, ne `until` | `core/src/metrics/indicators.rs` |
| m4 | „{n} stabala" bez hrvatske množine („1 stabala") | `apps/desktop/src/lib/i18n/hr.json` `overview.worktrees` |
| m5 · 1744 | jedan commit okida 1–3 `report_updated` (višak izračuna i snimki, nije petlja); uzrok mjeriti ispisom sirove putanje u `debounce_loop` | `io/src/watch.rs` |
| m6 | greška izračuna ide samo na stderr; kartica projekta s pokvarenim profilom trajno piše „još nije izračunano" | `desktop/src-tauri/src/engine.rs`, `commands.rs::list_projects` |
| m8 · 988 | `Store::open` kroz `expect`; migracija i `schema_version` nisu u istoj transakciji; druga instanca otvara bazu prije odbijanja, bez `busy_timeout` | `desktop/src-tauri/src/lib.rs`, `store/src/store.rs` |
| m9 | `MockApi` i cijeli insta snapshot u produkcijskom `main.js` (dinamički `import()` u `createApi`) | `apps/desktop/src/lib/api.ts` |
| m10 · 1087 | dokaz signala i Rust greške su hrvatski i u EN sučelju | `core/src/rules/*.rs`, `engine.rs` |
| m11 · 595 | decimalna točka na osima grafova u HR sučelju | `apps/desktop/src/lib/charts/layout.ts` |
| m12 | zaglavlja datoteka postala su dnevnik cigli (`git.rs`, `report.rs`, `io/error.rs`, `App.svelte`, `scales.ts`, `lib.rs`, `phases.rs`, `Sidebar.svelte`, `settings.rs`, `read-tokens.mjs`) — prijedlog: zaglavlje = konstrukti koji su SADA u datoteci, povijest u `git log` | više datoteka |
| m14 | keš commita učitava sve keširane commite pri svakom izračunu, i nedostižne — raste bez granice; mjeriti kad naraste | `io/src/cache.rs` |
| m15 | odjava/gašenje Windowsa uz `prevent_close` — nije izmjereno | `desktop/src-tauri/src/lib.rs` |
| m16 · 967 | autostart se čita iz baze, ne iz OS-a; rub „plugin uspije, baza padne" (R12) | `desktop/src-tauri/src/autostart.rs`, `commands.rs::get_settings` |
| m17 | nova radna stabla watcher nadzire tek nakon ponovnog pokretanja | `engine.rs::watch` |
| — | instalacija preko starije verzije i deinstalacija: mapa instalacije i mapa baze su ista (`%LOCALAPPDATA%\sokratis`); **mjeriti** da nadogradnja čuva registar i što deinstalacija briše | `tauri.conf.json`, `state.rs::db_path` |
| — | CLI `report` nad 11 grana ≈ 5 s (raste s obujmom; desktop ide kroz keš); mjeriti bez opterećenja stroja | `crates/sokratis-cli` |
| 79 | `write_atomic` ne čisti `.tmp` kad `rename` padne | `io/src/project.rs` |
| 80 | `IoError::Encode` bez `path` | `io/src/error.rs` |
| 81 | nema testa „novi najveći ključ" u pisanju overridea | `io/tests/write.rs` |
| 86 | `inside_root("docs:hidden")` (NTFS ADS) prolazi — ne izlazi iz korijena | `core/src/profile.rs` |
| 90 | `PathBuf → TEXT` kroz `to_string_lossy` (ne-UTF-8 putanje) | `store/src/registry.rs` |
| 140 · 141 | komentari u `tokens.css` („specifičnost" umjesto cascade layera, „≈190°") | `apps/desktop/src/styles/tokens.css` |
| 154 | `branches_per_ref` (git < 2.41) bez testa | `io/src/git.rs` |
| 164 | `i18n.test.ts`/`smoke.test.ts` bez zaglavlja | `apps/desktop/tests/` |
| 165 | `check-i18n` ne uspoređuje `{parametre}` (pokriva vitest) | `apps/desktop/scripts/check-i18n.mjs` |
| 177 · 178 | `kinds.rs`: testni `row_commit` duplira `c`; lanac od 6 koraka | `core/src/metrics/kinds.rs` |
| 195 · 196 · 197 | bez testa: `num` s 4 znamenke, `kindLabel`/`subLabel`; `ymd` ne provjerava raspon | `apps/desktop/src/lib/format.ts` |
| 208 | ne-ASCII test ne ide kroz `Project::docs()` | `io/tests/project.rs` |
| 226 | nema testa `input.branches` za detached + bez zadane grane | `io/tests/project.rs` |
| 235 | `ringSegments` mutira `let a` u `map` (stil) | `apps/desktop/src/lib/charts/scales.ts` |
| 240 · 241 · 242 | snapshot bez testa za: `docs: None`; id u `cur` a ne u `prev`; Info → Alert | `core/src/snapshot.rs` |
| 256 | `Sparkline` s `NaN` elementom crta točku s `NaN cy` | `apps/desktop/src/lib/charts/Sparkline.svelte` |
| 258 | `Ring` s praznim natpisima → ime „, " | `apps/desktop/src/lib/charts/Ring.svelte` |
| 274 | `save_snapshot` ne provjerava vlasnika `profile_seen_id` | `store/src/snapshots.rs` |
| 275 | put `BadIndicatorKind` bez testa | `store/src/snapshots.rs` |
| 301 | snimka bez `ON CONFLICT` za dupli/rezervirani `id` | `store/src/snapshots.rs` |
| 405 | splash bez `onDestroy` | `apps/desktop/src/splash/Splash.svelte` |
| 417 · 418 · 421 | watcher: `p.starts_with(raw)` potiskuje pretka (2 s, bezopasno); nema pozitivnog testa za `Manual`; let-chain u `debounce_loop` | `io/src/watch.rs` |
| 512 | `bricksPerDay` računa u sučelju (odstupanje od S-012; kartica to kaže) | `apps/desktop/src/views/helpers.ts` |
| 516 | `types.test.ts` veže samo najgornju razinu `Report`-a (i ključevi `CommitRow`/`BranchStats`/`Touched`) | `apps/desktop/tests/types.test.ts` |
| 520 · 521 · 1235 | pristupačnost: `Sidebar` `<nav aria-label>` = natpis prve stavke; `Topbar` `aria-label` na `<div>` bez uloge; `Settings` `aria-label` duplicira natpis | `apps/desktop/src/lib/shell/`, `views/Settings.svelte` |
| 522 | `MockApi.setSetting` nije generički | `apps/desktop/src/lib/api.ts` |
| 535 | komentar o rezervi `until+2` u `parse/` nije provjeren | `core/src/parse/` |
| 564 | `SignalBar` uvozi iz `views/helpers` | `apps/desktop/src/lib/shell/SignalBar.svelte` |
| 602 · 604 | `perf.rs`: `warm_input = plain.clone()`; nema testa za prazan prozor | `io/tests/perf.rs` |
| 611 | `IndicatorsSection` `$effect` suvišan krug | `apps/desktop/src/views/project/IndicatorsSection.svelte` |
| 767 | stupac „model" u Isporukama nosi ostatak zaglavlja dnevnika | `core/src/parse/diary.rs` |
| 781 · 1086 | `helpers.test.ts` i `MockApi.root_path` nose stvarnu putanju s korisničkim imenom (zamjena `C:\repo`) | `apps/desktop/tests/helpers.test.ts`, `lib/api.ts` |
| 787 | zaglavlje Isporuka o `truncate` | `apps/desktop/src/views/project/DeliveriesSection.svelte` |
| 788 | potvrda brisanja vizije prelijeva 13 px na 960 px | `apps/desktop/src/views/Visions.svelte` |
| 866 | `project_worktree.branch` se puni praznim tekstom (R6) | `desktop/src-tauri/src/commands.rs` |
| 888 | komentar `today()` citira S-013 | `io/src/project.rs` |
| 907 · 944 | zaglavlja `src-tauri/cache.rs` (`StoreCache::lock`) i `engine.rs` („Sender ostaje u Watcher-u") | `desktop/src-tauri/src/` |
| 926 · 927 | `rule_title`/`docs_dirs` bez testa; `docs_dirs` ne deduplicira | `desktop/src-tauri/src/engine.rs` |
| 1085 | „zadnji pobjeđuje": nema testa „stariji zahtjev padne kasnije" | `apps/desktop/src/lib/state.svelte.ts` |
| 1102 | test `all(commits == 0)` prazno istinit — jasnije `phases.is_empty()` | `core/src/report.rs` |
| 1250 · 1251 | `Ring` `stroke-width={18 + 2}`; utrka `unlistenFocus` | `Ring.svelte`, `App.svelte` |
| 1293 · 1314 | kartice: `ind.test_share.how` („dijeljenje se preskače", stvarno `max(1.0)`); `deliveries.list.how` „diary.rs" bez „parse/" | `apps/desktop/src/lib/i18n/*.json` |
| 1454b · 1592 | komentar iznad `groupBy`; `DEFAULT_MARGINS` nije izvezen | `lib/charts/bucket.ts`, `Gantt.svelte`, `HBars.svelte` |
| 1520 | `Bars` `role="button"` bez tipkovne radnje; `Tooltip role="status"` | `lib/charts/Bars.svelte`, `Tooltip.svelte` |
| 1678 · 1679 | `chains.rs` obrnut komparator bez komentara; `perf.rs` izgubljena rečenica o `var_os` | `core/src/chains.rs`, `io/tests/perf.rs` |
| 1696 | traka ≈ 1 px iznad ljepljivog izbornika (subpikselni rub) — mjeriti | `apps/desktop/src/views/Project.svelte` |

## Za Leona (tuđi repo, ne naš posao)

| stavka | bilješka |
|---|---|
| `sokratstudy.dev/scripts/rad-xlsx.py`: dodati ` 00:00` uz `--since` (retci 154, 178) | S-011 — bez sata `git log --since` uzima trenutno doba dana; `RAD.xlsx` zato ovisi o satu pokretanja skripte, dnevni zadatak u 23:45 gubi gotovo cijeli tekući dan |

## Nakon 1.0.0 — vanjska lista 2026-09-24 (Leon, 2026-09-25: **ništa od ovoga ne ulazi u 1.0.0**)

Lista prijedloga iz vanjske analize (nije vidjela kod uživo), provjerena prema kodu; redoslijed po
omjeru učinka i cijene je orkestratorov prijedlog, Leon ga nije mijenjao. Vrijedi tek nakon prvih
stranih korisnika — oni će promijeniti redoslijed.

| # | stavka | što kod danas stvarno radi | bilješka |
|---|---|---|---|
| 1 | više signala s dokazom, prazna stanja koja uče | dva pravila (`unmerged_branches`, `docs_lag`); prazna stanja su goli natpisi (`tempo.empty`) | najjeftinije, pogađa jezgru vrijednosti; pravila iz „Čeka milestone" gore; prazno stanje = „Sokratis čita X iz Y; primjer retka" (obrazac S-027) |
| 2 | profil za tuđe projekte kao **preset**, ne čarobnjak | klasifikator VEĆ hvata `^fix`, `^docs`, `^ci:`, `^test:`, `refactor`; ali zadani `since` je **`2026-08-29`** (početak pariteta) → stranac s dvogodišnjim projektom vidi tri tjedna | `since` = pametna zadana vrijednost (prvi commit ili 90 dana) · `"preset": "conventional"` · detekcija dnevnika po obliku naslova; čarobnjak s UI-jem tek ako preset ne bude dovoljan (srodno: red „Profil za tuđe projekte" gore) |
| 3 | licenca | **odlučeno 2026-09-29 (S-039):** besplatno korištenje, bez izmjena i dijeljenja | prijedlog liste (MIT/Apache-2.0 ili AGPL-3.0) nije uzet |
| 4 | prvi strani korisnik | — | Leonov prijatelj s 1.0.0; jeftinije od bilo koje cigle |
| 5 | sati koji ne lažu | pragovi `session_gap_hours`/`session_start_hours` SU polja profila, nema sučelja; proxy označen (S-007) | prvo ručna korekcija po danu (mehanika kao `overrides.json`), pa senzor mtime-a |
| 6 | izvoz | CLI već daje JSON | JSON/CSV = gumb + dijalog; PNG grafa kasnije |
| 7 | tjedni izvještaj, „ovaj tjedan vs. prošli" | računljivo iz `days` bez snimki | usporedba projekata = zajednički graf (red gore) |
| 8 | cross-platform | `CREATE_NO_WINDOW` je `#[cfg(windows)]`, autostart kroz `tauri-plugin-autostart` (sva tri OS-a), NSIS je samo odabrani bundle-cilj (S-029) | trošak: CI-matrica + `.dmg`/`.AppImage` + testovi putanja; nema Mac/Linux stroja za ručnu provjeru |
| 9 | automatsko ažuriranje | instalater ručno | `tauri-plugin-updater` + GitHub Release `latest.json` + potpisni ključ; ide s prvim tagom (red „Potpisan instalater" gore) |
| 10 | performanse na velikom repou | mjereno ≈ 330 commita u prozoru; keš po SHA-i i `--since` prozor bi trebali držati | jedan `#[ignore]` test nad velikim javnim repoom (50 000 commita, 30 grana) — „trebali" nije mjerenje |
| 11 | engleski `docs/` | README EN (S-030), sučelje HR/EN; `docs/` HR po odluci i zbog učenja Rusta | najviše: engleski sažetak `ARCHITECTURE.md`; puni prijevod ne |
| 12 | izvori izvan gita (editor, PR-ovi, CI) | — | lokalno čitanje editorskih podataka ne ruši „bez oblaka, bez računa"; GitHub API ruši → svjesna odluka, ne nuspojava |
| 13 | signal „živa grana predugo izvan zadane" | pravilo gleda starost od ZADNJEG commita, pa `feat/f6-mcp` (108 ispred, tjednima) šuti | starost od točke grananja; nusprodukt S-038 |
| 14 | pravilo docs-a „citirana brojka/verzija u `.md` = izvor istine" (`Cargo.toml`, `package.json`, git) | `docs` provjerava strukturu, ne tvrdnje — Sokrat Study 100/100, a vlastiti `README.md` je tvrdio `pre.1` uz `pre.3` u kodu | jezgra ne čita `Cargo.toml` — `io` predaje izvore kao tekst, pravilo uspoređuje (S-010 kao pravilo u alatu, ne samo načelo); nitko drugi to ne mjeri |

## Nakon 1.0.0 — nalaz sesije 4 drugog reza (2026-09-26, Leon odlučuje)

- **Repo čija je glavna mapa označena `core.bare = true` a ima radne datoteke** (Sokrat Study
  `sokratstudy.dev` 2026-09-26 03:17–04:35, privremeno): `Project::open` nad njom pada (`fatal: this operation must be run
  in a work tree`), `worktree_heads` preskače blok bez retka `HEAD`, dnevnik s diska te mape se ne čita
  — treba li Sokratis to prepoznati i javiti (signal/kartica) ili je to Leonova okolina? Bez rješenja
  danas; T52 je zato mjerio nad povezanim radnim stablom `sokratstudy.f6` (isti `git-common-dir`,
  S-015). Detalji: `records/PROGRESS.md` (2026-09-26).

## Ideje, bez datuma

| stavka | bilješka |
|---|---|
| Pogled s telefona na LAN-u (lokalni servis + PWA) | razmatrano kao ljuska C; nije odabrano, ali jezgra to ne sprječava |
| Snimke ocjena kroz vrijeme kao graf (trend docs-čistoće) | → spec M2 [§4.2](../archive/ARHITEKTURA_M2.md) (S-014); samo pointer |
| `gix` umjesto `git` procesa | tek kad mjerenje kaže da je sporo (S-003) |
| Vektorizacija znaka (danas raster WebP/PNG iz Leonove datoteke) | kad instalater ili mala ikona to zatraže (tray je uklonjen, S-036) |
| Otvaranje nalaza dokumentacije u editoru (danas klik kopira putanju) | M3; traži `tauri-plugin-opener` — jedna ovisnost više |
| Dani bez commita nemaju prazan stupac u grafu stupaca | `Report.days` ih ne sadrži — graf preskače rupu umjesto da je crta praznu |
| Tekst u grafovima raste sa širinom prozora (`viewBox` 600) | graf se rasteže na širinu prozora zajedno s tekstom u sebi — na 1300 px oznake osi (≈ 17 px) veće su od naslova grafa (14 px) i od teksta tablica |

## Odbijeno (s razlogom)

| stavka | razlog |
|---|---|
| C++ | S-001: ekosustav, UTF-8 na Windowsu, ljuska |
| Rust GUI (Dioxus, Slint, egui) | S-006: tokeni i teme su CSS; isti izgled je cilj |
| Electron | RAM i veličina za aplikaciju koja radi cijeli dan; jezgra bi svejedno bila ista |
| Oblak / računi / Supabase | PRD §5: sve lokalno. *(2026-09-21: vrijedi za 1.0.0 i drugu verziju; timovi preko korisnikova vlastitog servera su u „Kasnije“)* |
| Štoperica za sate | PRD §5: sati ostaju git-proxy, označen kao proxy |
| Dnevni zadatak koji „bilježi" (kao `rad-dnevno.ps1`) | git je izvor istine; izvodi se na zahtjev |
