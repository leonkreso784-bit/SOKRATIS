# DECISIONS — odluke i zašto

> Jedna odluka = jedan zapis: **kontekst → odluka → posljedice**. Ostali dokumenti linkaju `S-xxx`,
> ne prepričavaju. Odluka koja prestane vrijediti dobiva pečat ⚰️ s datumom, ne briše se.
> Najnovija na dnu.

---

## S-001 — Rust kao jezik cijele jezgre (2026-09-17)

**Kontekst:** Leon želi nov jezik koji nije koristio (Rust ili C++). Aplikacija parsira tekst,
računa metrike, drži SQLite, gleda disk i pakira se kao desktop program.
**Odluka:** Rust, stable, MSVC target. C++ odbijen: bez standardnog upravljanja paketima, UTF-8 na
Windowsu bolan (Python skripta je već pala na cp1252), desktop ljuska bi bila Qt ili WebView2 kroz COM.
Go + Wails zabilježen kao lakša rezerva, ne odabran.
**Posljedice:** M0 = instalacija toolchaina (nema ga na stroju). Učenje je cilj → pravilo „zašto Rust
ovako" (CLAUDE.md #5) i `workflow/RUST.md`.

## S-002 — `core` bez I/O-a (2026-09-17)

**Kontekst:** ista logika treba CLI-ju, Tauriju i testovima.
**Odluka:** `sokratis-core` prima tekst i strukture, vraća strukture; ne otvara datoteke, ne zove procese.
**Posljedice:** testovi rade nad tekst-fixtureima bez gita; `io` je jedino mjesto s vanjskim svijetom;
ownership bez lifetimeova prema van.

## S-003 — git kroz proces, iza traita `GitSource` (2026-09-17)

**Kontekst:** `git2` (libgit2) slabo podržava radna stabla; `gix` još evoluira; tablica je čitala
izlaz `git` naredbe.
**Odluka:** M1 poziva `git` binarnu (`std::process::Command`) i parsira tekst. Trait ostaje.
**Posljedice:** paritet je doslovan; nema nativnog builda; `git` mora biti na PATH-u (greška, izlaz 3).
`gix` čeka mjerenje koje kaže da je sporo (BACKLOG).

## S-004 — ručni podaci u `.sokratis/` u repou projekta (2026-09-17)

**Kontekst:** tablica je imala dva ručna podatka (override vrste po SHA, vizije) i putovala kroz git.
**Odluka:** `<repo>/.sokratis/overrides.json` i `visions.json`; profil isto ondje. Keš i snimke lokalno
(M2, SQLite), nikad u repou.
**Posljedice:** ručni podaci su verzionirani i prenosivi; drugi korisnik Sokratisa dobiva isto ponašanje
kloniranjem. Mapa je konvencija koju profil ne može preseliti.

## S-005 — zadano = konvencije Sokrat Studyja; profil pregazi (2026-09-17)

**Kontekst:** prvi korisnik je Sokrat Study; drugi projekti imaju drukčije dnevnike i planove.
**Odluka:** svi regexi, putanje i pragovi imaju zadane vrijednosti jednake Sokrat Studyju; `profile.json`
mijenja pojedina polja; nepoznato polje = greška.
**Posljedice:** Sokrat Study radi bez konfiguracije; tuđi projekt piše profil (M3 dokumentira format).

## S-006 — sučelje web (Svelte 5 + Tailwind v4 + tokeni), ne Rust GUI (2026-09-17)

**Kontekst:** cilj je isti izgled kao Sokrat Study; tokeni i četiri teme već postoje kao CSS.
Sokrat Study je framework odbio (ADR-028) zbog Service Workera i nepromjenjivog keša — toga u
desktop aplikaciji nema, a nadzorna ploča ima puno stanja.
**Odluka:** Tauri 2 s web sučeljem u Svelte 5; `tokens.css` se prenosi. Dioxus/Slint/egui odbijeni.
**Posljedice:** jedini dio izvan Rusta je sučelje; M2 spec bira Tailwind kroz Vite plugin ili CLI.

## S-007 — sati po `author_time`, sortirano uzlazno (2026-09-17)

**Kontekst:** tablica računa razmak po datumu autora, a redoslijed uzima iz git loga (datum commita);
cherry-pick/rebase daju negativan razmak (`5233a0a`, `5da7119`, `1e2d157` → −144,1 h).
**Odluka:** commiti se sortiraju po `author_time` prije proxyja; pragovi 2 h / 0,5 h ostaju.
**Posljedice:** paritet s tablicom se u satima **namjerno** ne održava; test to tvrdi izričito.

## S-008 — engleski identifikatori u jezgri, natpisi u sučelju (2026-09-17)

**Kontekst:** projekt se objavljuje; Leon piše hrvatski.
**Odluka:** kod, JSON ključevi i vrijednosti enum-a su engleski (`work_kind = "debugging"`); hrvatske i
engleske natpise daje sučelje/CLI tablica.
**Posljedice:** jezgra je objavljiva bez prijevoda; HR/EN je posao sučelja od prvog dana; docs ostaju hrvatski.

## S-009 — SQLite tek u M2 (2026-09-17)

**Kontekst:** M1 je CLI koji sve izvodi iz gita na zahtjev; povijest ocjena treba tek ekranu.
**Odluka:** M1 bez pohrane osim `.sokratis/` datoteka; M2 uvodi SQLite za snimke i keš.
**Posljedice:** M1 je manji; „git je izvor istine" se dokazuje prije nego se uvede keš koji bi to mogao zamagliti.

## S-010 — jedna činjenica, jedno mjesto (2026-09-17)

**Kontekst:** preuzeto iz Sokrat Studyja (ADR-027): ista cigla pisana u četiri dokumenta pa usklađivana danima.
**Odluka:** što sustav radi = kod + testovi; zašto = ovdje; što vrijedi sad = `CLAUDE.md` + `product/` +
aktivni spec; što se dogodilo = `records/`. Duplikat se briše, ne sinkronizira; brojka u prozi pokazuje na izvor.
**Posljedice:** spec ne nosi odluke nego linka; `CLAUDE.md` ne nosi povijest; Sokratis to sam mjeri (dogfooding).

## S-011 — `--since` računa cijeli dan, ne golo `git log --since` (2026-09-17)

**Kontekst:** `git log --since=YYYY-MM-DD` bez sata koristi `approxidate` i uzima **trenutno doba dana**
kao granicu, ne ponoć. Izmjereno na `main`-u Sokrat Studyja: `git log --since=2026-08-29` u 17:15 vrati
183 commita, `--since='2026-08-29 00:00'` vrati 190. `rad-xlsx.py` (retci 154, 178) šalje goli datum →
`RAD.xlsx` ovisi o satu pokretanja skripte; dnevni zadatak u 23:45 zna izgubiti gotovo cijeli dan.
Isti kvar pogađa i fiksne raspone zatvorenih faza (npr. MREŽA 20→25 commita, RAČUN R1 0→6 commita kad
se doda puni dan).
**Odluka:** `io` šalje `git log` argument `--since=<datum> 00:00:00`; jezgra filtrira po
`commit_date >= since` na ponoć tog dana. Referentni `expected.json` u fixtureu je regeneriran istom
Python skriptom s dodanim `' 00:00'`, ne ručno prepravljen.
**Posljedice:** paritet sa `rad-xlsx.py` je u satima i danima namjeran (kao S-007) — kvar tablice se
**ispravlja, ne prenosi**. Alternativa (replicirati Pythonov approxidate-kvar radi doslovnog pariteta)
odbijena: paritet je test korisnosti, ne test bugova. Backlog: javiti Leonu da `sokratstudy.dev`
(tuđi repo, Leon odlučuje) doda ` 00:00` u `rad-xlsx.py` — `records/BACKLOG.md`.

**Dopuna (2026-09-17, nakon završne recenzije M1) — dvije stvari koje je spec prešutio:**

1. **Zatvorene faze filtriraju po `commit_date`,** ne po datumu autora. Dosljedno je i gitu
   (`--since`/`--until` gledaju committer-vrijeme) i Python-referenci, a cherry-pick obrazac koji bi
   to pomaknuo čuva test `closed_phases_filters_by_commit_date_not_author_date`
   (`core/src/metrics/phases.rs`). Sate i dalje računa `author_time` (S-007) — to su dvije različite
   mjere s dva različita razloga.
2. **Log se dovlači s rezervom od jednog dana** (`--since=<datum − 1> 00:00:00`). Sat je fiksiran na
   ponoć, ali ponoć je u zoni **stroja**, a datumi u logu su u zoni **commita**; bez rezerve bi stroj
   zapadnije od pohranjenog pomaka odbacio commite koje jezgrin filtar zadržava (prvi dan zatvorene
   faze prvi strada). Mjerodavan ostaje jezgrin `commit_date >= since`, pa rezerva ne mijenja ni
   jednu brojku — čini izvještaj neovisnim o stroju, što je uvjet za CI (M3).

---

## S-012 — Tauri ugovor = `Report` nepromijenjen; mjerenje u `core`, oblikovanje u Svelteu (2026-09-18)

**Kontekst:** M2 dodaje sučelje nad jezgrom iz M1. Tri pristupa: tanak Tauri koji vraća `Report`
kakav jest; debeo Tauri koji priprema gotove modele po ekranu; sve u desktop crateu.
**Odluka:** naredba `get_report` vraća **isti JSON koji ispisuje `sokratis report --json`**; jedini
nov oblik prema sučelju je tanak `ProjectSummary` za Pregled. Ono što je **mjerenje** (zbroj vizija po
stanju, razlika dviju snimki, `until`) ide u `core` i izlazi u `Report`; ono što je **oblikovanje**
(postoci, sati na decimalu, natpisi) radi Svelte u jednom modulu s testom. Debeo Tauri odbijen:
oblikovanje bi postojalo dvaput (tablica i ekran) i svaka promjena dizajna dirala bi Rust.
**Posljedice:** sučelje i terminal čitaju isti ugovor — kad se raziđu, zna se tko laže; snapshot
`Report`-a (S-022) čuva oba odjednom; CLI dobiva `--until` da svaku brojku s ekrana Leon može ponoviti.

## S-013 — nov crate `sokratis-store`; `desktop` bez logike (2026-09-18)

**Kontekst:** SQLite, registar projekata i watcher trebaju negdje živjeti. Kandidati: unutar `io`,
unutar Tauri cratea, ili nov crate.
**Odluka:** pohrana je nov crate `sokratis-store` koji ovisi o `core` (tipovi), ne o `io`; watcher ide
u `io` (to je I/O nad diskom); `apps/desktop/src-tauri` (crate `sokratis-desktop`) drži samo naredbe,
prozore, tray i splash — **ništa što bi se htjelo testirati**.
**Posljedice:** `store` se testira nad `:memory:` bazom bez gita; watcher se testira bez prozora
(javlja kroz `mpsc` kanal); CLI kasnije može posuditi registar (`sokratis projects`). Cijena: jedan
crate više u workspaceu.

## S-014 — što SQLite drži: istina koju git ne zna + pogodnost koja se izvodi iznova (2026-09-18)

**Kontekst:** S-009 je rekao „SQLite u M2", ne za što. PRD §6: git je izvor istine, izvještaj je
uvijek isti za isti git. Leon je odabrao registar + snimke + keš.
**Odluka:** baza `%LOCALAPPDATA%\sokratis\sokratis.db` drži (a) **istinu koju git ne zna**: registar
projekata, postavke, odabran raspon po projektu; (b) **pogodnost**: snimke brojki jednom dnevno
(`SnapshotMetrics`, ne cijeli `Report` — on se reproducira iz gita) uz zapis profila kao **kanonski
JSON** (ne hash: `DefaultHasher` nije stabilan među verzijama, a nova ovisnost samo za hash je pravilo
#6 naopako), i keš **sirovih** činjenica o commitu po SHA — nikad klasifikacije, jer ona ovisi o
regexima iz profila. **Keš ulazi tek ako mjerenje nakon M11 pokaže da treba** (pravilo #4); do tada
je tablica definirana i prazna. Zamjenjuje `%APPDATA%\sokratis\config.json` iz ocrta M1.
**Posljedice:** promjena profila je vidljiva kao oznaka u trendu, ne kao lažni lom krivulje; nikad
nema zapisa po commitu („dnevni zadatak koji bilježi" ostaje odbijen); brisanje baze gubi samo
registar i postavke, sve ostalo se izvodi iznova.

## S-015 — projekt = zajednički git-direktorij; ručni podaci se pišu u glavno stablo (2026-09-18)

**Kontekst:** Sokrat Study ima pet radnih stabala; `.sokratis/` je praćen u gitu, pa svako stablo
ima svoj primjerak koji se razilazi do spajanja. PRD kaže da su stabla jedan projekt.
**Odluka:** identitet projekta je `git rev-parse --git-common-dir`; dodavanje bilo kojeg stabla
grupira sva; `overrides.json` i `visions.json` se pišu **uvijek u glavno stablo** (ono uz `.git`),
atomarno (privremena datoteka + `rename`), u obliku kakav Leon piše rukom. **Sokratis nikad ne
commita sam** — upis je necommitana izmjena u `git status`. Alternative (stablo zadane grane; stablo
koje je dodano; pitati) odbijene: prebacuju se, nestaju, ili traže odluku u trenutku ispravka.
**Posljedice:** jedno mjesto za ručne podatke, bez razilaženja; `GitSource::worktrees` iz M1 dobiva
prvog potrošača; stablo obrisano izvan Sokratisa nestaje s popisa, glavno se ne mijenja samo.

## S-016 — watcher u `io`: `.git` + docs + `.sokratis`, odgoda 600 ms, bez petlje (2026-09-18)

**Kontekst:** aplikacija je stalno otvorena; Leon je odabrao osvježavanje watcherom + gumb.
Periodični tajmer i „samo ručno" odbijeni (stare brojke, prazan CPU-posao).
**Odluka:** `notify` u `io`, po projektu nadzire zajednički `.git` (`HEAD`, `refs/`, `logs/HEAD`,
`packed-refs`), dnevnik/plan/`docs/` u svakom stablu i `.sokratis/` u glavnom. Tri zaštite sa svojim
testovima: **odgoda 600 ms** sažima rafal jednog commita u jedan izračun; **vlastiti upisi su
potisnuti** 2 s da uređivanje vrste ne pokrene petlju; **izračun je serijski po projektu**, novi
događaj zamjenjuje čekanje. Javlja kroz `mpsc` kanal, ne zna za Tauri.
**Posljedice:** M11 (performanse) prestaje biti udobnost i postaje uvjet — 3,2 s u petlji je kvar;
cilj ispod 500 ms je mjerenje u testu, ne tvrdnja.

## S-017 — `tokens.css` preuzet cijel; `brand-*` iz loga, izmjeren; Tailwind kroz Vite plugin (2026-09-18)

**Kontekst:** S-006 traži isti izgled kao Sokrat Study; Sokratis ima svoj znak (cijan `#00dce8`,
ljubičasti čvorovi, tamni disk). Neonski cijan na bijeloj ima kontrast ≈1,7:1.
**Odluka:** struktura `tokens.css` (`@theme static`, semantička imena, brisanje zadane palete) i sve
četiri teme preuzimaju se kakve jesu; **mijenja se samo obitelj `brand-*`**, izvedena iz hue-a loga
(≈183°) tako da `brand-600/700` prolaze ≥ 4,5:1 na sve tri plohe svake svijetle teme, a neon ostaje
za tamne teme i sam znak. Provjera je skripta `check:contrast`, ne oko. Tailwind kroz
`@tailwindcss/vite` — Vite je ionako tu zbog Sveltea.
**Posljedice:** Sokratis izgleda kao rođak Sokrat Studyja, ne klon; svaka boja u markupu je token;
promjena vrijednosti tokena traži ponovno mjerenje. Sokrat Study se ne dira.

## S-018 — grafovi su vlastiti SVG, bez biblioteke (2026-09-18)

**Kontekst:** tablica je imala grafove (kumulativna linija, pita vrsta rada) i gubila ih pri dopuni;
Leon proučava, pa grafovi nisu ukras. Kandidati: ručni SVG, uPlot/LayerChart, Chart.js.
**Odluka:** četiri Svelte komponente (`Bars` · `Line` · `Ring` · `Sparkline`) s bojama isključivo
`var(--color-*)`. Biblioteke odbijene: jedna ovisnost više za pinati i obrazlagati, i posao vezanja
njezinih boja na tokene bez kojeg teme razočaraju; Chart.js uz to nosi vlastit prepoznatljiv izgled.
**Posljedice:** teme rade bez dodatnog koda; osi, mreža i tooltip su naši i ostaju jednostavni
(`<title>`/`aria-label`); kad točaka bude tisuće, mjerenje odlučuje o biblioteci — ne prije.

## S-019 — animacija jednom po pokretanju procesa; glavni prozor čeka; preskočiva (2026-09-18)

**Kontekst:** Leon je dostavio gotovu animaciju (`sokratis-intro-clean-graph.html`, 4,2 s, canvas,
dva WebP-a) i zaključak `S ◍ KRATIS`. Uz autostart i tray, pravo pokretanje je rijetko, otvaranje
prozora iz traya često.
**Odluka:** splash je zaseban prozor bez okvira; animacija se preuzima **doslovno** (vremenska crta,
boje, dvije slike) i vrti **jednom po pokretanju procesa**, ne pri otvaranju iz traya. Za to vrijeme
se učitavaju projekti; glavni prozor se pokazuje kad su **oba** gotova. Klik/tipka preskače na zadnji
kadar; `prefers-reduced-motion` dobiva zadnji kadar odmah (kao u Leonovu kodu).
**Posljedice:** 4,2 s nikad nije prazno čekanje; znak je dio identiteta bez da smeta petnaest puta
na dan; tray-ikona traži pojednostavljen znak jer se lik na 16 px stopi (provjera gledanjem).

## S-020 — tray minimizira, autostart, jedna instanca, obavijest samo na prijelaz u Alert (2026-09-18)

**Kontekst:** PRD traži tray i obavijest na Alert; Leon je odabrao najprisutniju varijantu.
**Odluka:** X sakriva prozor, izlaz je izričit iz tray-izbornika; autostart s Windowsom je postavka
koju Leon može isključiti; `single-instance` podiže postojeći prozor umjesto druge ikone; obavijest
OS-a ide **samo kad signal prijeđe u Alert ili se pojavi nov Alert**, nikad pri svakom osvježavanju.
**Posljedice:** watcher radi i dok prozor ne postoji, pa obavijest stiže i dok Leon ne gleda; ista
nespojena grana ne javlja svakih par minuta; četiri Tauri plugina (dialog, notification, autostart,
single-instance) su namjerne ovisnosti.

## S-021 — HR/EN prekidač već u M2; rječnik s jednakim ključevima (2026-09-18)

**Kontekst:** BACKLOG je HR/EN vodio kao „M2/M3"; S-008 je jezgru već učinio engleskom. Leon je
odabrao prekidač odmah, ne samo hrvatski s rječnikom.
**Odluka:** `hr.json` + `en.json`, jedan ključ po natpisu, vlastiti `t(key)` bez biblioteke; test
tvrdi da obje datoteke imaju iste ključeve; nijedan natpis u komponenti. Jezgrini identifikatori su
ključevi, natpisi su vrijednosti.
**Posljedice:** svaki novi ekran traži dva natpisa i provjeru širine; objava u M3 ne traži prolazak
kroz ekrane; CLI-tablica ostaje hrvatska (BACKLOG, M3).

## S-022 — snapshot `Report`-a je prva cigla M2; oblik se mijenja samo namjerno (2026-09-18)

**Kontekst:** M1 je `insta` uklonio jer bi snimka zamrznula oblik koji se još mijenja (I7); M2 gradi
sučelje nad tim oblikom, pa svaka nenamjerna promjena JSON-a tiho lomi ekran.
**Odluka:** prva cigla M2 je snapshot cijelog `Report`-a nad fixtureom pariteta (`insta` se vraća
jednim retkom); svaki dodatak koji mijenja oblik (`until`, `vision_totals`, `commits`) mijenja
snimku **namjerno** (`cargo insta review`) s obrazloženjem u commitu.
**Posljedice:** ugovor CLI ↔ sučelje ima jedan test; recenzent vidi promjenu oblika u diffu snimke,
ne u pogađanju; `RUST.md` §2 dobiva `insta` natrag kao dev-ovisnost `core`-a.

**Dopuna (2026-09-18, pri pisanju plana):** „prva cigla" znači **prva cigla toka JEZGRA (T2)**. Prije nje
ide kostur T1 (orkestrator), koji nova polja `Report`-a dodaje **prazna** — pa snimka nastaje nad kosturom
i od tada čuva svako punjenje (T3–T5 mijenjaju snimku namjerno, s rečenicom u commitu).
