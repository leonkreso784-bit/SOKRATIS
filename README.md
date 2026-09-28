# Sokratis

**Track how you work, not just what you shipped.** A desktop app that attaches to a git repository,
computes work statistics, scores how clean the documentation is, and reports where the project is
heading — always with evidence. It watches several projects at once.

> **Version 1.0.0** (September 2026, Windows only, untagged). The installer is unsigned and, for now,
> meant for the author. **No license has been chosen yet** — see [License](#license). What has shipped:
> [docs/records/CHANGELOG.md](./docs/records/CHANGELOG.md).

## Why it exists

The author tracked his work on [Sokrat Study](https://www.sokratstudy.com) in a spreadsheet that a
Python script regenerated every day from git, a session diary and a plan. It worked, but it had limits:
charts were lost on every regeneration, it knew nothing about multiple projects or branches, and it
had a measurable bug in worked hours. Sokratis moves that logic into a real application:
**everything is derived from git on demand, only hand-entered data is stored, and instead of a
spreadsheet you get signals with evidence.**

The same numbers are available from a command line tool, so they can also run in scripts.

## What 1.0.0 does

- **Measures work across all local branches** — pace per day, kinds of work (planning, documentation,
  execution, polish, debugging), quality and speed indicators, phases, deliveries from the diary, and
  hours per branch. Hours are an estimate derived from commit times, and are labelled as such. All
  worktrees of one repository count as one project.
- **A dashboard per project** — an overview card for every project; a click opens eight sections
  (summary, pace, branches, kinds of work, phases, deliveries, indicators, documentation) with charts:
  bars by day/week/month, a calendar heatmap, time of day, a phase Gantt chart and trend lines.
- **Signals with evidence** — unmerged branch chains that have gone stale, and a diary lagging behind
  the code. A signal always names what triggered it; a new alert raises an OS notification.
- **Documentation health** — a 0–100 score from dead links, documents missing from the index, more
  than one active plan, a diary inside the product definition, a lagging diary and an oversized key
  file; the score is never shown without the list of findings.
- **Hand-entered data stays in the repository** — per-commit corrections of the kind of work and a list
  of visions live in `<repo>/.sokratis/`. The app keeps its project list and daily snapshots in a local
  SQLite database. No cloud, no accounts.
- **Explanation card** on every number and chart (what it measures, how, how to read it); Croatian and
  English interface; four themes; start with the system; animations can be turned off.
- **Command line** — `report`, `docs` and `signals` over any repository.

## Requirements

- Windows with **WebView2** (included in Windows 11).
- **git** on `PATH` — Sokratis reads history by running `git`.
- To build from source: the Rust toolchain via rustup (pinned in `rust-toolchain.toml`, fetched on the
  first `cargo` run), Visual Studio Build Tools with the C++ workload (MSVC target), and Node.js with
  npm for the interface.

## Running from source

Command line (JSON by default, `--table` for a terminal table):

```sh
cargo run -p sokratis-cli -- report  <path-to-repo> [--since YYYY-MM-DD] [--until YYYY-MM-DD] [--scope all|default] [--json|--table]
cargo run -p sokratis-cli -- docs    <path-to-repo>
cargo run -p sokratis-cli -- signals <path-to-repo>
```

The exit code of `signals` is a contract for pre-flight scripts: `0` no signals · `1` warn · `2` alert ·
`3` error or wrong usage.

Desktop app in development mode (uses its own database, separate from an installed copy):

```sh
cd apps/desktop
npm ci
npm run tauri dev
```

## Building the installer

```sh
cd apps/desktop
npm ci
npm run tauri build
```

This produces an NSIS installer in `target/release/bundle/nsis/` (per-user install, unsigned). The
version number has a single source: `[workspace.package]` in the root `Cargo.toml`.

## Conventions and the project profile

Without configuration Sokratis expects the conventions of its first user:

- a session diary in `docs/records/PROGRESS.md` with headings like `## 2026-09-28 (MODEL) — title`;
- a plan in `docs/plan/RASPORED.md`, documentation under `docs/` with `docs/README.md` as the index;
- a changelog in `docs/records/CHANGELOG.md`.

**Note:** the default profile starts measuring on **2026-08-29**. For any other repository, set `since`
in `.sokratis/profile.json`, together with whatever else differs — diary and plan paths, heading
patterns, the commit classifier, test paths and signal thresholds. Missing fields keep their defaults;
an unknown field is an error. Every field and its default is listed in
[docs/architecture/ARCHITECTURE.md](./docs/architecture/ARCHITECTURE.md) §4. A repository without these
conventions still gets its numbers from git; sections that need a diary or a plan simply stay empty.

## Stack

Rust (`core` · `io` · `store` · `cli`) · SQLite via `rusqlite` · Tauri 2 · Svelte 5 · Tailwind v4 ·
d3 maths under hand-written SVG charts.

## Documentation

The documentation is written in Croatian — the author reads it and learns Rust from it. The single
entry point is [docs/README.md](./docs/README.md).

## License

No license has been chosen yet. Until one is, all rights are reserved. Author: Leon Kreso.
