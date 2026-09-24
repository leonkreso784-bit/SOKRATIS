//! ZAŠTO RUST OVAKO (cigla M2/11, krug popravka 1 — last_changes kroz pravi merge-sudar)
//! Recenzija je dokazala pokusom da batch `last_changes` gubi doprinos merge-commita (zadano
//! `git log --name-only` ne ispisuje datoteke za merge) i da ne-ASCII ime datoteke nikad ne pogodi
//! mapu (zadano `core.quotepath` escapea ga u navodnike). Test NE upisuje očekivana vremena
//! napamet: računa ih STARIM putem po datoteci (`GitSource::last_change`, dokazano ispravan još iz
//! M1 jer git-ova simplifikacija povijesti po pathspecu i bez `--name-only` ispravno prepoznaje
//! merge koji je razlikuje putanju od SVIH roditelja) — tako test definira semantiku: batch mora
//! dati ISTO što i stari put, ne obrnuto.
mod common;
use common::Repo;
use sokratis_io::{GitCli, GitSource};

#[test]
fn last_changes_matches_last_change_across_a_real_merge_conflict() {
    let r = Repo::init();
    // t0: baza za conflict.md — obje grane je mijenjaju iz ISTE polazišne točke (preduvjet za
    // stvarni sudar, ne samo "dvije grane dirnu istu datoteku").
    r.commit(
        "docs/conflict.md",
        "base\n",
        "baza",
        "2026-09-01T08:00:00+02:00",
        "2026-09-01T08:00:00+02:00",
    );
    r.git(&["checkout", "-q", "-b", "topic"]);
    // topic mijenja SAMO branch_only.md ovim commitom — spoj s glavnom granom bit će čist.
    r.commit(
        "docs/branch_only.md",
        "topic only\n",
        "topic branch_only",
        "2026-09-02T09:00:00+02:00",
        "2026-09-02T09:00:00+02:00",
    );
    // topic mijenja i conflict.md — sudar je zajamčen jer main mijenja ISTI redak drukčije.
    r.commit(
        "docs/conflict.md",
        "topic version\n",
        "topic conflict",
        "2026-09-03T09:00:00+02:00",
        "2026-09-03T09:00:00+02:00",
    );
    r.git(&["checkout", "-q", "main"]);
    r.commit(
        "docs/conflict.md",
        "main version\n",
        "main conflict",
        "2026-09-04T09:00:00+02:00",
        "2026-09-04T09:00:00+02:00",
    );
    r.commit(
        "docs/plain.md",
        "plain\n",
        "plain",
        "2026-09-05T09:00:00+02:00",
        "2026-09-05T09:00:00+02:00",
    );
    r.try_merge("topic");
    // Ručno razrješenje sudara: sadržaj se RAZLIKUJE od oba roditelja (nije puko "uzmi topic" ili
    // "uzmi main"), jer samo tako kombinirani diff nedvosmisleno kaže "merge je dirnuo ovu putanju".
    std::fs::write(r.path().join("docs/conflict.md"), "merged resolution\n").expect("write");
    r.git(&["add", "-A"]);
    r.commit_merge(
        "merge topic",
        "2026-09-06T10:00:00+02:00",
        "2026-09-06T10:00:00+02:00",
    );

    let g = GitCli::new(r.path());
    // Ground truth: stari put PO DATOTECI, jedan poziv po putanji — ISTA metoda koju je M1 koristio
    // i koju ova cigla NE mijenja.
    let expected_conflict = g.last_change("docs/conflict.md").unwrap().unwrap();
    let expected_branch_only = g.last_change("docs/branch_only.md").unwrap().unwrap();
    let expected_plain = g.last_change("docs/plain.md").unwrap().unwrap();
    // Fixture mora doista imati TRI različita vremena, inače test ne dokazuje ništa (svi commiti
    // imaju eksplicitno različite GIT_AUTHOR_DATE/GIT_COMMITTER_DATE upravo zato).
    assert_ne!(expected_conflict, expected_branch_only);
    assert_ne!(expected_conflict, expected_plain);
    assert_ne!(expected_branch_only, expected_plain);

    let batch = g.last_changes("HEAD", &["docs"]).unwrap();
    assert_eq!(
        batch.get("docs/conflict.md").copied(),
        Some(expected_conflict),
        "sudar razrijesen U MERGE-COMMITU mora dati vrijeme mergea, ne ranije grane"
    );
    assert_eq!(
        batch.get("docs/branch_only.md").copied(),
        Some(expected_branch_only),
        "cist spoj ne smije lazno dati vrijeme merge-commita"
    );
    assert_eq!(batch.get("docs/plain.md").copied(), Some(expected_plain));
}

#[test]
fn last_changes_reads_non_ascii_file_names() {
    let r = Repo::init();
    r.commit(
        "docs/čšž.md",
        "# cro\n",
        "dodaj hrvatski naziv",
        "2026-09-01T08:00:00+02:00",
        "2026-09-01T08:00:00+02:00",
    );
    let g = GitCli::new(r.path());
    let expected = g.last_change("docs/čšž.md").unwrap().unwrap();
    let batch = g.last_changes("HEAD", &["docs"]).unwrap();
    assert_eq!(
        batch.get("docs/čšž.md").copied(),
        Some(expected),
        "core.quotepath mora biti iskljucen, inace kljuc s hrvatskim slovom nikad ne pogodi mapu"
    );
}
