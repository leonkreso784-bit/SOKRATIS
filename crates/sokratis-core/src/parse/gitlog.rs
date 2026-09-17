//! ZAŠTO RUST OVAKO (cigla M1/3 — parser git loga)
//! `let … else` (Rust 1.65+): raspakiraj ili izađi s greškom u istom retku — bez ugniježđenog
//! `match`. `splitn(6, '|')` čuva `|` unutar opisa commita jer zadnji komad uzima ostatak.
//! `commits.last_mut()` = posudba zadnjeg elementa za upis (`&mut`) — jedan `&mut` u jednom trenu.
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
}
