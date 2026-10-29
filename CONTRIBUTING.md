# Contributing to daily-commando

Thanks for wanting to help! A few rules keep the boat afloat.

## Toolchain

* Rust stable (`rustup`, or [mise](https://mise.jdx.dev/) via the bundled
  `mise.toml`).
* `docker compose` — only if you touch the `observability/` stack.

```sh
git clone https://github.com/R34SGA/daily-commando
cd daily-commando
cargo run -- -n 3 -t 5        # quick smoke test
```

## Ground rules

* **Headless core.** `Session` (`src/session.rs`) stays pure: no I/O, no
  clock reads, no terminal. Renderers and future backends consume `Tick`
  snapshots. If you need a side effect, put it in a renderer or in `main.rs`,
  never in the core.
* **No auto-advance.** The timer going negative and manual hand-over are
  intentional product decisions, not bugs.
* **EUPL-1.2.** By contributing you agree your work is licensed under it,
  like the rest of the codebase (see [LICENSE](LICENSE)).
* Names of real people are user data: they stay on the host, are cached
  locally only (`src/cache.rs`), and never travel in telemetry beyond the
  local Prometheus labels.

## Before you commit

```sh
cargo test          # required — must pass
cargo clippy --all-targets   # no new warnings on code you touched
```

The codebase predates rustfmt defaults, so `cargo fmt --check` is not
enforced; match the surrounding style of the file you edit instead.
Tests live next to the code (`#[cfg(test)]` modules); new domain behavior
in `Session` or `cache` deserves a unit test — TUI drawing is verified by
running it.

## Commit messages

Conventional Commits format:

```
feat(tui): add shuffle keyboard shortcut
fix(session): clamp soldiers to 1..=20
```

* Subject ≤ 50 chars, imperative mood, no trailing period.
* Body only when the *why* is not obvious from the diff; wrap at 72 chars.
* Scope when it helps: `tui`, `session`, `cache`, `telemetry`, `obs`, `cli`.

## Pull requests

1. Branch from `main` (`feat/…`, `fix/…`).
2. Keep PRs small and focused; one logical change per PR.
3. Run the three checks above and say so in the PR description.
4. Squash-merge is the default; your individual commits will be flattened.

## Reporting issues

Include your OS/terminal, the exact command line, and what you expected
versus what happened. A screenshot of the ASCII-art scene helps a lot.
