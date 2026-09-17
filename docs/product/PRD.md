# PRD — Sokratis

**Status:** 🟩 VRIJEDI · **Otvoren:** 2026-09-17 · **Vlasnik:** Leon Kreso

> Ovaj dokument kaže **ŠTO** gradimo i **kad je gotovo**. Kako je izgrađeno kaže
> [architecture/ARCHITECTURE.md](../architecture/ARCHITECTURE.md) (spec M1 je ispunjen i arhiviran:
> [archive/ARHITEKTURA_M1.md](../archive/ARHITEKTURA_M1.md)); zašto — [records/DECISIONS.md](../records/DECISIONS.md).
> Bez kronologije: to je posao `records/`.

---

## 1 · Problem

Solo-developer vodi projekt kroz git i pisanu dokumentaciju (dnevnik sesija, plan faza, changelog).
Želi znati **koliko radi, koliko dobro radi i ide li projekt u dobrom smjeru** — bez ručnog
prepisivanja. Dosadašnje rješenje (Excel koji generira skripta) radi za jedan projekt, gubi grafove
pri dopuni, ne zna za više projekata i ima mjerljive kvarove.

## 2 · Za koga

1. **Leon** i Sokrat Study — prvi korisnik, od Milestonea 1, bez ikakve konfiguracije.
2. **Solo-developer s GitHuba** koji vodi git + markdown dokumentaciju — od Milestonea 3, kroz profil projekta.

Nije za timove s ticketing sustavima; nije za ljude bez gita.

## 3 · Što Sokratis zna

| mogućnost | gotovo kad korisnik može… |
|---|---|
| **Priključiti projekt** | …pokazati na mapu repoa i dobiti izvještaj bez ijedne postavke (zadane konvencije), a radna stabla istog repoa vidjeti kao **jedan** projekt |
| **Statistika rada** | …vidjeti tempo po danu, vrste rada, pokazatelje kvalitete i brzine, faze i vizije — isto što je imao u tablici, s ispravnim satima |
| **Ručni ispravci** | …promijeniti vrstu rada jednom commitu i urediti vizije, i te promjene preživjeti svako osvježavanje jer žive u `.sokratis/` u repou |
| **Čistoća dokumentacije** | …vidjeti ocjenu i **popis nalaza s mjestom** (datoteka:redak), a projekt bez docs-a vidjeti kao „nema", ne kao nulu |
| **Signali smjera** | …vidjeti upozorenje s **dokazom** (koje grane, koliko dana, koji commit), ne samo boju |
| **Više projekata** | …u jednom pregledu vidjeti sve projekte i njihove signale |
| **CLI kao brana** | …staviti `sokratis signals` u preflight i dobiti izlazni kod koji zaustavlja push |
| **Desktop [M2]** | …otvoriti aplikaciju iz traya, dobiti sistemsku obavijest kad signal padne na Alert, i sve to u izgledu Sokrat Studyja |
| **Vlastite konvencije [M3]** | …napisati `.sokratis/profile.json` za projekt s drukčijim dnevnikom, planom i pragovima |

## 4 · Opseg po milestoneima

| milestone | u opsegu | izvan opsega |
|---|---|---|
| **M1** | jezgra · CLI · paritet s RAD.xlsx · ispravak sati · docs-ocjena · signali `unmerged-branches` i `docs-lag` | ekran, SQLite, watcher |
| **M2** | Tauri 2 · Svelte 5 · tokeni · SQLite snimke · watcher · tray · obavijesti | novi signali, adapteri |
| **M3** | ostala pravila · profil za tuđe projekte · HR/EN · instalater · znak · README EN · objava | oblak, timovi |

## 5 · Ne-ciljevi

- **Bez oblaka i računa.** Sve lokalno. (Pogled s telefona je ideja za kasnije, ne obećanje.)
- **Bez štoperice.** Sati ostaju git-proxy i tako su označeni.
- **Ne uređuje tuđe dokumente.** Čita `docs/`, ne piše u njega (osim `.sokratis/`).
- **Bez GitHub/Vercel API-ja u M1.** CI-status i deployi su kandidati za M3.
- **Bez sučelja u Rustu.** Tokeni i teme su CSS; isti izgled je cilj.

## 6 · Načela koja se mjere

| načelo | kako se provjerava |
|---|---|
| **Mjera, ne procjena** | svaki pokazatelj u JSON-u nosi `kind: measure \| proxy` i formulu |
| **Git je izvor istine** | nema dnevnog zapisa; izvještaj se izvodi na zahtjev i uvijek je isti za isti git |
| **Ručno = minimalno** | ručni podaci su točno dva: override vrste po SHA i vizije |
| **Mjerač kaže koliko je dotaknuo** | `touched` u svakom izvještaju; preskočen redak se broji, ne guta |
| **Dokaz uz svaki signal** | signal bez `evidence` ne prolazi test |

## 7 · Rječnik

| pojam | značenje |
|---|---|
| **projekt** | jedan git-direktorij; sva radna stabla koja ga dijele su isti projekt |
| **radno stablo** | `git worktree`; Sokrat Study ih ima pet |
| **zadana grana** | grana nad kojom se računaju metrike (paritet); ostale ulaze samo u signale |
| **profil** | `.sokratis/profile.json` — deklaracija konvencija projekta; zadano = Sokrat Study |
| **dnevnik** | markdown s naslovima `## YYYY-MM-DD (MODEL) — naslov`; jedan naslov = jedna **isporuka** |
| **plan** | markdown s tablicom cigli po fazama; `✅` = gotova cigla |
| **cigla** | jedan commit čiji opis počinje oznakom faze (npr. `F2/3`) |
| **faza** | skup cigli; zatvorena (iz profila, broji se iz gita) ili aktivna (iz plana) |
| **vrsta rada** | planiranje · vođenje dokumentacije · izvođenje procesa · poliranje koda · debugging |
| **signal** | nalaz pravila s težinom (Info/Warn/Alert) i **dokazom** |
| **mjera / proxy** | mjera dolazi iz gita ili dokumenta; proxy je heuristika (sati) i tako je označen |
| **touched** | koliko je mjerač pročitao i koliko preskočio |
