//! ZAŠTO RUST OVAKO (cigla M1/7 — civilni datumi · popravci C2 i I2)
//! Nula ovisnosti: `chrono` bi ovdje bio top za muhu. `Option` kroz `?` u pomoćnoj funkciji:
//! prvi neuspjeli `parse` vraća `None` i gotovo. Cijeli brojevi (`i64`) jer je danima mjesto u
//! cijelim brojevima, a prijestupne godine rješava formula, ne tablica. `is_ymd` i `prev_day` su
//! ČISTE funkcije nad `&str` (bez sata i bez zone) — zato ih smiju zvati i jezgra i `io`.
//!
//! ZAŠTO RUST OVAKO (cigla M2/3 — `next_day` za gornju granicu `until`)
//! `next_day` je zrcalo `prev_day`-a: ista `Option<...>` kroz `?`, isti kalendar (`days_in_month`),
//! samo smjer unaprijed. Nema nove logike za prijestupnu godinu — ista formula koja već postoji.

/// Rastavlja `YYYY-MM-DD` u `(godina, mjesec, dan)`; `None` ako format ne valja.
fn ymd(s: &str) -> Option<(i64, i64, i64)> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    Some((
        s[0..4].parse().ok()?,
        s[5..7].parse().ok()?,
        s[8..10].parse().ok()?,
    ))
}

/// Dani od 1970-01-01 (algoritam H. Hinnanta, „days_from_civil").
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    // era = redni broj 400-godišnjeg ciklusa (npr. 0 za 1600.-1999., -1 za 1200.-1599.)
    let era = if y >= 0 { y } else { y - 399 } / 400;
    // yoe = godina unutar ere (year of era), 0..399
    let yoe = y - era * 400;
    // mp = mjesec pomaknut tako da ožujak bude mjesec 0 (siječanj/veljača idu na kraj prošle godine)
    let mp = (m + 9) % 12;
    // doy = redni dan unutar (pomaknute) godine, 0..365
    let doy = (153 * mp + 2) / 5 + d - 1;
    // doe = dan unutar 400-godišnje ere (day of era), 0..146096
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Dana u mjesecu `m` godine `y`; veljača ovisi o prijestupnoj godini (gregorijansko pravilo).
fn days_in_month(y: i64, m: i64) -> i64 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
            if leap { 29 } else { 28 }
        }
    }
}

/// Je li `s` valjan civilni datum `YYYY-MM-DD` — i po obliku i po kalendaru (`2026-02-30` nije)?
/// Jedina provjera datuma u sustavu: `--since`, `profile.since` i `closed_phases[].from/to`
/// prolaze kroz nju prije nego postanu granica mjerenja (nalaz C2).
pub fn is_ymd(s: &str) -> bool {
    match ymd(s) {
        Some((y, m, d)) => (1..=12).contains(&m) && d >= 1 && d <= days_in_month(y, m),
        None => false,
    }
}

/// Dan prije `date`, isti oblik `YYYY-MM-DD`; `None` ako `date` nije valjan datum.
/// Rezerva pri dovlačenju git-loga (nalaz I2): granica `--since` je ponoć u zoni STROJA, a
/// datumi commita su u zoni COMMITA, pa se dovlači dan više i odluku prepušta jezgri.
/// Prijelaz mjeseca i godine rješava `days_in_month`, ne tablica izuzetaka.
pub fn prev_day(date: &str) -> Option<String> {
    if !is_ymd(date) {
        return None;
    }
    let (mut year, mut month, day) = ymd(date)?;
    let day = if day > 1 {
        day - 1
    } else {
        month -= 1;
        if month == 0 {
            month = 12;
            year -= 1;
        }
        days_in_month(year, month)
    };
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

/// Sljedeći kalendarski dan (`YYYY-MM-DD`); `None` ako `date` nije valjan datum.
/// Zrcalo `prev_day`-a, u drugom smjeru: `io` (T14) njime gradi `--until=<dan+1> 00:00:00`, pa git
/// dovuče cijeli `until` dan uz rezervu zone, a jezgra presudi `commit_date <= until`
/// (isti obrazac kao I2/S-011 dopuna 2, samo za gornju granicu razdoblja).
pub fn next_day(date: &str) -> Option<String> {
    if !is_ymd(date) {
        return None;
    }
    let (mut year, mut month, day) = ymd(date)?;
    let day = if day < days_in_month(year, month) {
        day + 1
    } else {
        month += 1;
        if month == 13 {
            month = 1;
            year += 1;
        }
        1
    };
    Some(format!("{year:04}-{month:02}-{day:02}"))
}

/// Dani od `from` do `to` (`to − from`); negativno ako je `to` prije `from`.
/// `None` ako bilo koji datum nije u formatu `YYYY-MM-DD`.
pub fn days_between(from: &str, to: &str) -> Option<i64> {
    let (fy, fm, fd) = ymd(from)?;
    let (ty, tm, td) = ymd(to)?;
    Some(days_from_civil(ty, tm, td) - days_from_civil(fy, fm, fd))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn days_between_handles_month_and_year_edges() {
        assert_eq!(days_between("2026-08-02", "2026-08-06"), Some(4));
        assert_eq!(days_between("2026-08-07", "2026-09-01"), Some(25));
        assert_eq!(days_between("2025-12-31", "2026-01-01"), Some(1));
        assert_eq!(
            days_between("2024-02-28", "2024-03-01"),
            Some(2),
            "prijestupna"
        );
        assert_eq!(days_between("2026-09-02", "2026-09-01"), Some(-1));
        assert_eq!(days_between("2026-9-2", "2026-09-01"), None);
        assert_eq!(days_between("x", "2026-09-01"), None);
    }

    /// C2: oblik I kalendar. `2026-9-17` i `17.09.2026` su tipfeleri koje je stari kod puštao u
    /// leksikografsku usporedbu, a `2026-02-30` je „valjan oblik, nepostojeći dan".
    #[test]
    fn is_ymd_accepts_only_real_calendar_days() {
        assert!(is_ymd("2026-09-17") && is_ymd("2024-02-29") && is_ymd("2026-12-31"));
        for bad in [
            "2026-9-17",
            "17.09.2026",
            "banana",
            "",
            "2026-02-30",
            "2026-13-01",
            "2026-00-10",
            "2026-09-00",
            "2025-02-29",
            "2026-09-17 ",
        ] {
            assert!(!is_ymd(bad), "`{bad}` nije valjan datum");
        }
    }

    /// I2: rezerva od jednog dana mora preživjeti prijelaz mjeseca, godine i prijestupni 29.02.
    #[test]
    fn prev_day_crosses_month_and_year() {
        assert_eq!(prev_day("2026-09-08").as_deref(), Some("2026-09-07"));
        assert_eq!(prev_day("2026-09-01").as_deref(), Some("2026-08-31"));
        assert_eq!(prev_day("2026-03-01").as_deref(), Some("2026-02-28"));
        assert_eq!(prev_day("2024-03-01").as_deref(), Some("2024-02-29"));
        assert_eq!(prev_day("2026-01-01").as_deref(), Some("2025-12-31"));
        assert_eq!(prev_day("2026-05-01").as_deref(), Some("2026-04-30"));
        assert_eq!(prev_day("banana"), None);
        assert_eq!(prev_day("2026-02-30"), None);
    }

    /// M2/3: zrcalo `prev_day`-a za gornju granicu `until` — ista rezerva prijestupne godine,
    /// samo u drugom smjeru (prijelaz mjeseca i godine unaprijed).
    #[test]
    fn next_day_crosses_month_and_year() {
        assert_eq!(next_day("2026-09-17").as_deref(), Some("2026-09-18"));
        assert_eq!(next_day("2026-09-30").as_deref(), Some("2026-10-01"));
        assert_eq!(next_day("2026-12-31").as_deref(), Some("2027-01-01"));
        assert_eq!(next_day("2028-02-28").as_deref(), Some("2028-02-29"));
        assert_eq!(next_day("nije-datum"), None);
    }
}
