# Sokratis

**Track how you work, not just what you shipped.** A desktop app that attaches to a git repository,
computes work statistics, scores how clean the documentation is, and reports where the project is
heading — always with evidence. Built to watch several projects at once.

> 🚧 **Work in progress (September 2026).** Milestone 1 is closed — version 0.1.0: `core`, `io` and
> `cli` run against real git history, and parity with the spreadsheet it replaces is a test, not a
> claim. **Milestone 2 (the desktop app) is being built and will ship as 1.0.0.** The Rust side
> (core, profile, io, store, cli) and the Svelte interface are both in `main`, but the interface only
> runs in a browser (`npm run dev`) against mock data so far — the real Tauri shell is next, so there
> is no installable app yet. Where things stand:
> [docs/plan/ROADMAP.md](./docs/plan/ROADMAP.md) · what has shipped:
> [docs/records/CHANGELOG.md](./docs/records/CHANGELOG.md).

## Why it exists

The author tracked his work on [Sokrat Study](https://www.sokratstudy.com) in an Excel sheet that a
Python script regenerated every day from git, a session diary and a plan. It worked, but it had
limits: charts were lost on every regeneration, it knew nothing about multiple projects, it went
stale silently on the main branch, and it had a measurable bug in worked hours. Sokratis moves that
logic into a real application: **everything is derived from git on demand, only hand-entered data is
stored, and instead of a spreadsheet you get signals with evidence.**

## What it does

- **Work statistics:** pace per day, kinds of work, quality and speed indicators, phases and visions.
- **Documentation health:** dead links, documents missing from the index, more than one active plan,
  a diary lagging behind the code.
- **Direction signals:** unmerged branches, documentation lag and more — each one carries its
  evidence, never a bare number.
- **Multiple projects:** each project declares its conventions in `.sokratis/profile.json`; without
  a profile the defaults apply.

Everything stays local: no cloud, no accounts. Hand-entered data lives in `<repo>/.sokratis/`.

## Stack

Rust (`core` · `io` · `store` · `cli`) · SQLite through `rusqlite` · Tauri 2 · Svelte 5 ·
Tailwind v4 with the design tokens of Sokrat Study.

## Running it

From source, today (0.1.0 command line):

```
cargo run -p sokratis-cli -- report  <path-to-repo> [--since YYYY-MM-DD] [--until YYYY-MM-DD] [--json|--table]
cargo run -p sokratis-cli -- docs    <path-to-repo>
cargo run -p sokratis-cli -- signals <path-to-repo>
```

The toolchain is pinned in `rust-toolchain.toml` (1.98.1); rustup fetches it on the first `cargo`
run. The exit code of `signals` is a contract for pre-flight scripts: `0` no signals · `1` warn ·
`2` alert · `3` error or wrong usage.

## Documentation

The documentation is written in Croatian — the author reads it and learns Rust from it. The single
entry point is [docs/README.md](./docs/README.md).

## License

No license has been chosen yet. Until one is, all rights are reserved. Author: Leon Kreso.
