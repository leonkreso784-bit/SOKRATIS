//! ZAŠTO RUST OVAKO (cigla M1/6 — klasifikator)
//! `for (kind, re) in &p.classifier` posuđuje vektor parova bez kopiranja; `*kind` kopira `Copy`
//! enum iz reference. Redoslijed pravila je dio ugovora (planiranje > dokumentacija > debugging >
//! poliranje), zato je to `Vec`, ne `HashMap`.
use crate::{Patterns, SubKind, WorkKind};

pub fn classify_kind(subject: &str, p: &Patterns) -> WorkKind {
    let t = subject.to_lowercase();
    for (kind, re) in &p.classifier {
        if re.is_match(&t) {
            return *kind;
        }
    }
    WorkKind::Execution
}

pub fn classify_sub(subject: &str, p: &Patterns) -> SubKind {
    let t = subject.to_lowercase();
    if subject.contains('🚀') || p.deploy.is_match(&t) {
        SubKind::Deploy
    } else if p.gate.is_match(&t) {
        SubKind::GateOrMeasure
    } else if p.phase_tag.is_match(subject) {
        SubKind::Brick
    } else {
        SubKind::Other
    }
}

pub fn phase_tag(subject: &str, p: &Patterns) -> Option<String> {
    p.phase_tag.captures(subject).map(|c| c[1].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Profile, SubKind::*, WorkKind::*};

    #[test]
    fn kinds_follow_sokrat_study_order() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        let cases = [
            ("docs: BUGS.md razdvojen + RASPORED §0 SPAJANJE", Planning), // 'raspored' prije 'docs'
            (
                "docs(progress): sesija 07.09. -- model monetizacije zakljucan",
                Documentation,
            ),
            (
                "fix(check:docs): gitignoriran artefakt nije duh-datoteka",
                Debugging,
            ),
            (
                "BUG-045: Service Worker nikad nije spremao runtime assete",
                Debugging,
            ),
            ("C5b/1: paleta ostatak", Polish),
            ("F2/2 cigla 3: zid crta profilnu i naslovnu", Execution),
            ("N1: naucena kartica se pamti po IDENTITETU", Execution),
        ];
        for (s, want) in cases {
            assert_eq!(classify_kind(s, &p), want, "{s}");
        }
    }

    #[test]
    fn sub_kinds_and_tags() {
        let p = Patterns::compile(&Profile::default()).unwrap();
        assert_eq!(
            classify_sub("docs: 🚀 F2/2 slike profila na produkciji", &p),
            Deploy
        );
        assert_eq!(
            classify_sub("MREZA A2: check:node napisan i dokazan", &p),
            GateOrMeasure
        );
        assert_eq!(classify_sub("F2/2 cigla 3: zid", &p), Brick);
        assert_eq!(classify_sub("N1: naucena kartica", &p), Other);
        assert_eq!(phase_tag("F2/2 cigla 3: zid", &p).as_deref(), Some("F2/2"));
        assert_eq!(
            phase_tag("MREZA A1: baza popravljena", &p).as_deref(),
            Some("MREZA A1")
        );
        assert_eq!(phase_tag("docs: nešto", &p), None);
    }
}
