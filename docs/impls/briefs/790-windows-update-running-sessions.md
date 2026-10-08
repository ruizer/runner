# 790 — Windows agent updates no longer require stopping sessions

Fix [#790](https://github.com/yicheng47/runner/issues/790). Jason requested this codex duo mission on 2026-10-08. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-790-windows-update-running-sessions`, on the existing branch `fix/790-windows-update-running-sessions`, created from `origin/main` at `0aef92ae`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory. `.worktrees/fix-782-rail-handle-truncate` and `.worktrees/fix-718-recents-project-inference` belong to other work; treat them as another machine's checkout.

## The bug

On Windows, Settings → Agents disables an agent's Update button while any session of that runtime is running, with "Stop the running Codex session first.", and the daemon refuses the update before running the updater. #533 added this guard because an in-use executable could make the updater fail. Jason reports that `codex update` works on Windows with Codex sessions running, so the guard only forces people to interrupt work.

## Decision

Lift the guard for every runtime on Windows, as Jason asked: Windows behaves like macOS. Update stays enabled while sessions run, the row shows the existing macOS caption ("1 running Codex session keeps 0.159.3 until it relaunches." / "N running … sessions keep … until they relaunch."), and running sessions are never stopped or restarted by an update. If an updater fails because a file is in use, its own error stays visible in the update's terminal modal; Runner adds no retry or stop prompt.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions, and issue #790.
- `crates/runner-app/src/surfaces/settings/agents.rs`: `update_action` (about line 1327), its call site (about line 791) and its tests (about lines 1691–1740).
- `crates/runner-app/src/platform_ui/{macos,windows,mod}.rs`: `AGENT_UPDATE_NEEDS_STOPPED_SESSIONS`.
- `crates/runner-daemon/src/ops/runtime.rs`: `runtime_update_spawn_spec` (about line 114) and its `#[cfg(windows)]` live-session refusal.
- `docs/features/archive/533-agent-cli-updates.md`, decision 4 (the platform-split guard) for the history; it is an archived record, do not edit it.

## Deliverables

1. **Remove the guard in both layers.** Delete the daemon's Windows refusal, drop `AGENT_UPDATE_NEEDS_STOPPED_SESSIONS` from the platform modules and `update_action`'s `needs_stopped_sessions` parameter, and remove anything that becomes unused (for example `live_session_counts`, if nothing else calls it). The caption logic keeps only the informational variant. The Update button is never disabled because of running sessions.
2. **Tests.** Replace the tests that enforce the blanket restriction: `update_action` with 0, 1 and several running sessions returns an enabled action with the right caption (singular and plural), and the daemon builds the update spawn spec while sessions of that runtime are live. Gate any Windows-only test or import with `#[cfg(windows)]`, and anything used only by `cfg(unix)` tests with `#[cfg(unix)]`.
3. **Docs.** If `docs/arch/` or either README states the Windows stop-first rule, update it; `README.md` and `README.zh-CN.md` change together. Otherwise no doc change.

No design file edits: the caption already exists on macOS. No change to the updater commands, the modal, or macOS behavior.

## Boundaries

Crews never run the development app or drive Jason's Runner; no live tests in this mission. Native Windows is unavailable here, so the Windows CI job is the Windows evidence; say what stays unverified natively (the updater's own behavior with a running session). Do not start extra agents, crews or subagents.

## Review, verification and authorization

The coder owns implementation and checks. The reviewer waits for an explicit Runner handoff, then reviews the full working-tree diff against #790 and this brief, must-fix findings first with file:line pointers. Focus: both layers lifted, nothing left that still blocks or stops sessions on update, no dead code left behind, macOS unchanged. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

Run `cargo test --locked -p runner-app --profile ci --no-fail-fast`, `cargo test --locked -p runner-daemon --profile ci --no-fail-fast`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), macOS updater Clippy (the same with `--features updater`), `cargo fmt --all --check` and `git diff --check`. Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`.

After a clean review, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the fix (for example `fix(settings): allow agent updates on Windows while sessions run`), pushing `fix/790-windows-update-running-sessions`, and opening a PR against main whose body says `Fixes #790`. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff: PR URL, what changed, tests and exit codes, CI result, the reviewer's verdict, and what Jason should check on the PC (Update enabled with a Codex session running, the caption, and the update completing while the session keeps working). Then stand by.
