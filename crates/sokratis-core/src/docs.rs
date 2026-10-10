//! ZAŠTO RUST OVAKO (cigla M1/12 — čistoća dokumentacije · popravci M6 i M8)
//! Svaka provjera je mala privatna funkcija koja PUNI `Vec<Finding>` kroz `&mut` — jedan
//! vlasnik vektora (ova funkcija), više posudbi u nizu, nikad istodobno. `HashSet<&str>` nad
//! putanjama daje O(1) provjeru „postoji li cilj poveznice" bez kopiranja stringova. Regexi su
//! u `Patterns` (i konstantni), pa u ovoj datoteci nema ni jednog `expect()` (nalaz M6).
//!
//! ZAŠTO RUST OVAKO (cigla 1.0.2/T69 — zaglavlje plana)
//! `plan_header(&str) -> &str` vraća REZ (`&content[..end]`) iste memorije, ne kopiju: nema
//! `String`, nema alokacije. Rust iz potpisa sam zaključi (lifetime elision) da rez živi točno
//! koliko i ulaz, pa ga ne može nadživjeti. `end` dolazi iz `find("\n## ")`, a `\n` je jedan bajt,
//! zato je rez uvijek na granici znaka i ne može panicirati usred `⏸`.
//!
//! ZAŠTO RUST OVAKO (cigla 1.0.2/T70 — kod nije poveznica)
//! Stanje „jesmo li u bloku koda" je `Option<(char, usize)>`: `None` = izvan bloka, `Some(('~', 4))`
//! = unutra, a blok zatvara samo ograda istog znaka i barem te duljine — jedan tip nosi i „jesmo li"
//! i „čime se zatvara", pa ta dva ne mogu otići u raskorak. Inline kod su `Range<usize>` (bajtni
//! rasponi, kraj isključen); `span.contains(&start)` kaže je li poveznica počela u kodu.
use crate::{DocFile, DocsHealth, Finding, Patterns, Profile};
use std::collections::HashSet;
use std::ops::Range;

fn finding(check: &str, path: &str, line: Option<usize>, message: String) -> Finding {
    Finding {
        check: check.into(),
        path: path.into(),
        line,
        message,
    }
}

/// `dir/../x.md` → normalizirano s `/`; vraća None ako izlazi iznad korijena.
fn resolve(from_file: &str, link: &str) -> Option<String> {
    let mut parts: Vec<&str> = from_file
        .rsplit_once('/')
        .map(|(dir, _)| dir)
        .unwrap_or("")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    for seg in link.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

/// Ako redak izgleda kao ograda bloka koda (CommonMark): do 3 razmaka uvlake, pa niz od barem 3
/// ista znaka `` ` `` ili `~`. Vraća (znak, duljinu niza, ostatak retka iza niza).
fn fence_run(line: &str) -> Option<(char, usize, &str)> {
    let rest = line.trim_start_matches(' ');
    let indent = line.len() - rest.len();
    if indent > 3 {
        return None;
    }
    let mark = rest.chars().next()?;
    if mark != '`' && mark != '~' {
        return None;
    }
    // Oba znaka su ASCII (1 bajt), pa je broj znakova ujedno broj bajtova za rez ispod.
    let len = rest.chars().take_while(|&c| c == mark).count();
    if len < 3 {
        return None;
    }
    Some((mark, len, &rest[len..]))
}

/// Redak koji OTVARA blok koda. Iza ``` ne smije biti backticka: ``` `` x `` ``` je inline kod.
fn fence_open(line: &str) -> Option<(char, usize)> {
    let (mark, len, info) = fence_run(line)?;
    if mark == '`' && info.contains('`') {
        return None;
    }
    Some((mark, len))
}

/// Redak koji ZATVARA blok otvoren s `open`: isti znak, barem ista duljina, iza samo razmaci.
fn fence_closes(line: &str, open: (char, usize)) -> bool {
    match fence_run(line) {
        Some((mark, len, rest)) => mark == open.0 && len >= open.1 && rest.trim().is_empty(),
        None => false,
    }
}

/// Rasponi inline koda u retku (bajtni indeksi, kraj isključen): niz od N backtickova otvara,
/// sljedeći niz od TOČNO N backtickova zatvara. Niz bez para je obični znak i ne otvara ništa.
fn inline_code_spans(line: &str) -> Vec<Range<usize>> {
    // Prvo svi nizovi backtickova kao (početak, duljina). `as_bytes` je siguran: backtick je ASCII.
    let bytes = line.as_bytes();
    let mut runs: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            let start = i;
            while i < bytes.len() && bytes[i] == b'`' {
                i += 1;
            }
            runs.push((start, i - start));
        } else {
            i += 1;
        }
    }
    let mut spans = Vec::new();
    let mut k = 0;
    while k < runs.len() {
        let (open_at, n) = runs[k];
        let partner = runs[k + 1..].iter().position(|&(_, len)| len == n);
        match partner {
            Some(offset) => {
                let (close_at, _) = runs[k + 1 + offset];
                spans.push(open_at..close_at + n);
                k += offset + 2; // iza zatvarajućeg niza
            }
            None => k += 1,
        }
    }
    spans
}

fn dead_links(files: &[DocFile], known: &HashSet<&str>, p: &Patterns, out: &mut Vec<Finding>) {
    let re = &p.md_link;
    for f in files {
        // Otvoreni blok koda: (znak ograde, duljina). Poveznica u bloku je primjer, ne poveznica.
        let mut open_fence: Option<(char, usize)> = None;
        for (i, line) in f.content.lines().enumerate() {
            if let Some(open) = open_fence {
                if fence_closes(line, open) {
                    open_fence = None;
                }
                continue;
            }
            if let Some(open) = fence_open(line) {
                open_fence = Some(open);
                continue;
            }
            let code = inline_code_spans(line);
            for c in re.captures_iter(line) {
                // Grupa 0 (cijeli pogodak) uvijek postoji; počinje li u inline kodu, to je primjer.
                let starts_in_code = c
                    .get(0)
                    .is_some_and(|m| code.iter().any(|span| span.contains(&m.start())));
                if starts_in_code {
                    continue;
                }
                // Grupa 1 je zajamčena: regex je konstanta iz `Patterns`, a ne iz profila.
                let Some(link) = c.get(1).map(|m| m.as_str()) else {
                    continue;
                };
                if link.contains("://") {
                    continue;
                }
                let ok = resolve(&f.path, link).is_some_and(|t| known.contains(t.as_str()));
                if !ok {
                    out.push(finding(
                        "dead-link",
                        &f.path,
                        Some(i + 1),
                        format!("poveznica na {link} ne postoji"),
                    ));
                }
            }
        }
    }
}

fn not_indexed(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    let Some(index) = files.iter().find(|f| f.path == p.docs_index) else {
        return;
    };
    let prefix = format!("{}/", p.docs_dir);
    for f in files
        .iter()
        .filter(|f| f.path.starts_with(&prefix) && f.path != p.docs_index)
    {
        let rel = &f.path[prefix.len()..];
        if !index.content.contains(rel) {
            out.push(finding(
                "not-indexed",
                &f.path,
                None,
                format!("{rel} nije naveden u {}", p.docs_index),
            ));
        }
    }
}

/// Zaglavlje plana: tekst do prvog retka koji počinje s `## ` (bez tog retka), ili cijela datoteka
/// ako takvog retka nema. Status plana („PAUZIRAN") vrijedi samo ovdje — isto slovo u tijelu
/// (povijest, primjer) ne pauzira plan.
fn plan_header(content: &str) -> &str {
    if content.starts_with("## ") {
        return "";
    }
    match content.find("\n## ") {
        Some(end) => &content[..end],
        None => content,
    }
}

fn active_plans(files: &[DocFile], p: &Profile, pat: &Patterns, out: &mut Vec<Finding>) {
    let prefix = format!("{}/", p.plan_dir);
    let specs: Vec<&DocFile> = files
        .iter()
        .filter(|f| {
            f.path.starts_with(&prefix)
                && !p
                    .plan_dir_ignore
                    .iter()
                    .any(|i| f.path.ends_with(i.as_str()))
        })
        .collect();
    if specs.is_empty() {
        return;
    }
    let active: Vec<&str> = specs
        .iter()
        .filter(|f| !pat.paused.is_match(plan_header(&f.content)))
        .map(|f| f.path.as_str())
        .collect();
    if active.len() > 1 {
        out.push(finding(
            "multiple-active-plans",
            &p.plan_dir,
            None,
            format!("više aktivnih planova: {}", active.join(", ")),
        ));
    } else if active.is_empty() {
        out.push(finding(
            "no-active-plan",
            &p.plan_dir,
            None,
            "svi planovi su PAUZIRANI; točno jedan mora nositi prvenstvo".into(),
        ));
    }
}

fn diary_in_definition(files: &[DocFile], profile: &Profile, p: &Patterns, out: &mut Vec<Finding>) {
    let re = &p.iso_date;
    let prefix = format!("{}/", profile.product_dir);
    for f in files.iter().filter(|f| f.path.starts_with(&prefix)) {
        let n = re.find_iter(&f.content).count();
        if n > 3 {
            out.push(finding(
                "diary-in-definition",
                &f.path,
                None,
                format!("{n} datuma u definiciji; kronologija ide u records/"),
            ));
        }
    }
}

fn key_file_budget(files: &[DocFile], p: &Profile, out: &mut Vec<Finding>) {
    if let Some(f) = files.iter().find(|f| f.path == p.key_file) {
        let bytes = f.content.replace('\r', "").len() as u64;
        if bytes > p.key_file_budget_bytes {
            out.push(finding(
                "key-file-budget",
                &f.path,
                None,
                format!("{bytes} B > budžet {} B", p.key_file_budget_bytes),
            ));
        }
    }
}

/// Kašnjenje dokumentacije u CIJELIM danima: od zadnjeg commita koda do zadnje promjene
/// dnevnika/changeloga. Formula živi na JEDNOM mjestu (nalaz M8) — dijele je pokazatelj
/// (`docs_health`) i pravilo (`rules::docs_lag`), pa brojka i signal ne mogu reći različito.
pub fn lag_days(code_time: i64, docs_time: i64) -> i64 {
    (code_time - docs_time) / 86_400
}

fn lag(
    files: &[DocFile],
    last_code: Option<i64>,
    p: &Profile,
    out: &mut Vec<Finding>,
) -> Option<i64> {
    let code = last_code?;
    let docs_time = files
        .iter()
        .filter(|f| f.path == p.diary_path || f.path == p.changelog_path)
        .filter_map(|f| f.last_change_time)
        .max()?;
    let days = lag_days(code, docs_time);
    if days <= 0 {
        return Some(0);
    }
    if days >= p.docs_lag_warn_days {
        out.push(finding(
            "docs-lag",
            &p.diary_path,
            None,
            format!("dnevnik i changelog kasne {days} dana za zadnjim commitom koda"),
        ));
    }
    Some(days)
}

pub fn docs_health(
    files: &[DocFile],
    last_code_commit_time: Option<i64>,
    profile: &Profile,
    p: &Patterns,
) -> Option<DocsHealth> {
    let prefix = format!("{}/", profile.docs_dir);
    if !files.iter().any(|f| f.path.starts_with(&prefix)) {
        return None;
    }
    let known: HashSet<&str> = files.iter().map(|f| f.path.as_str()).collect();
    let mut findings = Vec::new();
    dead_links(files, &known, p, &mut findings);
    not_indexed(files, profile, &mut findings);
    active_plans(files, profile, p, &mut findings);
    diary_in_definition(files, profile, p, &mut findings);
    key_file_budget(files, profile, &mut findings);
    let lag_days = lag(files, last_code_commit_time, profile, &mut findings);
    let w = &profile.docs_weights;
    let penalty: u32 = findings
        .iter()
        .map(|f| match f.check.as_str() {
            "dead-link" => w.dead_link,
            "not-indexed" => w.not_indexed,
            "multiple-active-plans" => w.multiple_plans,
            "no-active-plan" => w.no_active_plan,
            "diary-in-definition" => w.diary_in_definition,
            "docs-lag" => w.lag,
            "key-file-budget" => w.key_file_budget,
            _ => 0,
        } as u32)
        .sum();
    Some(DocsHealth {
        score: 100u32.saturating_sub(penalty) as u8,
        findings,
        lag_days,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Profile;

    fn f(path: &str, content: &str, t: Option<i64>) -> DocFile {
        DocFile {
            path: path.into(),
            content: content.into(),
            last_change_time: t,
        }
    }
    const DAY: i64 = 86_400;

    #[test]
    fn no_docs_dir_means_none_not_zero() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        assert!(docs_health(&[f("README.md", "# x", None)], None, &p, &pat).is_none());
    }

    #[test]
    fn findings_with_locations_and_score() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f(
                "docs/README.md",
                "[PRD](./product/PRD.md)\n[nema](./plan/NEMA.md)\nplan/A.md plan/B.md plan/ROADMAP.md records/PROGRESS.md records/CHANGELOG.md",
                None,
            ),
            f(
                "docs/product/PRD.md",
                "2026-01-01 2026-01-02 2026-01-03 2026-01-04",
                None,
            ),
            f("docs/plan/A.md", "**Status:** AKTIVAN", None),
            f("docs/plan/B.md", "**Status:** AKTIVAN", None),
            f("docs/plan/ROADMAP.md", "", None),
            f("docs/records/DUH.md", "", None),
            f("docs/records/PROGRESS.md", "", Some(10 * DAY)),
            f("docs/records/CHANGELOG.md", "", Some(9 * DAY)),
            f("CLAUDE.md", "x", None),
        ];
        let h = docs_health(&files, Some(13 * DAY), &p, &pat).unwrap();
        let checks: Vec<(&str, &str)> = h
            .findings
            .iter()
            .map(|x| (x.check.as_str(), x.path.as_str()))
            .collect();
        assert!(checks.contains(&("dead-link", "docs/README.md")));
        assert!(checks.contains(&("not-indexed", "docs/records/DUH.md")));
        assert!(checks.contains(&("multiple-active-plans", "docs/plan")));
        assert!(checks.contains(&("diary-in-definition", "docs/product/PRD.md")));
        assert!(checks.contains(&("docs-lag", "docs/records/PROGRESS.md")));
        assert!(!checks.iter().any(|(c, _)| *c == "key-file-budget"));
        let dead = h.findings.iter().find(|x| x.check == "dead-link").unwrap();
        assert_eq!(dead.line, Some(2));
        assert_eq!(h.lag_days, Some(3));
        assert_eq!(h.score, 100 - 5 - 3 - 15 - 5 - 10);
    }

    #[test]
    fn paused_plan_is_not_active_and_budget_is_checked() {
        let p = Profile {
            key_file_budget_bytes: 3,
            ..Profile::default()
        };
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", "**Status:** ⏸️ PAUZIRAN", None),
            f("docs/plan/B.md", "aktivan", None),
            f("CLAUDE.md", "abcd\r\n", None),
        ];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        assert!(
            !h.findings
                .iter()
                .any(|x| x.check.ends_with("active-plan") || x.check.ends_with("active-plans"))
        );
        let b = h
            .findings
            .iter()
            .find(|x| x.check == "key-file-budget")
            .unwrap();
        assert!(b.message.contains("5 B"), "mjeri se bez CR: {}", b.message);
        assert_eq!(h.lag_days, None);
    }

    /// Pomoćnik za T69: dva plana — `A.md` s danim sadržajem i `B.md` koji je sigurno aktivan.
    /// Vraća provjere koje je ocjena javila o planovima.
    fn plan_checks(a_content: &str) -> Vec<String> {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", a_content, None),
            f("docs/plan/B.md", "# B\n\n**Status:** AKTIVAN", None),
        ];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        h.findings
            .into_iter()
            .map(|x| x.check)
            .filter(|c| c.ends_with("active-plan") || c.ends_with("active-plans"))
            .collect()
    }

    #[test]
    fn paused_status_under_title_and_blank_line_is_paused() {
        let a = "# Plan A\n\n**Status:** ⏸️ PAUZIRAN — čeka B\n\n## Cigle\n\nnešto";
        assert_eq!(plan_checks(a), Vec::<String>::new());
    }

    #[test]
    fn paused_status_with_crlf_under_title_is_paused() {
        let a = "# Plan A\r\n\r\n**Status:** ⏸️ PAUZIRAN\r\n\r\n## Cigle\r\n";
        assert_eq!(plan_checks(a), Vec::<String>::new());
    }

    #[test]
    fn paused_word_only_in_body_below_first_h2_keeps_plan_active() {
        let a =
            "# Plan A\n\n**Status:** AKTIVAN\n\n## Povijest\n\n**Status:** ⏸️ PAUZIRAN bio jednom";
        assert_eq!(plan_checks(a), vec!["multiple-active-plans".to_string()]);
    }

    #[test]
    fn pause_symbol_without_variation_selector_is_paused() {
        // `⏸` bez nevidljivog U+FE0F iza (neki uređivači ga ne upišu).
        let a = "# Plan A\n\n**Status:** \u{23F8} PAUZIRAN\n";
        assert!(!a.contains('\u{FE0F}'));
        assert_eq!(plan_checks(a), Vec::<String>::new());
    }

    #[test]
    fn custom_paused_marker_applies_to_header_only() {
        let p = Profile {
            paused_marker: r"(?m)^STOP$".into(),
            ..Profile::default()
        };
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", "# A\nSTOP\n## x", None),
            f("docs/plan/B.md", "# B\n## x\nSTOP", None),
        ];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        // A je pauziran (STOP u zaglavlju), B aktivan (STOP ispod `## `) → nema nalaza o planovima.
        assert!(
            !h.findings
                .iter()
                .any(|x| x.check.ends_with("active-plan") || x.check.ends_with("active-plans")),
            "{:?}",
            h.findings
        );
        let files_both_body = vec![
            f("docs/README.md", "plan/A.md plan/B.md", None),
            f("docs/plan/A.md", "# A\n## x\nSTOP", None),
            f("docs/plan/B.md", "# B\n## x\nSTOP", None),
        ];
        let h = docs_health(&files_both_body, None, &p, &pat).unwrap();
        assert!(
            h.findings
                .iter()
                .any(|x| x.check == "multiple-active-plans"),
            "{:?}",
            h.findings
        );
    }

    /// Pomoćnik za T70: brojevi redaka `dead-link` nalaza u jednoj datoteci (`docs/README.md`).
    fn dead_link_lines(content: &str) -> Vec<usize> {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![f("docs/README.md", content, None)];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        h.findings
            .iter()
            .filter(|x| x.check == "dead-link")
            .filter_map(|x| x.line)
            .collect()
    }

    #[test]
    fn link_inside_tilde_fence_is_an_example() {
        let text = "uvod\n~~~\n[x](nema.md)\n```\n[x](nema.md)\n~~~ ne zatvara\n~~~\n[y](nema.md)";
        // ``` unutar ~~~ bloka ne zatvara blok (drugi znak); `~~~ ne zatvara` ima tekst iza ograde.
        assert_eq!(dead_link_lines(text), vec![8]);
    }

    #[test]
    fn longer_backtick_fence_is_closed_only_by_fence_at_least_as_long() {
        let text = "````md\n```\n[x](nema.md)\n```\n[x](nema.md)\n````\n[y](nema.md)";
        assert_eq!(dead_link_lines(text), vec![7]);
    }

    #[test]
    fn unclosed_fence_lasts_to_end_of_file() {
        assert!(dead_link_lines("```\n[x](nema.md)\n\n[x](nema.md)").is_empty());
    }

    #[test]
    fn fence_needs_at_most_three_spaces_and_no_backtick_in_info() {
        // 4 razmaka = nije ograda; ``` s backtickom u info-nizu = inline kod, nije ograda.
        let text = "    ```\n[a](nema.md)\n  ``` `` x `` ```\n[b](nema.md)\n   ~~~\n[c](nema.md)";
        assert_eq!(dead_link_lines(text), vec![2, 4]);
    }

    #[test]
    fn link_inside_inline_code_is_an_example() {
        let text = "primjer `[x](nema.md)` i ``[x](nema.md) ` unutra``\nnapola ``[x](nema.md)` nije zatvoren\n[`OK.md`](OK.md)";
        // Red 2: `` se ne zatvara jednim ` → nema raspona, poveznica se provjerava.
        // Red 3: kod u TEKSTU poveznice ne skriva cilj; README nema OK.md pa je i to nalaz.
        assert_eq!(dead_link_lines(text), vec![2, 3]);
    }

    #[test]
    fn real_dead_link_next_to_inline_code_is_still_reported() {
        let text = "prazno\n`[x](nema.md)` ali [y](nema.md) pa `kod`";
        assert_eq!(dead_link_lines(text), vec![2]);
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![f("docs/README.md", text, None)];
        let h = docs_health(&files, None, &p, &pat).unwrap();
        let dead: Vec<&str> = h
            .findings
            .iter()
            .filter(|x| x.check == "dead-link")
            .map(|x| x.message.as_str())
            .collect();
        assert_eq!(dead.len(), 1, "samo jedan nalaz u retku: {dead:?}");
    }

    #[test]
    fn lag_none_when_code_exists_but_docs_have_no_change_time() {
        let p = Profile::default();
        let pat = Patterns::compile(&p).unwrap();
        let files = vec![
            f("docs/README.md", "", None),
            f("docs/records/PROGRESS.md", "", None),
            f("docs/records/CHANGELOG.md", "", None),
        ];
        let h = docs_health(&files, Some(13 * DAY), &p, &pat).unwrap();
        assert_eq!(h.lag_days, None);
        assert!(!h.findings.iter().any(|x| x.check == "docs-lag"));
    }
}
