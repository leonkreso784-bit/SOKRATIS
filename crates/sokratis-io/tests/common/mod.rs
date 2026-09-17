//! Privremeni git-repo s FIKSNIM datumima: `GIT_AUTHOR_DATE`/`GIT_COMMITTER_DATE` čine test
//! determinističkim, a `tempfile::TempDir` briše mapu kad test završi (RAII — `Drop`).
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
}
