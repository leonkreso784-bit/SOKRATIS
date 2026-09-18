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
| HR/EN natpisi u **sučelju** | → spec M2 [§6.4](../plan/ARHITEKTURA_M2.md) (S-021) | samo pointer |
| HR/EN natpisi u **CLI tablici** | M3 | jezgra je već engleska (S-008); tablica je pomoć za terminal |
| README na engleskom · LICENCA (MIT kao Sokrat Study — potvrditi) · GitHub Actions | M3 | prije objave |
| Instalater (Tauri bundler, MSI/NSIS) | M3 | |

## Iz završne recenzije M1 (nalazi koji nisu popravljeni u M1)

Izvor je izvještaj završne recenzije (`.superpowers/sdd/2026-09-17-m1-jezgra-i-cli/final-review-report.md`
— radni zapis izvan gita), zato je uz svaku stavku broj nalaza. **Stanje koda** (što danas ne radi)
opisuje [`../architecture/ARCHITECTURE.md`](../architecture/ARCHITECTURE.md) §11; ovdje stoji **plan**.

**Sedam od devet stavki preuzeo je spec M2** — [`../plan/ARHITEKTURA_M2.md`](../plan/ARHITEKTURA_M2.md)
§8 ih nabraja s mjestom u specu i testom koji ih dokazuje: snapshot `Report`-a (I7) · performanse
(M11) · ograda putanja (I9) · zbroj vizija (I6) · `phase_tag` (I3 + M14) · detached HEAD · testni redak
koji fixture prevlada (odgođena 8). Ovdje ostaju samo dvije koje M2 **ne** uzima:

| stavka | za | bilješka |
|---|---|---|
| Jedinice i natpisi **CLI-tablice**: udjeli u %, prijevod `Closed/Running/Planned`, „nema faza" umjesto praznog naslova, širina stupca | M3 | M3; sučelje to rješava u specu M2 §6.3, tablica u terminalu je pomoć — JSON je ugovor i on je točan |
| `include_unmerged`: metrike i nad nespojenim granama (danas polje postoji, jezgra ga ne čita) | M3 ili odluka | odgođena 7; nijedan pogled M2 to ne traži |

## Za Leona (tuđi repo, ne naš posao)

| stavka | bilješka |
|---|---|
| `sokratstudy.dev/scripts/rad-xlsx.py`: dodati ` 00:00` uz `--since` (retci 154, 178) | S-011 — bez sata `git log --since` uzima trenutno doba dana; `RAD.xlsx` zato ovisi o satu pokretanja skripte, dnevni zadatak u 23:45 gubi gotovo cijeli tekući dan |

## Ideje, bez datuma

| stavka | bilješka |
|---|---|
| Pogled s telefona na LAN-u (lokalni servis + PWA) | razmatrano kao ljuska C; nije odabrano, ali jezgra to ne sprječava |
| Snimke ocjena kroz vrijeme kao graf (trend docs-čistoće) | → spec M2 [§4.2](../plan/ARHITEKTURA_M2.md) (S-014); samo pointer |
| `gix` umjesto `git` procesa | tek kad mjerenje kaže da je sporo (S-003) |
| Keš parsiranih commita po SHA u SQLite-u | → spec M2 [§4.4](../plan/ARHITEKTURA_M2.md): tablica definirana, cigla **tek ako mjerenje nakon M11** to traži (pravilo #4) |
| Vektorizacija znaka (danas raster WebP/PNG iz Leonove datoteke) | kad tray na 16 px ili instalater to zatraže; M2 rješava pojednostavljenim rasterom |
| Otvaranje nalaza dokumentacije u editoru (danas klik kopira putanju) | M3; traži `tauri-plugin-opener` — jedna ovisnost više |

## Odbijeno (s razlogom)

| stavka | razlog |
|---|---|
| C++ | S-001: ekosustav, UTF-8 na Windowsu, ljuska |
| Rust GUI (Dioxus, Slint, egui) | S-006: tokeni i teme su CSS; isti izgled je cilj |
| Electron | RAM i veličina za aplikaciju koja radi cijeli dan; jezgra bi svejedno bila ista |
| Oblak / računi / Supabase | PRD §5: sve lokalno |
| Štoperica za sate | PRD §5: sati ostaju git-proxy, označen kao proxy |
| Dnevni zadatak koji „bilježi" (kao `rad-dnevno.ps1`) | git je izvor istine; izvodi se na zahtjev |
