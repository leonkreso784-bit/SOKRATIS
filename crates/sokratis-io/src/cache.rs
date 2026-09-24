//! ZAŠTO RUST OVAKO (cigla M2/14b — potrošač keša commita)
//! `&dyn CommitCache` je trait-objekt: `io` NE zna tko stvarno pamti commite (`sokratis-store`,
//! memorija u testu…) i ne smije o njemu ovisiti (S-013) — poziva ga samo kroz ovaj ugovor.
//! `CacheError = Box<dyn Error + Send + Sync>` je greška čiji KONKRETAN tip `io` ne poznaje (dolazi
//! iz stranog cratea, dodaje se u desktopu T29) — jedini ispravan potpis je „bilo koja greška koja
//! smije putovati između dretvi". Poznato ograničenje: `touched.skipped_lines` na keširanom putu
//! ne broji retke preskočene pri PRVOM čitanju commita, jer keš pamti commite, ne sirovi tekst.
//!
//! Cigla M2/49 (S-032): `cached_log` dobiva `scope: Scope<'_>` umjesto imena grane — prosljeđuje
//! ga izravno `rev_list`-u, ne odlučuje o njemu (odluka je profila, u `project.rs`).
use crate::{GitSource, IoError, Scope};
use sokratis_core::Commit;
use std::collections::HashMap;

pub type CacheError = Box<dyn std::error::Error + Send + Sync>;

/// Mjesto koje pamti `Commit`-e po SHA-i. `io` ne zna KO ovo implementira — samo da smije pitati
/// „što već znaš" i reći „zapamti ovo". Implementacija smije zadržati i commite koji više nisu
/// dostižni (amend/rebase/reset ih ostave iza sebe): `cached_log` ih nikad ne čita, jer popis
/// commita za izvještaj dolazi iz `GitSource::rev_list`, ne iz keša.
pub trait CommitCache {
    /// Svi commiti koje keš drži za OVAJ projekt (uključujući one koji više nisu dostižni).
    fn cached(&self) -> Result<Vec<Commit>, CacheError>;
    /// Dopisuje nove; SHA koji keš već zna se ne dira (numstat commita se po SHA-i nikad ne mijenja).
    fn store(&self, commits: &[Commit]) -> Result<(), CacheError>;
}

/// Tekst git loga za prozor `[since, until]`, složen iz keša + dovlačenja SAMO nedostajućih
/// commita. U izvještaj ulaze SAMO commiti koje `rev_list` SADA vidi kao dostižne — golo „dovuci
/// sve novije od najnovijeg keširanog" (spec §4.4) taj rub ne pokriva: stari SHA nakon amenda bi
/// ostao u brojkama.
pub fn cached_log(
    git: &dyn GitSource,
    cache: &dyn CommitCache,
    scope: Scope<'_>,
    since: &str,
    until: Option<&str>,
) -> Result<String, IoError> {
    // 1. Prozor presuđuje `rev_list`, ne keš — ISTIM argumentima kao `log` (`window_args` u
    // `git.rs`), da se dva puta ne mogu razići.
    let reachable = git.rev_list(scope, since, until)?;
    // 2. Što keš već zna, indeksirano po SHA-i za brzo pitanje „imam li ovaj".
    let mut known: HashMap<String, Commit> = cache
        .cached()
        .map_err(IoError::Cache)?
        .into_iter()
        .map(|c| (c.sha.clone(), c))
        .collect();
    // 3. Dostižni kojih keš nema, ISTIM redom kao `reachable`.
    let missing: Vec<String> = reachable
        .iter()
        .filter(|sha| !known.contains_key(sha.as_str()))
        .cloned()
        .collect();
    // 4. Dovuci SAMO nedostajuće (jedan `git log --stdin`, bez šetnje cijelom poviješću), zapamti.
    if !missing.is_empty() {
        let text = git.log_commits(&missing)?;
        let parsed = sokratis_core::parse::parse_git_log(&text).map_err(IoError::LogParse)?;
        cache.store(&parsed.commits).map_err(IoError::Cache)?;
        for c in parsed.commits {
            known.insert(c.sha.clone(), c);
        }
    }
    // 5. Slaganje TIM redom iz `reachable` — manjak commita je laž u brojkama, nikad tiho
    // preskakanje.
    let mut ordered = Vec::with_capacity(reachable.len());
    for sha in &reachable {
        match known.remove(sha) {
            Some(c) => ordered.push(c),
            None => return Err(IoError::CacheIncomplete { sha: sha.clone() }),
        }
    }
    // 6. Natrag u tekst — `ReportInput.git_log` ostaje TEKST bez obzira na izvor (S-012).
    Ok(sokratis_core::parse::format_gitlog(&ordered))
}
