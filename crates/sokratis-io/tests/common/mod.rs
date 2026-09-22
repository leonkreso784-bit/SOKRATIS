//! Privremeni git-repo s FIKSNIM datumima: `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE` čine test
//! determinističkim, a `tempfile::TempDir` briše mapu kad test završi (RAII — `Drop`).
//!
//! Cigla M2/11, krug popravka 1 (last_changes i merge-commiti): `try_merge` i `commit_merge` grade
//! PRAVI merge sa sudarom u testnom repou — `try_merge` namjerno NE puca na sudaru (za razliku od
//! `git()`, koji puca na svakoj grešci), jer je sudar OČEKIVAN ishod fixturea.
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

pub struct Repo {
    pub dir: TempDir,
}

impl Repo {
    pub fn path(&self) -> &Path {
        self.dir.path()
    }

    pub fn git(&self, args: &[&str]) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(self.path())
            .args(args)
            .output()
            .expect("git");
        assert!(
            out.status.success(),
            "git {:?}: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    }

    pub fn init() -> Repo {
        let dir = tempfile::tempdir().expect("tempdir");
        let r = Repo { dir };
        r.git(&["init", "-q", "-b", "main"]);
        r.git(&["config", "user.email", "t@t"]);
        r.git(&["config", "user.name", "T"]);
        r.git(&["config", "commit.gpgsign", "false"]);
        r
    }

    /// Commit s datumom autora `a` i commita `c` (ISO 8601 s pomakom), datoteka `path` dobiva `content`.
    pub fn commit(&self, path: &str, content: &str, msg: &str, a: &str, c: &str) -> String {
        let full: PathBuf = self.path().join(path);
        std::fs::create_dir_all(full.parent().expect("parent")).expect("mkdir");
        std::fs::write(&full, content).expect("write");
        self.git(&["add", "-A"]);
        let out = Command::new("git")
            .arg("-C")
            .arg(self.path())
            .env("GIT_AUTHOR_DATE", a)
            .env("GIT_COMMITTER_DATE", c)
            .args(["commit", "-q", "-m", msg])
            .output()
            .expect("git commit");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        self.git(&["rev-parse", "--short", "HEAD"])
    }

    /// Pokreće `git merge --no-commit --no-ff <branch>` i NE PUCA na sudaru (za razliku od `git()`
    /// koji puca na svakoj grešci) — sudar je OČEKIVAN ishod za merge-fixture testove. `--no-commit`
    /// ostavlja rezultat staged (čist ili sa sudarom) da test sam odluči treba li ručno razrješenje
    /// prije `commit_merge`.
    /// `#[allow(dead_code)]`: `mod common` se prevodi ZASEBNO za svaku `tests/*.rs` datoteku — ovu
    /// metodu koristi samo `last_changes.rs`, pa bi ostali test-binarni ciljevi inače prijavili
    /// „nikad korišteno" i pali na `cargo clippy -D warnings`.
    #[allow(dead_code)]
    pub fn try_merge(&self, branch: &str) {
        Command::new("git")
            .arg("-C")
            .arg(self.path())
            .args(["merge", "--no-commit", "--no-ff", branch])
            .output()
            .expect("git merge");
    }

    /// Dovršava merge (nakon što je `try_merge` pripremio radno stablo, a test po potrebi
    /// razriješio sudar i `git add -A`) s FIKSNIM datumom — isti obrazac kao `commit()`, ali bez
    /// pisanja datoteke jer je merge to već učinio. Vidi napomenu o `#[allow(dead_code)]` uz
    /// `try_merge`.
    #[allow(dead_code)]
    pub fn commit_merge(&self, msg: &str, a: &str, c: &str) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(self.path())
            .env("GIT_AUTHOR_DATE", a)
            .env("GIT_COMMITTER_DATE", c)
            .args(["commit", "-m", msg])
            .output()
            .expect("git commit merge");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        self.git(&["rev-parse", "--short", "HEAD"])
    }

    /// M2/36 (spec §13.7, R17): commit BEZ ijedne datoteke (`--allow-empty`) — točno ono što „Dodaj
    /// projekt" donese kad korisnik doda tek `git init`-iran repo s jednim praznim commitom. Isti
    /// obrazac kao `commit()`/`commit_merge` (FIKSNI `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE`), samo
    /// bez pisanja/`add`-anja jer nema sadržaja. Vidi napomenu o `#[allow(dead_code)]` uz `try_merge`
    /// — ovaj pomoćnik koristi samo `project.rs`.
    #[allow(dead_code)]
    pub fn commit_empty(&self, msg: &str, a: &str, c: &str) -> String {
        let out = Command::new("git")
            .arg("-C")
            .arg(self.path())
            .env("GIT_AUTHOR_DATE", a)
            .env("GIT_COMMITTER_DATE", c)
            .args(["commit", "--allow-empty", "-q", "-m", msg])
            .output()
            .expect("git commit --allow-empty");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        self.git(&["rev-parse", "--short", "HEAD"])
    }
}
