# TESTING — kako dokazujemo da radi

> Preuzeto iz Sokrat Studyja ono što vrijedi za Rust CLI bez ekrana. Sučelje M2 se testira `vitest`-om
> i `svelte-check`-om (**ne** Playwrightom — spec [ARHITEKTURA_M2.md](../archive/ARHITEKTURA_M2.md) §9);
> što se od toga danas može pokrenuti piše u tablici ispod.

## 1 · Vrste testova

| vrsta | gdje | što tvrdi | ulaz |
|---|---|---|---|
| **jedinični (core)** | `crates/sokratis-core/src/**` uz kod (`#[cfg(test)]`) i `tests/` (uklj. **od 2026-09-24** `tests/branches.rs` — sati po udjelu commita grane, S-032; `tests/diary_union.rs` — unija dnevnika po (datum, naslov), S-033) | parser vraća točno ove strukture; metrika daje točno ovu brojku; pravilo daje točno ovaj signal s dokazom | **tekst fixture** — nikad živi git |
| **lanci nespojenih grana (core)** — *od T64, 2026-09-27* | `crates/sokratis-core/src/chains.rs` (`#[cfg(test)]`: `parse_parents`, `assign_containment` nad fixture-grafom `a ⊂ b ⊂ c ⊂ d` + odvojena `e`), `crates/sokratis-core/src/rules/unmerged_branches.rs` (`#[cfg(test)]`: pravilo broji SAMO vrhove, dokaz nabraja sadržane s hrvatskim paucalom) | sadržanost je deterministička neovisno o redoslijedu grana; pravilo ne broji sadržanu granu ni u prag ni u prosjek starosti; vrh mlađi od `unmerged_warn_days` utišava cijeli lanac | **tekst fixture** (`%H\|%P`) — isti razlog kao ostatak core-a |
| **integracijski (io)** | `crates/sokratis-io/tests/` | git-proces, radna stabla (**od T49/T50, 2026-09-24** `tests/common/mod.rs` += `add_worktree`/`commit_at`, uklj. detached stablo bez retka `branch` u porcelainu), profil, ručni podaci; **od T64 (2026-09-27)** `tests/project.rs` dokazuje da se `branch_graph` puni SAMO kad postoji nespojena grana osim zadane (repo s jednom granom ne plaća proces); `tests/perf.rs` tvrdi GORNJU GRANICU git-procesa po izvještaju (brojka: `../records/CHANGELOG.md`, S-010); **od T52 (2026-09-26)** isti fajl += `measure_real_repo_both_scopes` — `#[ignore]` jer ovisi o pravom repozitoriju na disku, ručno se pokreće `SOKRATIS_MEASURE_REPO=<putanja> cargo test -p sokratis-io --test perf -- --ignored --nocapture` (naredba u doc-komentaru cigle), brojka procesa/trajanja: `../records/CHANGELOG.md` | **privremeni repo** stvoren u testu; `measure_real_repo_both_scopes` čita **pravi** repozitorij s diska |
| **izlazni kodovi (cli)** | `crates/sokratis-cli/tests/cli.rs` | ugovor prema preflightu: **0** (nema signala) i **2** (Alert) nad repoom s poznatom poviješću, **3** za pogrešnu uporabu i za putanju koja nije repozitorij, **0** za `--help`/`--version`; **od T51 (2026-09-26)** += `--scope`: nepoznata vrijednost je pogrešna uporaba (**3**), `--scope default` daje isto ponašanje kao `1.0.0-pre.1` (`scope_default_matches_pre_1_0_behaviour`) | **privremeni repo** (`tests/common/mod.rs`) |
| **pohrana (store)** — *od M2* | `crates/sokratis-store/src/**` (`#[cfg(test)]`) | migracije se primijene od prazne baze; registar, postavke, snimke i keš sirovih commita (M2/15–18) rade; od DESKTOP-a (T29–T30) imaju pravog pozivatelja u aplikaciji, ali test i dalje ostaje nad `:memory:` bazom (`docs/architecture/ARCHITECTURE.md` §1) | **`:memory:` baza** (`Store::open_in_memory`) — bez gita i bez datoteka |
| **sučelje (TS/Svelte)** — *od M2* | `apps/desktop/tests/**`, uz komponente | oblikovanje brojki i lokalni „danas" (`tests/format.test.ts`, `localYmd` od `M2/63`), i18n ključevi (hr = en), kontrast tokena, grafovi s praznim ulazom, ugovor `TauriApi` prema `commands.rs` (`tests/tauri-api.test.ts`, lažni `invoke`/`listen`, **od 2026-09-24 uklj. `quit`/`onCloseRequested`**), verzija iz jednog izvora (`tests/version.test.ts`), pokret bez DOM-a (`tests/motion.test.ts`, čita `motion.css` kroz `scripts/read-motion.mjs`), pokrivenost kartice s objašnjenjem (`tests/explain.test.ts` — svaki `id` ima sva tri ključa u oba jezika, svaki ključ `explain.*` ima svoj `id`, `EXPLAIN_IDS.length === 44` kao regresijska brana, 18 id-eva pokazatelja iz prave snimke jezgre), matematika osi grafova bez DOM-a (`tests/scales.test.ts`, `tests/bucket.test.ts`, od T53; **od `M2/62`** += `dayLevelTicks` — korak u cijelim danima za dnevni format, raspon od jedne točke daje točno jedan tick bez ponavljanja), **raspored grafova bez DOM-a** (`tests/layout.test.ts`, od T54/T55, **prošireno T56/T57 na 6 rasporeda**: okvir, stupci, linija, toplinska karta, histogram, gantt, vodoravni stupci — **stari `tests/scale.test.ts` je nestao**, njegovi testovi su preseljeni u `scales.test.ts`/`layout.test.ts` uz kod koji sada testiraju, R51/R52; **od `M2/62`** += `fitLabel`/`shortLabel` (natpis retka kraćen na „…", pun naziv u `label`), `XTick.anchor` (zadnja oznaka `'end'` kad bi izašla iz okvira), test da nijedna koordinata nije `NaN` nad rubnim rasporedima), **od T58** `tests/helpers.test.ts` += pokrivenost `SECTIONS`/`signalCounts` (ploča projekta), **od T59** += `kindSeries` (`views/helpers.ts`, pet vrsta rada → `Series[]` za naslagani graf) | `npm run check` u `apps/desktop`: `svelte-check` + `check:i18n` + `check:contrast` + `vitest` (brojke: `CHANGELOG.md`) |
| **ljuska (desktop)** — *od M2* | `apps/desktop/src-tauri/src/{commands,state,summary}.rs` (`#[cfg(test)]`) | S-013: crate nema logike koju bi vrijedilo testirati, pa su testovi samo čiste pretvorbe: birač raspona (`Range`, JSON oblik i `to_dates`), ime datoteke baze po gradnji, lanac uzroka greške u tekstu za sučelje (`state::text`, `M2/63`), broj stabala iz izvještaja ili registra (`summary::summarize`, `M2/63`) | fiksni „danas", ručno složene greške i minimalan `Report` iz `build_report` |

**Tekuće brojke brana (fmt/clippy/test/svelte-check/i18n/vitest/build) nikad se ne prepisuju ovamo**
(S-010) — kanonske su u [`../records/CHANGELOG.md`](../records/CHANGELOG.md), po svakoj spojenoj
cigli; tijek sesija u [`../records/PROGRESS.md`](../records/PROGRESS.md).

`scripts/read-motion.mjs` čita `motion.css` kroz `node:fs` umjesto Viteova `?raw` uvoza — isti razlog
kao `scripts/read-tokens.mjs` (M2/20): `@tailwindcss/vite` hvata svaki `.css` uvoz i vraća prazan
modul kad datoteka nema vlastiti `@import "tailwindcss"`.

**Test-prvo** (CLAUDE.md #7): fixture → očekivano → implementacija. Rub koji prepoznaš odmah dobiva test.

**Snapshot `Report`-a** je ugovor prema sučelju (S-022): `insta` je **dev-ovisnost `core`-a od
`M2/1a`**, a sam test postoji od cigle M2/2 (2026-09-18) —
`crates/sokratis-core/tests/snapshot.rs` + `tests/snapshots/snapshot__report-sokratstudy-2026-09-17.snap`
(po jedan ključ na vrhu za svako polje `Report`-a). Svaka promjena oblika JSON-a mora biti **namjerna**:
`INSTA_UPDATE=always cargo test -p sokratis-core --test snapshot` prepiše snimku, `git diff` pokaže
točno što se promijenilo, a rečenica u commitu kaže koje polje i zašto. Provjeri prije `git add` da
nije ostao `.snap.new` (insta ga ostavi kad snimka još ne postoji ili se pending-diff nije razriješio).

## 2 · Fixture-politika

- **Core testira tekst.** `git log` izlaz se snimi u `tests/fixtures/*.log` jednom, tekstualno, i
  više se ne mijenja. Isto za `PROGRESS.md` i `RASPORED.md` snimke. Test koji ovisi o živom repou
  nije test nego lutrija.
- **Fixture je mala i imenovana po onome što dokazuje:** `hours-cherry-pick.log`, ne `test1.log`.
- **Privremeni repo za io:** `git init` u `tempdir`, commiti s fiksnim vremenima:

  ```
  GIT_AUTHOR_DATE="2026-09-06T05:16:40+02:00" GIT_COMMITTER_DATE="2026-09-08T21:19:43+02:00" git commit -m "..."
  ```

  Tako se cherry-pick i rebase reproduciraju deterministički.
- **Rub koji OS ne dopušta ide u unit test parsera, ne u privremeni repo** (R49, T49/2026-09-24):
  grana s imenom `y|z` ne može se stvoriti na Windowsu/NTFS-u (`|` je rezerviran znak), pa
  `parse_commit_sources` (dijeli `git log --format=%h|%S` na PRVOM `|`) dokazuje taj rub tekstom
  (`"abc123|y|z"`), ne stvarnom granom u fixture-repou.

## 3 · Test pariteta s `RAD.xlsx` (izlazni uvjet M1)

Dokazuje da Sokratis daje **iste brojke** kao Python skripta za isti ulaz, i da su **jedina**
razlika sati (ispravak S-007).

1. **Snimi ulaz** (jednom, iz `sokratstudy.dev` na `main`; od 2026-08-02 jer se zatvorene faze broje iz
   commita, a metrike filtriraju `commit_date >= 2026-08-29`):

   ```
   git log main --since="2026-08-02 00:00" --reverse --date=format:%Y-%m-%d --format=@@%h|%at|%ct|%ad|%cd|%s --numstat > tests/fixtures/sokratstudy-2026-09-17.log
   ```

   ⚠️ ` 00:00` je obavezan (S-011): bez sata `git log --since` uzima trenutno doba dana, pa bi
   ponovna snimka pomaknula lijevi rub i prvu zatvorenu fazu. Zašto je snimka takva kakva je i što se
   promijenilo prema staroj: `crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.README.md`.

   plus `git show main:docs/records/PROGRESS.md` i `git show main:docs/plan/RASPORED.md` s istog commita
   (SHA u `.sha` datoteci). Točan postupak: plan M1, cigla T2.
2. **Snimi očekivano** iz lista Sažetak knjige koju `rad-xlsx.py` generira nad ISTIM stanjem (kopija skripte
   sa svježim izlazom, bez ručnih overridea): tempo po danu, vrste, 18 pokazatelja, faze — u
   `tests/fixtures/sokratstudy-2026-09-17.expected.json`.
3. **Test tvrdi:** sve jednako **osim** `hours` po danu i ukupno; za sate tvrdi novu vrijednost i
   u komentaru navodi staru (−144,1 h) i razlog.

⚠️ Fixture se snima s **točno onog commita** s kojeg je tablica generirana; inače paritet uspoređuje
dva različita svijeta i pada iz krivog razloga.

## 4 · Brane prije commita

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Cigla koja dira `apps/desktop` (tokovi SUČELJE i DESKTOP) uz to vrti **`npm run check`** u
`apps/desktop` — `npm install` je odrađen 2026-09-18 (`M2/1b`), `node_modules` postoji i brana je
zelena (svelte-check 0 grešaka + vitest). Rust-tokovi u svojim stablima vrte brane **bez** desktop
cratea; razlog i naredba: [`AGENTI.md`](./AGENTI.md) §5.

Crveno ne ide u commit. **Izlazni kod 1 nije dokaz da je pao test koji testiraš** — čita se poruka
(pouka iz Sokrat Studyja). CI (GitHub Actions) još ne postoji — `records/BACKLOG.md`, M4. Koliko testova ima
danas piše [`../records/CHANGELOG.md`](../records/CHANGELOG.md) — brojka prepisana ovamo bi ostarila.

**Nakon SVAKE gradnje iz drugog radnog stabla u ZAJEDNIČKI `target`** (dimni test s
`CARGO_TARGET_DIR` postavljenim na `target` glavnog repoa, ili mjerenje R33 preko klona) — prije
punih brana na `main`-u i prije gradnje instalatera: `cargo clean -p sokratis-core -p sokratis-io -p
sokratis-store -p sokratis-cli -p sokratis-desktop` (Ruling R78, 2026-09-28). Razlog: cargo ime
artefakta radnog prostora gradi iz putanje RELATIVNE prema korijenu, pa dva stabla dijele isto ime
(`libsokratis_core-<hash>`), a svježina se sudi po vremenu izmjene izvora — artefakt iz drugog stabla
može biti noviji od izvora u `main`-u i proći kao svjež iako je preveden protiv druge verzije koda.
`clippy` to ne vidi (koristi zasebne `check`-artefakte); vidljivo je samo u `cargo test --workspace`
(krive greške prevođenja) ili, gore, u tihom prolazu sa zastarjelom logikom. Tauri ovisnosti (velike,
sporo se prevode) ostaju netaknute — briše se samo pet naših crateova.

## 5 · Što se NE testira testom

- **Ljuska i izgled** — `sokratis-desktop` po S-013 gotovo nema logike (testovi su samo čiste
  pretvorbe, §1). Dokaz da se ljuska stvarno pokreće je **dimni test** `npm run tauri dev`, ručno
  pokrenut prije spajanja svake DESKTOP/INTEGRACIJA-cigle (spec M2 §9): splash prikazan i zatvoren
  točno jednom, glavni prozor se pojavi tek nakon animacije I prvog izračuna svih projekata, druga
  instanca podigne postojeći prozor umjesto da otvori novi, **X pokazuje upit „Zatvoriti Sokratis?" —
  Odustani ostavlja prozor, Zatvori gasi proces (S-036, tray uklonjen M2/45)**, svako pokretanje
  (uklj. iz autostarta) je nov proces pa se animacija pokretanja prikazuje **svaki put**.
- **Od T37 (2026-09-22) razvojna baza je odvojena od prave** (`state::db_file_name`,
  `cfg!(debug_assertions)`): `npm run tauri dev` piše u `sokratis-dev.db`, instalirana aplikacija u
  `sokratis.db` — razvoj više ne dira Leonovu pravu bazu. Dimni test INTEGRACIJE (S3, prije spajanja)
  je unatoč tome i dalje preusmjeravao `LOCALAPPDATA` (dodatna sigurnost) i čitao prozor kroz Chrome
  DevTools Protocol (`--remote-debugging-port`, nad vlastitim webviewom) umjesto snimke ekrana
  (`PrintWindow`, obrazac iz S2) — provjera DOM-a je pouzdanija od piksela.
- **Dimni test S4 (SUČELJE-2, T38–T42, 2026-09-23)** je prvi koji je morao raditi UZ instaliranu
  aplikaciju: `tauri dev --config identifier=dev.sokratis.app.dev` dobiva vlastiti identifikator, pa
  `single-instance` ne sudara dvije aplikacije; `LOCALAPPDATA` preusmjeren u scratchpad, CDP nad
  vlastitim webviewom (kao u S3). Dijalog za mapu bira PRVI proces imenom `sokratis-desktop` (a to je
  instalirana, ne dev) — pomoćna skripta odabire proces po PID-u kad oba rade istodobno.
- **Dimni test DESKTOP-2 (S1 izvedbe drugog reza, 2026-09-24)** je prvi nakon uklanjanja traya
  (M2/45, S-036): WM_CLOSE → dijalog „Zatvoriti Sokratis?" s fokusom na „Odustani"; Esc i klik
  „Odustani" ostavljaju prozor i proces živ; klik „Zatvori" → proces nestao za ~250 ms, instalirana
  verzija netaknuta; ponovno pokretanje → splash pa glavni prozor ≈ 5,7 s kasnije (animacija svaki
  put). „Osvježenje bez bljeska konzole" (kvar 4) se u `tauri dev` ne da izmjeriti — debug build je
  konzolni proces pa djeca dijele konzolu s roditeljem; dokaz ostaje test izvora (T44,
  `command_new_lives_only_inside_git_command`) i Leonova ručna provjera na instaliranoj verziji.
- **Vizualni dimni test Tempa (sesija 3 izvedbe drugog reza, 2026-09-24, T55)** — grafovi se ne
  renderiraju u vitestu (R37, nema jsdom-a): `vite` dev-server + preglednik, `MockApi`. Provjereno
  ručno na instaliranoj verziji: datumi na X-osi, mreža, tooltip na najbližem stupcu/točki, stupci
  fokusabilni tipkovnicom — prošao.
- **Vizualni dimni test ploče (sesija 4 izvedbe drugog reza, 2026-09-26, T58)** — isti razlog kao gore
  (grafovi/DOM izvan vitesta), ovaj put alatom orkestratora `vite` + Playwright nad `MockApi`, **bez
  Tauri-ja** (dovoljno za provjeru izgleda/rasporeda ploče, ne treba pravi prozor). Provjereno: Pregled
  → klik na karticu → ploča Projekt s 8 sekcija; ljepljivi skok-izbornik; „‹ Pregled" u gornjoj traci;
  stavke izbornika (Projekt/Dnevnik/Vizije) zasivljene bez odabranog projekta; šest kartica Sažetka bez
  vodoravnog klizača; tablica Grane s natpisom „Mjerena je samo zadana grana." kad je opseg `default`
  — prošao (nakon popravka 1: sidra ispod izbornika, kartice Sažetka od `xl`).
- **Dimni test ploče nad PRAVIM podacima (sesija 5→6 izvedbe drugog reza, 2026-09-27/28, T62)** —
  `npm run tauri dev` nad KLONOM Sokrat Studyja (ne pravi repo — samo se čita: sha256 `.git/config` +
  `.git/HEAD` i `for-each-ref` prije = poslije kloniranja), dva dodatna radna stabla klona (grane u
  radu), vlastiti identifikator aplikacije i zasebna baza (`LOCALAPPDATA` preusmjeren), `git worktree
  add`. Mjerenje kroz Chrome DevTools Protocol (`--remote-debugging-port`) nad vlastitim webviewom —
  isti obrazac kao S3/S4. Provjereno po svakoj sekciji ploče: broj `svg[role=img]` (grafovi +
  sparklinei) i vidljivih `<h3>` naslova, oznake X-osi na rubovima raspona, prekidač dan/tjedan/mjesec,
  legenda, tooltip na `pointermove`, sekcija Grane s više od jedne grane, sažetak i Dnevnik sa stupcem
  grane, sve četiri teme × `data-motion="off"` na dvije širine. Ovaj test je izvor nalaza **G1–G3**
  (krug popravaka `M2/62`, §1 iznad) — dimni test nad pravim podacima našao je rubove koje `MockApi`
  (fiksni fixture) ne pokriva (dugi nazivi faza, gust raspon datuma).
- **Vizualna provjera u pregledniku prije spajanja svake cigle koja dira izgled ploče** (od T58) —
  `vite` dev-server + preglednik ili Playwright, nad `MockApi`, **bez Tauri-ja**: brže od dimnog testa
  jer ne treba pravi prozor ni pravi repo, dovoljno za raspored/tekst/boje. Popravak `M2/62` je nakon
  krugova popravaka provjeren ovako: svježe učitavanje, klik na karticu + mjerenje u jednom
  `evaluate`, provjera da nijedan `<text>` ne izlazi iz okvira grafa na dvije širine prozora. Dimni test
  nad pravim podacima (gore) i vizualna provjera nad `MockApi`-jem hvataju RAZLIČITE rubove — obje idu
  prije spajanja cigle koja dira grafove.
- **Dimni test izdanja 1.0.0 (sesija 6, 2026-09-28, nalazi završne recenzije T63)** — isti postav kao
  T62 (`tauri dev`, klon pravog repoa, zasebna baza, CDP), mjereno PRIJE i POSLIJE kruga `M2/63`
  (pravilo #4): **glavna nit** se mjeri lakom naredbom (`get_settings`) poslanom usred teške
  (`get_report`) — koliko laka čeka kaže drži li teška glavnu nit; **zatvaranje usred izračuna** —
  poruka za zatvaranje prozora dok niz izračuna traje, dijalog potvrde mora biti u DOM-u prije nego
  ijedan završi; **prvi izračun novog projekta** — `last_refresh`/`last_commit` kartice nekoliko sekundi
  nakon dodavanja; **broj stabala** — kartica prema `touched.worktrees` nakon dodanog stabla u klonu.
  Brojke prije → poslije: `CHANGELOG.md`, unos `M2/63`. Vučenje prozora mišem nije mjereno (nema
  ulaznog uređaja) — zamjena je mjerenje glavne niti.
- **Na ručnoj listi za Leona ostaju** (`docs/architecture/ARCHITECTURE.md` §11 — nisu provjereni ni u
  S2 ni u S3 ni u S4): autostart u `HKCU\…\Run`, obavijest na prijelaz u
  Alert, preskok splasha, `prefers-reduced-motion`, osvježenje bez klika (sa štopericom), četiri teme,
  kartica s objašnjenjem vizualno (S4 ju je provjerio samo testovima i strukturom).
- **Performanse** — test tvrdi samo GORNJU GRANICU git-procesa po izvještaju (`io/tests/perf.rs`);
  trajanje se **mjeri**, ne testira: `#[ignore]` testovi nad pravim repozitorijem
  (`measure_real_repo_both_scopes`, `io/tests/cache.rs::measure_cached_input_over_a_real_repo`) i
  mjerenja CLI-ja i desktopa na izdanju — brojke su na jednom mjestu, `CHANGELOG.md` (`[1.0.0]`, unos
  `M2/35`). `gix` je odgovor koji i dalje čeka pitanje.
