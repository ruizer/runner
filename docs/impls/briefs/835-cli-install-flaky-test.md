# 835 — Flaky cli_install test on macOS CI

Fix [#835](https://github.com/yicheng47/runner/issues/835) (P3). Jason requested this codex duo mission on 2026-10-08. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-835-cli-install-flaky-test`, on the existing branch `fix/835-cli-install-flaky-test`, created from `origin/main` at `257937ca`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory. `.worktrees/fix-814-busy-chat-redraw` belongs to another mission whose qa slot is running the development app; treat it as another machine's checkout and never touch the development app or its `runnerd`.

## The flake

`cli_install::tests::installing_while_daemon_is_locked_updates_only_the_cli` in `crates/runner-core/src/cli_install.rs` failed once on macOS CI (first attempt of run 37742637783, PR #832 head `08b1830d`) and passed on the re-run. The last assertion (`:164`) found `bin/runnerd` still at `OLD-BUILD` after the test dropped its daemon-lock handle and installed again. `up_to_date` hashes contents without caching, so the third `install_from_source` most likely saw the lock as contended. The issue's hypothesis, unverified: another test in the same `runner-core` test binary forks a child (the daemon spawn uses `pre_exec`, so no `posix_spawn`) while the lock file is open, and the child's duplicated descriptor keeps the `flock` held until `exec`. #832 added process-spawning tests to `daemon_process.rs`.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions; issue #835 in full.
- `crates/runner-core/src/cli_install.rs`: `install_from_source` (:44), `install_binary`, `up_to_date` (:111) and the test (:129).
- `crates/runner-core/src/daemon_process.rs`: `lock_file`, the spawn with `pre_exec` (about :90–:120), `wait_unlocked`, and its tests, including those #832 added.

## Deliverables

1. **The cause, shown.** Reproduce the failure before fixing it: run the test inside the full `runner-core` test binary in a loop, under load if needed, or write a focused test that forces the suspected overlap. Report the reproduction rate before the fix. If the hypothesis is wrong, follow the evidence and say what the cause was.
2. **A deterministic fix where the cause lives.** If it is a test-only race, make the test independent of other tests' spawns without weakening what it checks (the CLI updates while the daemon is locked, the daemon only after the lock is released). If the same race can affect production — an install, an app start or a CLI start seeing a released lock as held because a sibling fork briefly holds it — fix it in product code and say which path was exposed. No unrelated changes.
3. **Proof.** The same loop that reproduced it passes at least 200 consecutive runs after the fix; give the command and counts.

## Validation

Run `cargo test --locked --workspace --no-fail-fast --profile ci`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), macOS updater Clippy (the same with `--features updater`), `cargo fmt --all --check` and `git diff --check`. Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`. If the default 256 file-descriptor limit fails unrelated tests, raise it only for the validation shell and say so. Gate Windows-only tests and imports with `#[cfg(windows)]`, and anything used only by `cfg(unix)` tests with `#[cfg(unix)]`.

## Review and authorization

The reviewer waits for coder's Runner handoff, then reviews the working-tree diff against #835 and this brief, must-fix findings first with file:line pointers. Focus: the reproduction is real, the fix removes the race instead of retrying around it, the test still proves both halves of its claim, and any product-path exposure is handled. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

No live tests, no development app, no extra agents, crews or subagents. Do not touch Jason's installed Runner or global agent configuration.

After a clean review, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the fix, pushing `fix/835-cli-install-flaky-test`, and opening a PR against main whose body says `Fixes #835`, with the cause, the reproduction rate before and the run count after. If main has moved, rebase; never merge main into the branch. Fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff: PR URL, the cause, what changed, the before and after numbers, tests and exit codes, CI result and the reviewer's verdict. Then stand by.
