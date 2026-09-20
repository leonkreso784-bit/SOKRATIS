//! ZAŠTO RUST OVAKO (cigla M1/3 — parser git loga)
//! `let … else` (Rust 1.65+): raspakiraj ili izađi s greškom u istom retku — bez ugniježđenog
//! `match`. `splitn(6, '|')` čuva `|` unutar opisa commita jer zadnji komad uzima ostatak.
//! `commits.last_mut()` = posudba zadnjeg elementa za upis (`&mut`) — jedan `&mut` u jednom trenu.
//!
//! Cigla M2/14b (potrošač keša, `io`): `format_gitlog` je INVERZ ovog parsera — keš pamti
//! `Commit`-e (strukturu), ne sirovi tekst, pa `io` mora znati sastaviti tekst natrag u istom
//! obliku da `ReportInput.git_log` ostane TEKST bez obzira dolazi li od `git log` ili od keša (S-012).
use crate::{Commit, FileChange, ParseError};

#[derive(Debug)]
pub struct Parsed {
    pub commits: Vec<Commit>,
    pub skipped_lines: usize,
}

fn numstat(s: &str) -> Option<u64> {
    if s == "-" { Some(0) } else { s.parse().ok() }
}

pub fn parse_git_log(text: &str) -> Result<Parsed, ParseError> {
    let mut commits: Vec<Commit> = Vec::new();
    let mut skipped_lines = 0usize;
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        if let Some(rest) = line.strip_prefix("@@") {
            let mut parts = rest.splitn(6, '|');
            let (Some(sha), Some(at), Some(ct), Some(date), Some(commit_date), Some(subject)) = (
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
            ) else {
                return Err(ParseError::BadLine {
                    line: n,
                    text: line.to_string(),
                });
            };
            let num = |s: &str| {
                s.parse::<i64>().map_err(|_| ParseError::BadNumber {
                    line: n,
                    text: s.to_string(),
                })
            };
            commits.push(Commit {
                sha: sha.to_string(),
                author_time: num(at)?,
                commit_time: num(ct)?,
                date: date.to_string(),
                commit_date: commit_date.to_string(),
                subject: subject.to_string(),
                files: Vec::new(),
            });
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split('\t').collect();
        match (commits.last_mut(), cols.as_slice()) {
            (Some(cur), [a, d, path]) => match (numstat(a), numstat(d)) {
                (Some(added), Some(deleted)) => cur.files.push(FileChange {
                    path: path.to_string(),
                    added,
                    deleted,
                }),
                _ => skipped_lines += 1,
            },
            _ => skipped_lines += 1,
        }
    }
    Ok(Parsed {
        commits,
        skipped_lines,
    })
}

/// Inverz `parse_git_log`: tekst koji parser pročita natrag u ISTE commite (`skipped_lines == 0`).
/// Redak po commitu `@@{sha}|{author_time}|{commit_time}|{date}|{commit_date}|{subject}`, pa po
/// datoteci `{added}\t{deleted}\t{path}`; svaki redak završava s `\n`, bez praznih redaka. Binarna
/// datoteka je u modelu već `0/0` (parser `-` čita kao 0), pa se ispisuje `0\t0\t…` — brojke iste.
pub fn format_gitlog(commits: &[Commit]) -> String {
    let mut out = String::new();
    for c in commits {
        out.push_str(&format!(
            "@@{}|{}|{}|{}|{}|{}\n",
            c.sha, c.author_time, c.commit_time, c.date, c.commit_date, c.subject
        ));
        for f in &c.files {
            out.push_str(&format!("{}\t{}\t{}\n", f.added, f.deleted, f.path));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOG: &str = "@@abc1234|1788664600|1788817583|2026-09-06|2026-09-08|fix(check:docs): gitignoriran artefakt nije duh-datoteka -- brana je padala\n3\t1\tscripts/check-docs.js\n-\t-\tassets/logo.png\n\n@@def5678|1788700000|1788700000|2026-09-06|2026-09-06|docs: zapis | s okomitom crtom\n10\t0\tdocs/records/PROGRESS.md\nsmeće bez tabova\n";

    #[test]
    fn parses_commits_files_binary_and_counts_skipped() {
        let p = parse_git_log(LOG).unwrap();
        assert_eq!(p.commits.len(), 2);
        let a = &p.commits[0];
        assert_eq!(
            (a.sha.as_str(), a.author_time, a.commit_time),
            ("abc1234", 1788664600, 1788817583)
        );
        assert_eq!(
            (a.date.as_str(), a.commit_date.as_str()),
            ("2026-09-06", "2026-09-08")
        );
        assert_eq!(a.files.len(), 2);
        assert_eq!(
            (
                a.files[0].added,
                a.files[0].deleted,
                a.files[0].path.as_str()
            ),
            (3, 1, "scripts/check-docs.js")
        );
        assert_eq!((a.files[1].added, a.files[1].deleted), (0, 0));
        assert_eq!(p.commits[1].subject, "docs: zapis | s okomitom crtom");
        assert_eq!(p.skipped_lines, 1);
    }

    #[test]
    fn bad_header_is_an_error_with_line_number() {
        let err = parse_git_log("@@abc|notanumber|1|2026-01-01|2026-01-01|x\n").unwrap_err();
        assert!(
            matches!(err, ParseError::BadNumber { line: 1, .. }),
            "{err}"
        );
        assert!(matches!(
            parse_git_log("@@abc|1|2\n").unwrap_err(),
            ParseError::BadLine { line: 1, .. }
        ));
    }

    /// Cigla M2/14b: `format_gitlog` je inverz `parse_git_log` — ručno složen `Vec<Commit>`
    /// pokriva rubove koje sirovi git ispis zna proizvesti: commit bez datoteka, `0/0`, `|` u
    /// naslovu, ne-ASCII putanju, putanju s razmakom i putanju u navodnicima kakvu git ispiše
    /// (doslovan tekst s navodnicima i kosom crtom, NE pravi tab).
    #[test]
    fn format_then_parse_is_identity() {
        let cs = vec![
            Commit {
                sha: "abc1234".to_string(),
                author_time: 1788664600,
                commit_time: 1788817583,
                date: "2026-09-06".to_string(),
                commit_date: "2026-09-08".to_string(),
                subject: "F1/1 bez datoteka".to_string(),
                files: Vec::new(),
            },
            Commit {
                sha: "def5678".to_string(),
                author_time: 1788700000,
                commit_time: 1788700000,
                date: "2026-09-06".to_string(),
                commit_date: "2026-09-06".to_string(),
                subject: "docs: zapis | s okomitom crtom".to_string(),
                files: vec![
                    FileChange {
                        path: "assets/logo.png".to_string(),
                        added: 0,
                        deleted: 0,
                    },
                    FileChange {
                        path: "docs/čćž.md".to_string(),
                        added: 1,
                        deleted: 2,
                    },
                    FileChange {
                        path: "docs/s razmakom.md".to_string(),
                        added: 3,
                        deleted: 4,
                    },
                    FileChange {
                        path: "\"docs/a\\tb.md\"".to_string(),
                        added: 5,
                        deleted: 6,
                    },
                ],
            },
        ];
        let parsed = parse_git_log(&format_gitlog(&cs)).unwrap();
        assert_eq!(parsed.commits, cs);
        assert_eq!(parsed.skipped_lines, 0);
    }

    /// Krug LOG → parse → format → parse ostaje isti skup commita; jedini redak smeća u `LOG`
    /// nestaje jer format ne piše retke koje parser ne bi mogao pročitati natrag.
    #[test]
    fn parse_then_format_drops_only_the_garbage_line() {
        let p1 = parse_git_log(LOG).unwrap();
        let p2 = parse_git_log(&format_gitlog(&p1.commits)).unwrap();
        assert_eq!(p2.commits, p1.commits);
        assert_eq!(p2.skipped_lines, 0);
    }

    #[test]
    fn format_of_nothing_is_empty() {
        assert_eq!(format_gitlog(&[]), "");
    }
}
