//! ZAŠTO RUST OVAKO (cigla M1/14 — pravilo kašnjenja docs-a)
//! Rani `return vec![]` kroz `let … else`: tri preduvjeta (commit koda, dnevnik, prag) čitaju se
//! odozgo prema dolje kao rečenice, bez piramide ugniježđenih `if`-ova.
use super::Rule;
use crate::{Context, Severity, Signal};

pub struct DocsLag;

impl Rule for DocsLag {
    fn id(&self) -> &'static str {
        "docs-lag"
    }

    fn evaluate(&self, ctx: &Context) -> Vec<Signal> {
        let p = &ctx.profile;
        let Some(code) = ctx.last_code_commit.as_ref() else {
            return vec![];
        };
        let Some(docs_time) = ctx
            .docs
            .iter()
            .filter(|f| f.path == p.diary_path || f.path == p.changelog_path)
            .filter_map(|f| f.last_change_time)
            .max()
        else {
            return vec![];
        };
        let lag = (code.author_time - docs_time) / 86_400;
        if lag < p.docs_lag_warn_days {
            return vec![];
        }
        let severity = if lag >= p.docs_lag_alert_days {
            Severity::Alert
        } else {
            Severity::Warn
        };
        vec![Signal {
            rule: self.id().into(),
            severity,
            title_key: "signal.docs_lag".into(),
            evidence: vec![
                format!("zadnji commit koda: {} {}", code.sha, code.date),
                format!("zadnja promjena dnevnika: prije {lag} dana"),
            ],
            since: Some(docs_time),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Commit, DocFile, Profile, Severity};
    const DAY: i64 = 86_400;

    fn ctx(code_day: Option<i64>, diary_day: Option<i64>) -> Context {
        let last_code_commit = code_day.map(|d| Commit {
            sha: "abc1234".into(),
            author_time: d * DAY,
            commit_time: d * DAY,
            date: "2026-09-13".into(),
            commit_date: "2026-09-13".into(),
            subject: "x".into(),
            files: vec![],
        });
        let docs = diary_day
            .map(|d| {
                vec![DocFile {
                    path: "docs/records/PROGRESS.md".into(),
                    content: String::new(),
                    last_change_time: Some(d * DAY),
                }]
            })
            .unwrap_or_default();
        Context {
            profile: Profile::default(),
            now: 0,
            commits: vec![],
            branches: vec![],
            docs,
            last_code_commit,
        }
    }

    #[test]
    fn silent_without_code_or_diary_or_when_fresh() {
        assert!(DocsLag.evaluate(&ctx(None, Some(1))).is_empty());
        assert!(DocsLag.evaluate(&ctx(Some(5), None)).is_empty());
        assert!(
            DocsLag.evaluate(&ctx(Some(5), Some(4))).is_empty(),
            "1 dan < prag 2"
        );
    }

    #[test]
    fn warn_at_two_days_alert_at_five() {
        let s = DocsLag.evaluate(&ctx(Some(5), Some(3)));
        assert_eq!((s.len(), s[0].severity), (1, Severity::Warn));
        assert_eq!(
            s[0].evidence,
            vec![
                "zadnji commit koda: abc1234 2026-09-13",
                "zadnja promjena dnevnika: prije 2 dana"
            ]
        );
        assert_eq!(s[0].since, Some(3 * DAY));
        assert_eq!(
            DocsLag.evaluate(&ctx(Some(10), Some(5)))[0].severity,
            Severity::Alert
        );
    }
}
