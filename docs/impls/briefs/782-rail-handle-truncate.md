# 782 — Mission rail session card truncates a long handle

Fix [#782](https://github.com/yicheng47/runner/issues/782). Jason requested this codex trio mission on 2026-10-08: coder → reviewer → qa, with QA checking the fix visually in the live development app. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-782-rail-handle-truncate`, on the existing branch `fix/782-rail-handle-truncate`, created from `origin/main` at `4726184b`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory. `.worktrees/fix-718-recents-project-inference` belongs to other work; treat it as another machine's checkout.

## The bug

In a mission's rail, a session card with a long handle does not truncate it. The slot controls on the right draw over the end of the handle, and the restart icon sits on top of the LEAD badge. Expected: the handle shrinks and ends in "…", its tooltip shows the full handle, and the LEAD badge and the controls keep their own space with no overlap at any rail width.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions, and issue #782 (its diagnosis is the starting point, not a binding design).
- `crates/runner-app/src/surfaces/mission_workspace/rail.rs`: the card header row (about lines 297–332). The left group (avatar, handle, LEAD badge) has `min_w(0)` but no `flex_1`, and the handle's `Tooltip` wrapper is not `.expand()`ed, so the inner `.truncate()` never gets a narrower box. `controls` is built around line 215. The mission title at about line 524 already uses `.expand()`.
- `crates/runner-app/src/ui/tooltip.rs`: `expand` (line 92) gives the trigger `min_w(0)` + `flex_1` (line 112).
- `crates/runner-app/src/ui/avatar.rs`: `lead_badge` (line 153). It and the controls group must not shrink.
- `crates/runner-app/src/surfaces/sidebar/rows_render.rs` (about line 409) and commit `485dd477` (#769): the sidebar's working truncation, and its tests in `sidebar/tests.rs`, which use `VisualTestContext`, `simulate_resize` and `debug_bounds`. Truncating text in a `min_w(0)` column can render only "…" on its first measure; follow the sidebar's pattern.
- `crates/runner-app/src/surfaces/mission_workspace/tests.rs`: existing rail tests (`slot_rail_actions_match_status`, about line 443) and how they build a workspace.

## Deliverables

1. **The fix.** The handle truncates with "…", the tooltip still shows the full `@handle`, and the avatar, LEAD badge and controls keep their full size. Lead and non-lead cards both work. Short handles look exactly as before. No change to the card's other rows, the controls themselves, or any other surface.
2. **A layout test.** At a narrow rail width, with a long lead handle (32 characters, the handle limit), assert that the handle's bounds do not intersect the LEAD badge's or the controls' bounds, and that the handle is narrower than its unconstrained text, so it really truncated rather than clipped or collapsed to "…" only. Add debug selectors where needed. The test must fail on `4726184b`; say in the handoff that you checked this.

No design file edits; there is no `.pen` change for this bug. No README change.

## Validation

Run `cargo test --locked -p runner-app --profile ci --no-fail-fast`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), macOS updater Clippy (the same with `--features updater`), `cargo fmt --all --check` and `git diff --check`. Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`. Windows CI has repeatedly failed on imports or helpers used only by `cfg(unix)` tests; gate any such import or helper with `#[cfg(unix)]`.

## Review and QA

The reviewer waits for coder's Runner handoff, then reviews the whole working-tree diff against #782 and this brief, must-fix findings first with file:line pointers. Focus: nothing but the handle shrinks, the test would catch the regression and the "…"-only first measure, and no unrelated layout changes. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

After the clean review, coder freezes the source and hands it to qa as the crew conventions describe. Only qa runs the development app in this mission; coder and reviewer never run it. qa follows `docs/tests/full-smoke-test.md` for the macOS launch: the `Runner Smoke Dev.app` wrapper so computer use can select the exact build, `NO_COLOR` and the inherited `RUNNER_CREW_ID`, `RUNNER_MISSION_ID`, `RUNNER_HANDLE` and `RUNNER_EVENT_LOG` removed from the app and development CLI environment, and the development CLI by absolute path (`$HOME/Library/Application Support/com.wycstudios.runner-dev/bin/runner`), checking `status --json` reports the development endpoint. Never use bare `runner` for product checks; it addresses the installed app that hosts this crew. Before launching, check that no development app or daemon is running; if one is, ask Jason through `runner ask --human` and wait instead of stopping it. Verify computer use with a harmless action first; if it is unavailable, mark the visual checks Blocked and report.

Setup, all through the development CLI: one throwaway role with handle `qa782-long-lead-handle-truncates` (codex, the cheapest available model, low effort), a one-slot crew with that slot as lead, and one mission in a new canonical scratch directory with the goal "Reply exactly ACK and stay idle. Do not run tools." This is the only agent session qa may start.

Checks, each with a screenshot of the rail card:

- At the default rail width: the handle ends in "…", the LEAD badge and every slot control are fully visible, and nothing overlaps.
- At the narrowest rail width the rail allows: the same.
- Hovering the handle shows the full `@qa782-long-lead-handle-truncates`.
- A short handle is unchanged: compare with any existing short-handle card in the development app, or say none was available.

Keep screenshots and the ID ledger in a scratch directory outside the repository, and report the verdict to coder with every check classified Passed, Failed, Blocked or Skipped and the screenshot paths. Then stop and archive the test mission, quit only the wrapper app qa launched, and confirm with `ps` that nothing qa started survives. Keep the throwaway role and crew (deleting a crew destroys its mission metadata) and list them. No `docs/tests` record for this bug; the verdict message is the record.

## Authorization

Do not start extra agents, crews or subagents beyond qa's one test session above. Do not touch Jason's installed Runner, its chats or missions, or global agent configuration.

After a clean review and qa's verdict, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the fix (for example `fix(ui): truncate long handles in the mission rail session card`), pushing `fix/782-rail-handle-truncate`, and opening a PR against main whose body says `Fixes #782` and states qa's verdict as it is. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`; a fix that touches the layout goes back through reviewer and the affected qa checks. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff from coder: PR URL, what changed, tests and exit codes, CI result, the reviewer's verdict, qa's verdict with screenshot paths, and what Jason should check by eye. Then stand by.
