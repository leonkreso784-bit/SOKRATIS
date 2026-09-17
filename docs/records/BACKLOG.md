# BACKLOG — parkiralište

> **Nije plan i nije obećanje.** Stavka ovdje čeka milestone koji je uzme ili odluku koja je odbije.
> Plan ne nosi tuđe stavke; kad stavka uđe u spec, ovdje ostaje samo pointer.

## Čeka milestone

| stavka | za | bilješka |
|---|---|---|
| Pravila M3: udio debugginga raste 3 dana · cigle/dan ispod prosjeka zatvorenih faza · deploy > 14 dana · udio testnih pada · commiti bez unosa u dnevniku · faza dulja od prosjeka | M3 | popis iz speca §2.5; svako pravilo = svoja datoteka + test |
| Profil za tuđe projekte: dokumentiran JSON format + primjer za repo bez `docs/` | M3 | zadano = Sokrat Study (S-005) |
| GitHub adapter: CI status po grani, PR-ovi | M3 (opcionalno) | mreža; `gh` nije na stroju |
| Vercel adapter: deployi umjesto `🚀` u dnevniku | M3 (opcionalno) | mreža |
| HR/EN natpisi u CLI tablici i sučelju | M2/M3 | jezgra je već engleska (S-008) |
| README na engleskom · LICENCA (MIT kao Sokrat Study — potvrditi) · GitHub Actions | M3 | prije objave |
| Instalater (Tauri bundler, MSI/NSIS) | M3 | |

## Iz završne recenzije M1 (nalazi koji nisu popravljeni u M1)

Izvor je izvještaj završne recenzije (`.superpowers/sdd/2026-09-17-m1-jezgra-i-cli/final-review-report.md`
— radni zapis izvan gita), zato je uz svaku stavku broj nalaza. **Stanje koda** (što danas ne radi)
opisuje [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11; ovdje stoji **plan**.

| stavka | za | bilješka |
|---|---|---|
| Aktivne faze vezati na `Patterns.phase_tag` umjesto tvrdog prefiksa `"{id}/"` — time polje i `classify::phase_tag()` prestaju biti mrtvi | M2 | I3 + M14; projekt koji cigle označava `M1-3` ili `[M1.3]` danas dobiva praznu fazu bez poruke |
| Vizije: zbroj po stanju + odjeljak u ispisu (spec M1 §2.3 ga je tražio, plan mu nije dao ciglu) | M2 (pogled Vizije) | I6 |
| Ograditi putanje iz profila na korijen repoa (`docs_dir: "../.."` danas čita iznad repoa) | **M2, prije objave** | I9; jedna funkcija `inside_root(rel)` + poruka koje je polje krivo |
| Jedinice i natpisi tablice: udjeli u %, prijevod `Closed/Running/Planned`, „nema faza" umjesto praznog naslova, širina stupca | M2 (sučelje) | M3; JSON je ugovor i on je točan, tablica je pomoć za terminal |
| Performanse: jedan `git log --name-only` za sve docs umjesto `log -1` po datoteci, `for-each-ref` s ahead-behind umjesto `rev-list` po grani, klasifikacija commita jednom po izvještaju | M2 (watcher ih plaća u petlji) | M11; 3,2 s nad Sokrat Studyjem, 0,7 s nad Sokratisom |
| `include_unmerged`: metrike i nad nespojenim granama (danas polje postoji, jezgra ga ne čita) | M2 | odgođena 7 |
| Detached HEAD dobiva poruku „repozitorij nema commita" jer je `git branch --show-current` prazan | M2 | parkirano uz krug popravaka; nije regresija, rubno stanje koje CLI ne cilja |
| Snapshot JSON-a (`Report`) kad se oblik zaključa za Tauri; `insta` se tada vraća jednim retkom | M2 | I7; do tada bi snimka zamrznula oblik koji se još mijenja |
| Testni redak mjeriti tako da fixture ne prevlada (izuzeti `fixtures/` ili mjeriti drukčije) | M2 | odgođena 8; danas je većina Sokratisovih „testnih redaka" fixture (767 kB snimka `PROGRESS.md`) |
| `rust-toolchain.toml` (pin kompajlera, ne samo crateova) | **Leonova odluka** | M15; pin na konkretnu verziju tjera `rustup` da skine još jednu kopiju toolchaina na Leonov stroj — promjena njegova sustava, korisno prije M3 |

## Za Leona (tuđi repo, ne naš posao)

| stavka | bilješka |
|---|---|
| `sokratstudy.dev/scripts/rad-xlsx.py`: dodati ` 00:00` uz `--since` (retci 154, 178) | S-011 — bez sata `git log --since` uzima trenutno doba dana; `RAD.xlsx` zato ovisi o satu pokretanja skripte, dnevni zadatak u 23:45 gubi gotovo cijeli tekući dan |

## Ideje, bez datuma

| stavka | bilješka |
|---|---|
| Pogled s telefona na LAN-u (lokalni servis + PWA) | razmatrano kao ljuska C; nije odabrano, ali jezgra to ne sprječava |
| Snimke ocjena kroz vrijeme kao graf (trend docs-čistoće) | traži SQLite (M2) |
| `gix` umjesto `git` procesa | tek kad mjerenje kaže da je sporo (S-003) |
| Keš parsiranih commita po SHA u SQLite-u | tek kad repo bude dovoljno velik da se osjeti |

## Odbijeno (s razlogom)

| stavka | razlog |
|---|---|
| C++ | S-001: ekosustav, UTF-8 na Windowsu, ljuska |
| Rust GUI (Dioxus, Slint, egui) | S-006: tokeni i teme su CSS; isti izgled je cilj |
| Electron | RAM i veličina za aplikaciju koja radi cijeli dan; jezgra bi svejedno bila ista |
| Oblak / računi / Supabase | PRD §5: sve lokalno |
| Štoperica za sate | PRD §5: sati ostaju git-proxy, označen kao proxy |
| Dnevni zadatak koji „bilježi" (kao `rad-dnevno.ps1`) | git je izvor istine; izvodi se na zahtjev |
