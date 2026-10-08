# 845 — Halve CI time: nextest and the slowest tests

Deliver [#845](https://github.com/yicheng47/runner/issues/845). Jason requested this mission on 2026-10-08. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/chore-845-ci-nextest`, on the existing branch `chore/845-ci-nextest`, created from `origin/main` at `e5158b5e`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory.

## Read first

- `AGENTS.md`, including Development Commands, Worktrees and Crew Missions.
- Issue #845: the timing tables, the five scope items, the non-goals and the verification bar. The issue wins over this brief on scope.
- `.github/workflows/ci.yaml`: both jobs, the `Test` steps (lines 57 and 102), the timings artifact, and the cache step.
- `Cargo.toml` `[profile.ci]` (line 53) and `[patch.crates-io]` (the vendored `alacritty_terminal`).
- `crates/runner-cli/tests/golden.rs` (`built_cli_goldens`, line 620; `UPDATE_RUNNER_GOLDENS`, line 695), `crates/runner-cli/tests/daemon_process.rs` (`exited_daemon_reports_status_and_log_path`, line 568), `crates/runner-cli/tests/hook_process.rs`, `crates/runner-terminal/tests/snapshot.rs` (`recording_split_points`, line 155).
- The static locks: `THEME_LOCK` (`runner-app/src/theme_snapshot.rs:32`), `CONFIG_LOCK` (`runner-daemon/src/runtimes/codex/codex_trust.rs:8`, `copilot/copilot_trust.rs:6`), `SETTINGS_LOCK` (`antigravity/agy_trust.rs:15`).
- `crates/runner-daemon/src/db/mod.rs` `build_pool` (about line 54) and `crates/runner-core/src/daemon_process.rs` `connect_or_spawn`.
- `docs/arch/windows.md` line 30, which repeats the CI test command.

## Constraint the issue does not state

The cache step saves only on `refs/heads/main` (`save-if`), and its key hashes `Cargo.toml`. Any `Cargo.toml` change, such as a `profile.ci` opt-level, makes every run on this PR compile cold. So measure the test-run time (what nextest reports) on the PR, and say which runs were cold. The warm end-to-end numbers in the verification bar can only be taken on main after merge; state that in the PR body rather than claiming them. Weigh this when choosing between splitting `recording_split_points` and raising its opt-level.

## Deliverables

1. **nextest in both jobs.** Replace the two `Test` steps with `cargo nextest run`, installed from a pinned prebuilt release (exact version, no `latest`). Keep `--locked`, `--no-fail-fast` and the `ci` cargo profile (that is `--cargo-profile ci`; nextest's own `--profile` selects a nextest profile), and keep the build-timings artifact (check `cargo nextest run --help` for the cargo options it forwards). Confirm no workspace crate has doc tests. Put nextest config in `.config/nextest.toml`. Do not configure retries: they would hide the real-daemon flakes this issue wants gone.
2. **Shared-state tests.** Under nextest each test is its own process, so a static mutex no longer serializes anything. For each lock above, find out what it guards. In-process state (a GPUI global, for example) is now isolated and needs nothing; a real file, a fixed path or process environment shared across tests goes into a one-thread test group. Look beyond the four named locks for tests that share a fixed path, port, socket name or environment variable. Cap the real-daemon binaries (`daemon_process`, `golden`, `hook_process`) with a test group, sized from what the runners can carry; the issue cites two Windows timing failures under load. Plain `cargo test` (still `make test`) must keep passing, so keep the static locks.
3. **Split `built_cli_goldens`** into separate tests that run in parallel, keeping one golden file and `UPDATE_RUNNER_GOLDENS`. Updating must still rewrite the whole file deterministically from all cases, and a missing or reordered case must still fail.
4. **Shorten `recording_split_points`** without losing coverage: per-recording tests, or a higher `profile.ci` opt-level for `runner-terminal` and `alacritty_terminal`, whichever is cheaper overall given the cache constraint. Say which and why, with numbers.
5. **The 30 s wait in `exited_daemon_reports_status_and_log_path`.** Measure first: confirm where runnerd spends the time on an invalid database before changing anything. If it is r2d2's default 30 s `connection_timeout` in `build_pool`, fix startup to fail fast on a bad database without shortening the timeout that `pool.get()` uses under normal contention. Fix it in the product; the test then gets fast on its own. Add or adjust a test that pins the fast failure.
6. **Docs.** Update the CI reproduction command in `AGENTS.md` (Development Commands) and `docs/arch/windows.md`, including how to install nextest locally. Leave `make test` on `cargo test`.

Keep the check names `Rust / macOS` and `Rust / Windows`. Non-goals from the issue hold: no larger or paid runners, no sccache, no dropped or weakened check (Clippy, Clippy with updater, fmt, the installer check, the Windows resource check, the lockfile check).

## Boundaries

Crews never run the dev app. Do not run `runner` or `runner-dev` commands that start, stop or kill sessions or the daemon; the real-daemon tests spawn their own isolated daemons, which is fine. Do not start extra agents, crews or subagents. Gate imports and helpers used only by `cfg(unix)` tests with `#[cfg(unix)]`, and Windows-only ones with `#[cfg(windows)]`; Windows CI has repeatedly broken on ungated imports.

## Review, verification and authorization

The coder owns implementation and checks. The reviewer waits for an explicit Runner handoff, then reviews the full branch diff against #845 with must-fix findings first and file:line pointers. Focus on: every test `cargo test` ran still runs under nextest (no binary or test silently dropped, ignored tests still ignored); the lock audit, with each lock's verdict; the golden split still detecting a missing case and updating the whole file; the database fix leaving normal pool waits unchanged; no weakened check; check names unchanged. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

Locally, run the workspace through `cargo nextest run --locked --workspace --no-fail-fast --cargo-profile ci` and through plain `cargo test --locked --workspace --no-fail-fast --profile ci`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), `cargo fmt --all --check`, and `git diff --check`. Install nextest locally only through the same pinned prebuilt release, under `~/.cargo/bin`. Record the exact commands and exit codes.

After a clean review, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the change (for example `ci: run workspace tests with nextest and split the slowest tests`), pushing `chore/845-ci-nextest`, and opening a PR against main whose body says `Closes #845`. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`. Drive CI green on macOS and Windows. For the verification bar you may re-run this PR's CI workflow with `gh run rerun`, up to three green runs per platform in total; do not dispatch other workflows. Do not merge, delete the branch or worktree, or cut a nightly or release.

The PR body carries the evidence table: per platform, the test-step and job time before (the issue's runs) and after (each of your runs, marked cold or warm), the passed and ignored totals before and after with every difference explained (a split test adds cases), the lock audit verdicts, and which real-daemon tests passed on how many consecutive runs. Final Runner handoff: PR URL, what changed, local commands and exit codes, the CI table, the reviewer's verdict, and what remains to measure on main after merge. Then stand by.
