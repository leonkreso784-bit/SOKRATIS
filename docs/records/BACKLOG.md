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
| Znak Sokratisa | M2/M3 | nov; Sokratov logo se ne dira |
| README na engleskom · LICENCA (MIT kao Sokrat Study — potvrditi) · GitHub Actions | M3 | prije objave |
| Sokratis mjeri vlastiti repo (dogfooding): `.sokratis/profile.json` ovdje | M1 kraj | kad postoji CLI |
| Instalater (Tauri bundler, MSI/NSIS) | M3 | |

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
