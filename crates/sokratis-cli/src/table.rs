//! ZAŠTO RUST OVAKO (cigla M1/19 — tablica)
//! `match` nad `&str` s `_ => id` = tablica prijevoda bez `HashMap` i bez alokacije; natpisi su
//! ovdje, u sučelju, a ne u jezgri (S-008). `writeln!(s, …)` na `String` kroz `std::fmt::Write`
//! gradi izlaz bez međuvektora — nema `unwrap()`: `write!` na `String` ne može stvarno pasti,
//! ali potpis vraća `Result` pa se ignorira eksplicitno kroz `let _ =`.
use sokratis_core::{DocsHealth, IndicatorKind, PhaseState, Report, Severity, Signal};
use std::fmt::Write;

/// Hrvatski natpis za engleski identifikator pokazatelja/vrste/provjere. Nepoznat id se ispisuje
/// kakav jest (S-008: identifikatori su ugovor, natpisi su ukras sučelja).
pub fn label(id: &str) -> &str {
    match id {
        "working_days" => "radnih dana (dana s commitom)",
        "commits" => "commita",
        "commits_per_day" => "commita po radnom danu",
        "deliveries" => "isporuka (unosa u dnevniku)",
        "deliveries_per_day" => "isporuka po radnom danu",
        "hours" => "sati rada (git-hours proxy)",
        "commits_per_hour" => "commita po satu (proxy)",
        "lines_changed" => "redaka promijenjeno (+/−)",
        "test_lines" => "redaka u testovima i branama",
        "test_share" => "udio testnih redaka",
        "deploys" => "deploya na produkciju",
        "debugging_commits" => "debugging commita",
        "debugging_share" => "udio debugginga",
        "docs_share" => "udio dokumentacije",
        "ci_fixes" => "CI-padova popravljenih",
        "owner_driven_deliveries" => "isporuka pokrenutih vlasnikovim nalazom",
        "closed_phases_in_range" => "zatvorenih faza u razdoblju",
        "closed_phase_avg_days" => "prosječno trajanje zatvorene faze (dana)",
        "planning" => "planiranje",
        "documentation" => "vođenje dokumentacije",
        "execution" => "izvođenje procesa",
        "polish" => "poliranje koda",
        "debugging" => "debugging",
        _ => id,
    }
}

pub fn render(r: &Report) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "Sokratis — analiza rada · grana {} · od {} · dotaknuto: {} commita, {} redaka, {} datoteka, preskočeno {} redaka\n",
        r.branch,
        r.since,
        r.touched.commits,
        r.touched.lines,
        r.touched.files,
        r.touched.skipped_lines
    );
    let _ = writeln!(
        s,
        "TEMPO PO DANU\n{:<12}{:>8}{:>8}{:>10}{:>7}{:>9}{:>8}{:>10}",
        "dan", "commiti", "kum.", "redaka", "sati", "isporuke", "deploya", "testnih"
    );
    for d in &r.days {
        let _ = writeln!(
            s,
            "{:<12}{:>8}{:>8}{:>10}{:>7.1}{:>9}{:>8}{:>10}",
            d.date,
            d.commits,
            d.commits_cumulative,
            d.lines,
            d.hours,
            d.deliveries,
            d.deploys,
            d.test_lines
        );
    }
    let _ = writeln!(s, "\nVRSTE RADA");
    for k in &r.kinds {
        let _ = writeln!(
            s,
            "{:<26}{:>6}{:>8.1}%{:>10}",
            label(k.kind.id()),
            k.commits,
            k.share * 100.0,
            k.lines
        );
    }
    let _ = writeln!(s, "\nKVALITETA I BRZINA");
    for i in &r.indicators {
        let kind = if i.kind == IndicatorKind::Proxy {
            " (proxy)"
        } else {
            ""
        };
        let _ = writeln!(s, "{:<44}{:>10}{}", label(&i.id), i.value, kind);
    }
    let _ = writeln!(s, "\nFAZE");
    // I8: zatvorenu fazu bez ijednog pogođenog commita tablica preskače — pokazatelji je ne
    // broje, pa je ni ispis ne smije prikazati kao našu povijest.
    for p in r
        .phases
        .iter()
        .filter(|p| p.state != PhaseState::Closed || p.commits > 0)
    {
        let _ = writeln!(
            s,
            "{:<50} {:?} {}/{}",
            p.name, p.state, p.done_bricks, p.total_bricks
        );
    }
    if let Some(d) = &r.docs {
        let _ = writeln!(s, "\n{}", render_docs(d));
    }
    let _ = writeln!(s, "\nSIGNALI\n{}", render_signals(&r.signals));
    s
}

pub fn render_docs(d: &DocsHealth) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "DOKUMENTACIJA — ocjena {}/100, nalaza {}, kašnjenje {}",
        d.score,
        d.findings.len(),
        d.lag_days
            .map(|x| format!("{x} dana"))
            .unwrap_or_else(|| "n/a".into())
    );
    for f in &d.findings {
        let line = f.line.map(|l| format!(":{l}")).unwrap_or_default();
        let _ = writeln!(s, "  {:<22} {}{}  {}", f.check, f.path, line, f.message);
    }
    s.trim_end().to_string()
}

pub fn render_signals(signals: &[Signal]) -> String {
    if signals.is_empty() {
        return "nema signala".into();
    }
    let mut s = String::new();
    for sig in signals {
        let sev = match sig.severity {
            Severity::Alert => "ALERT",
            Severity::Warn => "WARN",
            Severity::Info => "INFO",
        };
        let _ = writeln!(s, "{sev:<6} {}", sig.rule);
        for e in &sig.evidence {
            let _ = writeln!(s, "  - {e}");
        }
    }
    s.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sokratis_core::*;

    #[test]
    fn labels_are_croatian_and_unknown_ids_pass_through() {
        assert_eq!(label("debugging_share"), "udio debugginga");
        assert_eq!(label("nepoznato"), "nepoznato");
        let r = Report {
            generated_at: 0,
            since: "2026-08-29".into(),
            branch: "main".into(),
            touched: Touched {
                commits: 3,
                lines: 10,
                files: 2,
                skipped_lines: 0,
            },
            days: vec![DayStats {
                date: "2026-09-01".into(),
                commits: 3,
                commits_cumulative: 3,
                lines: 10,
                hours: 1.5,
                deliveries: 1,
                deploys: 0,
                test_lines: 2,
            }],
            kinds: vec![],
            indicators: vec![Indicator {
                id: "commits".into(),
                value: 3.0,
                kind: IndicatorKind::Measure,
                formula: "n".into(),
            }],
            phases: vec![],
            visions: vec![],
            docs: None,
            signals: vec![],
        };
        let s = render(&r);
        assert!(
            s.contains("TEMPO PO DANU")
                && s.contains("2026-09-01")
                && s.contains("commita")
                && s.contains("dotaknuto: 3 commita")
        );
        let sig = render_signals(&[Signal {
            rule: "docs-lag".into(),
            severity: Severity::Warn,
            title_key: "signal.docs_lag".into(),
            evidence: vec!["dokaz".into()],
            since: None,
        }]);
        assert!(sig.contains("WARN") && sig.contains("docs-lag") && sig.contains("  - dokaz"));
        assert_eq!(render_signals(&[]), "nema signala");
    }

    /// I8: zatvorena faza bez pogođenih commita je tuđa povijest — pokazatelji je ne broje, pa je
    /// ni tablica ne ispisuje. Zatvorena faza S commitima i aktivna faza bez njih ostaju.
    #[test]
    fn closed_phase_without_commits_is_skipped_in_the_table() {
        let phase = |name: &str, state: PhaseState, commits: u32| Phase {
            id: name.into(),
            name: name.into(),
            state,
            total_bricks: commits,
            done_bricks: commits,
            from: None,
            to: None,
            days: None,
            commits,
        };
        let r = Report {
            generated_at: 0,
            since: "2026-08-29".into(),
            branch: "main".into(),
            touched: Touched {
                commits: 0,
                lines: 0,
                files: 0,
                skipped_lines: 0,
            },
            days: vec![],
            kinds: vec![],
            indicators: vec![],
            phases: vec![
                phase("TUĐA POVIJEST", PhaseState::Closed, 0),
                phase("NAŠA FAZA", PhaseState::Closed, 7),
                phase("PLANIRANA", PhaseState::Planned, 0),
            ],
            visions: vec![],
            docs: None,
            signals: vec![],
        };
        let s = render(&r);
        assert!(!s.contains("TUĐA POVIJEST"), "{s}");
        assert!(s.contains("NAŠA FAZA") && s.contains("PLANIRANA"), "{s}");
    }
}
