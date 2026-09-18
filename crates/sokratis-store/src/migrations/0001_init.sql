-- M2/1: shema iz speca ARHITEKTURA_M2.md §4.1. Tablice pune cigle M2/15 (registar), M2/16 (postavke),
-- M2/17 (profil i snimke), M2/18 (keš — uvjetno, po mjerenju M2/11).
CREATE TABLE project (
  id             INTEGER PRIMARY KEY,
  name           TEXT NOT NULL,
  root_path      TEXT NOT NULL,
  git_common_dir TEXT NOT NULL UNIQUE,
  added_at       INTEGER NOT NULL,
  last_seen_at   INTEGER NOT NULL
);
CREATE TABLE project_worktree (
  project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  path       TEXT NOT NULL,
  branch     TEXT NOT NULL,
  seen_at    INTEGER NOT NULL,
  PRIMARY KEY (project_id, path)
);
CREATE TABLE setting (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
CREATE TABLE project_setting (
  project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  key        TEXT NOT NULL,
  value      TEXT NOT NULL,
  PRIMARY KEY (project_id, key)
);
CREATE TABLE profile_seen (
  id         INTEGER PRIMARY KEY,
  project_id INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  json       TEXT NOT NULL,
  first_seen INTEGER NOT NULL,
  UNIQUE (project_id, json)
);
CREATE TABLE snapshot (
  project_id      INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  taken_on        TEXT NOT NULL,
  profile_seen_id INTEGER NOT NULL REFERENCES profile_seen(id),
  metric          TEXT NOT NULL,
  value           REAL NOT NULL,
  kind            TEXT NOT NULL,
  PRIMARY KEY (project_id, taken_on, metric)
);
CREATE TABLE commit_cache (
  project_id  INTEGER NOT NULL REFERENCES project(id) ON DELETE CASCADE,
  sha         TEXT NOT NULL,
  author_time INTEGER NOT NULL,
  commit_time INTEGER NOT NULL,
  date        TEXT NOT NULL,
  commit_date TEXT NOT NULL,
  subject     TEXT NOT NULL,
  files_json  TEXT NOT NULL,
  PRIMARY KEY (project_id, sha)
);
