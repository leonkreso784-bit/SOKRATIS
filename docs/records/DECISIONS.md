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

## S-023 — repozitorij je javan na GitHubu od 2026-09-20, prije kraja M2 (Leonova odluka)

**Kontekst:** dotadašnji plan je bio „privatni remote prvo; javna objava tek nakon ograde putanja iz
profila (I9)" i objava kao dio M3. Leon je 2026-09-20 dao izričit OK za push i potvrdio da repo
**treba biti javan** jer ga želi dijeliti s drugim ljudima. Prije pusha mu je rečeno što time postaje
javno: fixturei pariteta sadrže cijeli `PROGRESS.md`, `RASPORED.md`, `README.md` i git-log Sokrat
Studyja (`crates/sokratis-core/tests/fixtures/sokratstudy-2026-09-17.*`), dokumentacija nosi lokalne
putanje (`C:\Users\leonk\…`), a povijest commita autorov e-mail. Odgovor: nije bitno, neka bude javno.
**Odluka:** `origin` = GitHub repo `leonkreso784-bit/SOKRATIS` (javan). Prvi push: `main` i tri
nespojene grane tokova (`feat/io-m2`, `feat/store`, `feat/ui`) — grane i kao sigurnosna kopija rada koji
je dotad postojao samo na Leonovu disku. Prije pusha sve četiri grane su pretražene na tajne (ključevi,
tokeni, lozinke, privatni ključevi): **ništa nije nađeno**.
**Posljedice:** io-dio ograde putanja (I9, cigla T14) više nije uvjet objave nego **dug prema javnom
kodu** — alat koji netko skine smije čitati samo unutar repoa koji mjeri, pa T14 dobiva prednost.
Licenca, README na engleskom i instalater ostaju M3; dok licence nema, kod je javno vidljiv ali **nije
licenciran za tuđu uporabu** (BACKLOG). Svaki novi fixture iz tuđeg ili privatnog repozitorija od sada
je javna objava — pregledava se na tajne PRIJE commita, ne prije pusha.

## S-024 — izlaz iz M2 je verzija 1.0.0; 0.2.0 se preskače (2026-09-21)

**Kontekst:** spec M2 je izlazni uvjet zvao 0.2.0. Leon je 2026-09-21 zapisao namjeru
([archive/PLAN_DESIGNE.md](../archive/PLAN_DESIGNE.md)): prva verzija mora **što prije** zamijeniti
`RAD.xlsx` nad Sokrat Studyjem i nad samim Sokratisom, pa njome nadgleda gradnju druge verzije.
**Odluka:** desktop koji ispuni spec M2 (s dopunom §13) izlazi kao **1.0.0**. Međuizdanja 0.2.0 nema —
završna recenzija, krug popravaka i čuvar izdanja rade **jednom**. Popravci iz stvarne uporabe su
`1.0.x`. Tag i izdanje na GitHubu i dalje traže Leonov izričit OK.
**Posljedice:** ROADMAP: M2 = 1.0.0, M3 = „Druga verzija", objava tuđim korisnicima (potpisan
instalater · licenca · GitHub Actions) postaje M4; `CHANGELOG.md` `[Unreleased]` se pri izdanju
zatvara kao 1.0.0.

## S-025 — crta reza za 1.0.0: ostatak M2 + izgled koji Leon traži; redoslijed funkcija → izgled → izdanje (2026-09-21)

**Kontekst:** preostalih osam cigli M2 (T28–T35) nije ukras — T28 nosi override (jedino ručno što
`RAD.xlsx` ima), T29/T30/T34 su žice, T31 je Leonova animacija, T32 je ono čime se nadgleda. Rez se
zato ne dobiva rezanjem M2 nego odlukom što iz zapisa namjere ulazi. Orkestratorova preporuka je bila
„ništa"; **Leon je odlučio da u 1.0.0 ulaze** kartica s objašnjenjem, tema i jezik u Postavkama te
animacije grafova i pogleda.
**Odluka:** 1.0.0 = T28–T35 + osam novih cigli (Postavke · animacija grafova · prijelaz pogleda s
prekidačem · kartica s objašnjenjem ×2 · instalater · repo bez konvencija · dokumentacija točna i
manja). Opseg i „gotovo kad": spec [§13](../archive/ARHITEKTURA_M2.md). Gradi se u tri etape:
**(1) funkcija** — T28 → spajanje SUČELJA → DESKTOP T29–T33 → T34 → repo bez konvencija → instalater;
Leon instalira **„1.0.0-pre"** (bez taga, nije izdanje) i počinje mjeriti Sokrat Study i Sokratis ·
**(2) izgled** — pet cigli sučelja, dok 1.0.0-pre već nadgleda tu gradnju · **(3) izdanje** — T35,
završna recenzija, jedan krug popravaka, cigla dokumentacije, čuvar izdanja.
**Posljedice:** redoslijed ne košta ništa (iste cigle), a uporaba počinje na pola puta; cijena
Leonova izbora je ~5 cigli više od najmanje crte. Sve ostalo iz zapisa namjere je u
[BACKLOG.md](./BACKLOG.md) („Druga verzija", „Kasnije").

## S-026 — animacije: najviše 250 ms, bez biblioteke, gase se prekidačem i `prefers-reduced-motion` (2026-09-21)

**Kontekst:** Leon želi da se grafovi otvaraju animirano i da svaki prelazak pogleda animirano učita
sadržaj. Alat se otvara dvadesetak puta dnevno — animacija koja traje postaje smetnja.
**Odluka:** svaka animacija sučelja traje **≤ 250 ms**; izvedba je CSS/SVG (prijelazi, `stroke-dashoffset`,
`transform`) — **nijedna nova ovisnost** (pravilo #6, isto kao S-018). Jedan atribut na korijenu
(`data-motion="off"`, uzor `data-theme`) gasi sve; postavlja ga prekidač u Postavkama **ili**
`prefers-reduced-motion: reduce`. Splash (S-019) ostaje zaseban: 4,2 s, jednom po pokretanju, preskočiv.
**Posljedice:** komponente grafova dobivaju ulaznu animaciju bez promjene API-ja; test tvrdi da s
ugašenim pokretom nijedno trajanje nije veće od nule.

## S-027 — kartica s objašnjenjem je statična: što govori · kako je izračunato · kako čitati (2026-09-21)

**Kontekst:** Leon želi da klik na graf, broj ili znamenku otvori karticu koja objašnjava podatak.
Ponuđeno je dvoje: statičan tekst ili tekst + dokaz (commiti iz kojih je broj nastao). Leon je odabrao
statičan tekst.
**Odluka:** jedna komponenta kartice; sadržaj su i18n ključevi `explain.<id>.what|how|read` u `hr.json`
i `en.json` (S-021). Pokriva svaki graf i svaku brojku u svim pogledima; „formula na klik" iz speca
§6.2 (Pokazatelji) postaje ta ista kartica. Test tvrdi da svaki objašnjivi `id` ima sva tri ključa u
oba jezika. Tekst „kako je izračunato" **opisuje** kod jezgre i piše se uz otvoren izvor
(`metrics/*.rs`) — definira i dalje kod (S-010).
**Posljedice:** ~30 objašnjenja × 3 odlomka × 2 jezika je glavnina cijene; promjena formule u jezgri od
sada traži i promjenu teksta (recenzent to provjerava). Dokaz po commitu je druga verzija (BACKLOG).

## S-028 — tema, jezik, autostart i prekidač animacija žive u pogledu Postavke; gornja traka ostaje čista (2026-09-21)

**Kontekst:** spec §6.1 je temu i HR/EN držao u gornjoj traci, a autostart samo u izborniku traya.
**Odluka:** nov pogled **Postavke** (globalan, ne po projektu) s četiri postavke: tema · jezik ·
autostart · animacije. Gornja traka gubi temu i HR/EN; tray zadržava kvačicu autostarta (isti izvor
istine — `sokratis-store`, postavke iz M2/16). Izbor preživi ponovno pokretanje.
**Posljedice:** Postavke su u 1.0.0 namjerno male; „vlastiti dodaci" (pragovi i pravila → vlastiti
pokazatelj → slaganje Pregleda → pluginovi) su druga verzija i dobivaju svoj spec.

## S-029 — instalater (NSIS) ulazi u 1.0.0, samo za Leona: nepotpisan i neobjavljen (2026-09-21)

**Kontekst:** spec §12 je instalater držao izvan M2. Autostart i „koristim svaki dan" traže stalnu
putanju izvršne datoteke; `tauri dev` to nije.
**Odluka:** `npm run tauri build` daje NSIS instalater koji Leon pokreće na svom stroju. Ne potpisuje
se, ne objavljuje na GitHubu, nema automatskog ažuriranja. Ista cigla služi za „1.0.0-pre" na kraju
etape 1 i za 1.0.0 na kraju etape 3.
**Posljedice:** SmartScreen upozorenje je očekivano i prihvaćeno; potpisivanje, objava instalatera i
MSI ostaju M4 (objava).

## S-030 — korijenski README je na engleskom; `docs/` ostaje na hrvatskom (2026-09-21)

**Kontekst:** repo je javan (S-023). Rečenica „README i sve ubuduće na engleskom" stigla je u drugoj
sesiji izvedbe **unutar rezultata alata**, pa po njoj nije postupljeno. Leon je 2026-09-21 izravnom
porukom potvrdio: README treba biti na engleskom da ga svi mogu koristiti.
**Odluka:** `README.md` u korijenu je engleski. Dokumentacija u `docs/`, `CLAUDE.md`, poruke commita i
zaglavlja „zašto Rust ovako" ostaju hrvatski — autor ih čita i iz njih uči (pravilo #5).
**Posljedice:** README se osvježava pri izdanju 1.0.0; prijevod ostatka dokumentacije nije planiran.

## S-031 — dokumentacija: točnost se čuva smanjivanjem; ispunjeno ide u `archive/` (2026-09-21)

**Kontekst:** Leonova ocjena nakon dvije sesije izvedbe: problem dokumentacije je **točnost**, a
količina je uzrok — u kratkom vremenu je izgrađeno i zapisano puno. `sokratis docs .` mjeri strukturu
(poveznice, indeks, jedan plan, kašnjenje), ne istinitost tvrdnji.
**Odluka:** (1) `CLAUDE.md` „Stanje" drži samo što vrijedi sad, bez povijesti; (2) dokument koji je
ispunio svrhu seli u `archive/` isti dan, s pečatom — prvi je zapis namjere `PLAN_DESIGNE.md`;
(3) prije izdanja 1.0.0 jedna cigla čuvara: svaka tvrdnja `ARCHITECTURE.md` provjerena prema kodu,
`PROGRESS.md` drži samo tekući milestone (zatvoreni sele u `archive/`), spec i plan M2 u `archive/`,
„Gdje smo" u ROADMAP-u bez kronologije. Imena dokumenata govore ulogu.
**Posljedice:** manje teksta koji može zastarjeti; povijest ostaje dostupna, ali odvojena od onoga što
vrijedi sad (S-010).

## S-032 — metrike nad svim lokalnim granama; oznaka grane po commitu (2026-09-24)

**Kontekst:** Leon je instalirao „1.0.0-pre" i vidio zadnje podatke od 13. 9. — zadnji commit na
`main`-u Sokrat Studyja; on od tada radi u šest grana u radnim stablima (102 commita u granama, 11 na
`main`-u od 13. 9.). Mjerenje „samo zadana grana" (S-005, paritet s `RAD.xlsx`) je za njegov način
rada krivo mjerilo. Kandidati: sve lokalne grane · grane u radnim stablima + zadana · zadana +
upozorenje.
**Odluka:** zadano se mjeri nad **svim lokalnim granama** (`git log --branches`: svaki commit jednom,
bez remote-tracking referenci i tagova); profil `branch_scope: "all" | "default"` pregazi, CLI
`--scope`. Oznaka grane po commitu dolazi iz odvojene karte (`git log --branches --not <zadana>
--format=%h|%S`), **ne u keš** — nakon spajanja commit prelazi pod zadanu granu. Sati po grani = sati
dana po udjelu commita grane (zbroj = ukupno; proxy). Stabla-samo odbijeno: grana bez stabla (danas
33 commita) bila bi nevidljiva, a brisanje stabla bi rušilo brojke unatrag. Upozorenje odbijeno: ne
rješava problem.
**Posljedice:** S-005 dopunjena (zadano više nije „samo zadana grana"); paritet s `RAD.xlsx` ostaje
isti test (fixture je log `main`-a); `Report` dobiva `scope`, `branches` i `branch` po retku
(snapshot mijenjan namjerno); signal `unmerged_branches` nepromijenjen; spec
[plan/ARHITEKTURA_1_0.md](../plan/ARHITEKTURA_1_0.md) §1.

## S-033 — dnevnik je unija svih radnih stabala; plan i `docs/` iz vodećeg stabla (2026-09-24)

**Kontekst:** `io` čita dnevnik, plan i `docs/` iz glavnog stabla (uz `.git`), a Leon dnevnik piše u
grani u kojoj radi (`feat/f6-mcp` ga dira 22.–23. 9.; stablo `.f6` ima 1300 redaka više). Isporuke i
docs-ocjena zato staju na 13. 9. kao i commiti.
**Odluka:** dnevnik se čita s diska **svakog radnog stabla** i isporuke se uniraju po (datum, naslov),
najnoviji pobjeđuje (dnevnici se uniraju redom od vodećeg stabla); plan i `docs/` (ocjena, kašnjenje, nalazi) čitaju se iz **vodećeg stabla** —
onog čiji HEAD ima najnoviji `author_time`. Grane bez stabla daju samo commite. Pisanje ručnih
podataka ostaje u glavno stablo (S-015). „Sve iz vodećeg stabla" odbijeno (isporuke iz drugih grana
nevidljive do spajanja); „ostaje glavno stablo" odbijeno (ne rješava problem).
**Posljedice:** `ReportInput.diary` → `diaries`; `Touched` += `worktrees`, `diaries`; +2 git-procesa po
izvještaju (nova granica u `perf.rs`); isporuka s uređenim naslovom u drugoj grani pojavi se dvaput
(dokumentirano na kartici).

## S-034 — klik na karticu = nadzorna ploča projekta; zasebni ostaju Dnevnik i Vizije (2026-09-24)

**Kontekst:** Leon očekuje da klik na karticu u Pregledu otvori „stranicu tog projekta sa svim
grafovima i brojkama", ne devet pogleda; „više projekata usporedno" za njega znači neograničene
kartice i ulazak u svaku. Kandidati: ploča + Dnevnik + Vizije · ploča kao jedanaesti pogled · jedna
stranica za sve uključujući uređivanje.
**Odluka:** šest pogleda koji samo prikazuju (Tempo · Vrste rada · Pokazatelji · Faze · Isporuke ·
Dokumentacija) postaju **sekcije jedne ploče** `Projekt`; zasebne stranice ostaju samo one koje pišu
(Dnevnik · Vizije). Izbornik: Pregled · Projekt · Dnevnik · Vizije · Postavke; **birač projekta u
gornjoj traci se uklanja** — Pregled je jedini ulaz. Jedanaesti pogled odbijen (grafovi dvaput);
sve-u-jednom odbijeno (uređivanje usred grafova).
**Posljedice:** spec M2 §6.1/§6.2 zamijenjen (spec 1.0.0 §3); svi `explain` id-evi ostaju; zajednički
graf usporedbe projekata na Pregledu ide u BACKLOG.

## S-035 — grafovi: d3-matematika + naš Svelte SVG; 4 nove npm ovisnosti (2026-09-24)

**Kontekst:** Leon traži puno više grafova, čitljivih, s datumima na osima. Vlastiti SVG bez datuma na
X-osi (S-018) nije dovoljan; težina je u matematici osi (datumski ticksi, `nice()`, stack, binovi,
lukovi), ne u pravokutnicima. Leon je odbio tri prve ponude i tražio bolje. Kandidati: čisti vlastiti
SVG · LayerChart 2.5 · Observable Plot · ECharts 6 · d3-jezgreni moduli + naš SVG.
**Odluka:** **hibrid** — `d3-scale@4.0.2` · `d3-shape@3.2.0` · `d3-array@3.2.4` ·
`d3-time-format@4.1.0` (ISC, bez DOM-a, čiste funkcije testirane vitestom) za matematiku; Svelte crta,
boje samo tokeni, animacije kroz `data-motion`, tooltip/legenda naši. Leon je 2026-09-24 dao OK za te
četiri ovisnosti (pravilo #6). LayerChart odbijen (30+ paketa, `-next` pre-release ovisnosti), Plot
(cijeli d3), ECharts (canvas, vlastiti izgled, teme kroz runtime, ≈1 MB), čisti SVG (datumske osi bismo
pisali sami).
**Posljedice:** S-018 revidirana (izgled i dalje naš, matematika tuđa); temelji `Chart · Axis · Grid ·
Legend · Tooltip · scales.ts · bucket.ts` i 8 komponenata; dan/tjedan/mjesec je preslagivanje u
Svelteu s testom, ne mjerenje (dopuna S-012); iOS-glossy izgled (v2 želja) ostaje moguć.

## S-036 — X = upit → izlaz; tray se uklanja; autostart otvara prozor (2026-09-24)

**Kontekst:** X je skrivao u tray (S-020), animacija se vrtjela jednom po procesu (S-019) — Leona je
zbunilo da drugi klik na ikonu ne daje ni animaciju ni „novo" pokretanje. Traži: X zatvara uz upit,
animacija pri svakom pokretanju. Kandidati: upit → izlaz bez traya · upit s tri gumba i tray ·
zatvaranje bez upita.
**Odluka:** X → upit „Zatvoriti Sokratis? Nadzor projekata i obavijesti staju dok ga ponovno ne
pokreneš." [Zatvori] [Odustani] → izlaz procesa; **tray-ikona i izbornik se uklanjaju**; autostart
pokreće aplikaciju **s prozorom** pri prijavi; `single-instance` ostaje. Svako pokretanje je nov
proces, pa S-019 sam daje animaciju. Tri gumba odbijena (isto stanje koje je zbunilo); bez upita
odbijeno (slučajan klik).
**Posljedice:** S-020 ukinuta u dijelu „X sakriva, tray"; obavijesti na Alert stižu samo dok je
aplikacija otvorena; `tray.rs` van, `set_autostart` u `autostart.rs`; feature `tray-icon` van.

## S-037 — sve iz Leonovih nalaza ulazi prije 1.0.0; jedno izdanje, instalater nakon svake sesije (2026-09-24)

**Kontekst:** procjena ≈ 20 cigli u 4–5 kratkih sesija. Kandidati: sve prije 1.0.0 · 1.0.0 = grane +
kvarovi + osi, ploča u 1.1.0 · 1.0.0 = samo grane + kvarovi.
**Odluka:** **sve prije 1.0.0** — Leonov izbor unatoč preporuci da se ploča odvoji. Redoslijed
„vrijednost prvo": DESKTOP-2 (konzola, X) → JEZGRA-2 + IO-2 (grane, dnevnik) → GRAFOVI → PLOČA →
IZDANJE; nakon svake sesije instalater „1.0.0-pre.N" (S-029) da Leon vidi gotovo odmah.
**Posljedice:** etapa 3 iz M2 §13 (T35, završna recenzija, T43) seli na kraj ovog reza; T35 ostaje
napola u stablu `sokratis.rel` do tada; spec M2 arhiviran, aktivan je
[plan/ARHITEKTURA_1_0.md](../plan/ARHITEKTURA_1_0.md).

## S-038 — nespojene grane su lanci: signal broji vrhove, sadržane grane su dokaz (2026-09-25)

**Kontekst:** vanjska analiza (24. 9.) pustila je `sokratis signals` nad Sokrat Studyjem: 8 prekršitelja
i **Alert** (prag 3), a `git merge-base --is-ancestor` kaže da je pet grana (`feat/f2-slike`,
`feat/f2-tema-racun`, `feat/f2-zid`, `feat/f2-mail`, `feat/f3-dvojezicnost`) SADRŽANO u `feat/f6-mcp`,
`feat/f2-zid-radionica` je 1 commit odvojena, a stvarno odvojene su samo `feat/tinder-kadar` i
`fix/kadar-nalicje`. Leon svaku sesiju grana od prethodne, pa su grane ulančane; pravilo ih gleda kao
ravan popis. Fixture pravila gradi četiri nezavisne grane — kvar je bio strukturno nevidljiv. Izlazni
kod 2 (Alert) je ugovor za pre-flight skripte, pa lažni Alert može zaustaviti tuđi proces. Isti
kvar u drugom obliku: `--table` zaglavlje tvrdi `grana main · 334 commita` (nalaz ide u T51).
**Odluka (Leon, 2026-09-25):** lanac = **jedan vrh**; pravilo broji vrhove, sadržane grane su dokaz
(`+5 grana unutar feat/f6-mcp`); vrh mlađi od praga utišava lanac. Ide **prije 1.0.0** kao cigla
**T64** (sesija 5, prije T35/T63) jer dira ugovor `ReportInput`/`BranchInfo`; `Report` ostaje isti.
Ujedno potvrđeno: **ništa s vanjske liste od 14 prijedloga ne ulazi u 1.0.0** — sve u `BACKLOG.md`.
**Posljedice:** `BranchInfo` += `tip`, `contained_in`; `ReportInput` += `branch_graph` (jedan git
proces, samo kad postoji nespojena grana; granica 8 ostaje); nov `core/src/chains.rs`; spec §1.4; plan
tok LANCI. Nusprodukti u BACKLOG: signal „živa grana predugo izvan zadane" · pravilo docs-a „citirana
brojka/verzija = izvor istine". `include_unmerged` (mrtvo polje) ostaje odluka T63.
