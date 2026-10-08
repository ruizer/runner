# 814 — One busy chat redraws the whole window

Fix [#814](https://github.com/yicheng47/runner/issues/814) (P2, 0.13). Jason requested this codex trio mission on 2026-10-08: coder → reviewer → qa, with qa measuring the fix's CPU cost in the live development app on macOS. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-814-busy-chat-redraw`, on the existing branch `fix/814-busy-chat-redraw`, created from `origin/main` at `856d0729`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory. `.worktrees/fix-817-runnerd-reliability` belongs to another mission; treat it as another machine's checkout.

## The bug

While one visible chat is busy (an agent's working spinner animating), the app uses 37–49% of one core; idle, about 1.2%, and `runnerd` stays below 1% throughout. A `sample` of the app shows the main thread laying out and painting the whole window on every frame: taffy's `compute_flexbox_layout` and `compute_block_layout`, `BoundsTree::insert`, `Scene::insert_primitive`, sprite sorting and `TerminalElement::prepaint`. The daemon connection is idle, so the cost is in redrawing, not the protocol. Expected: output in one session repaints that session's terminal at most once per frame without laying out the rest of the window again, and a busy spinner costs a few percent of a core. Measured on this Mac (Apple silicon, 28 cores) on 2026-10-06.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions; issue #814 in full (its diagnosis is the starting point, not a binding design).
- `docs/tech/gpui-rendering.md` and `docs/tech/terminal-rendering.md`: how a frame is built and how the terminal element paints. GPUI is gpui-pre 0.3.7 (`Cargo.lock`); read its source for how `cx.notify()` on one entity marks views dirty, and for view caching (`AnyView::cached` and friends), before choosing a design.
- `crates/runner-app/src/app_store.rs` about :300–330: every terminal's waker feeds one channel; the loop coalesces for 4 ms, bumps `revisions.terminal_wake` and calls `cx.notify()` on the `AppStore`, which the whole window observes.
- Who reacts to that wake: `NativeRoot` in `crates/runner-app/src/main.rs` about :1184 notifies itself, the window root, on every wake while a terminal is visible; `mission_workspace/state.rs` about :598 does the same while active; `sidebar/state.rs` about :112 checks live titles on each wake. `StoreRevisions::reactions_since` (`app_store.rs` about :141) already separates `terminal_wake` from data changes.
- `crates/runner-terminal/src/terminal.rs`: `TerminalMirror::feed_output` and the waker; `crates/runner-app/src/terminal/element.rs`: `TerminalElement::prepaint`.
- `crates/runner-app/src/surfaces/panes.rs` and `pane_layout.rs`: how chat panes, splits and the side panel are composed, and which of them are their own entities.

## Deliverables

1. **The fix.** A terminal wake re-renders only the view or views that draw a terminal whose content changed; the window root, sidebar, tab bar, side panels and other panes are not re-rendered or re-laid out on a wake unless something they display changed (a live title, cwd or status still updates the sidebar and pane headers as it does today). Repaints stay at most one per displayed frame. Choose the mechanism from GPUI's own model (per-terminal entities or notifications, cached views, or both) and say in the handoff why. Every surface that shows a terminal must stay live: direct chats, splits, the mission workspace, the chat and mission drawers, a second window on the same tab, and a session shown in two places at once.
2. **A render-count test.** Using the existing `VisualTestContext` patterns, feed output into one of two split terminals and assert that the other pane, the sidebar and the root are not re-rendered, while the busy terminal is; and that a live title change still re-renders the sidebar row. Keep the render counter a small test helper that later UI performance tests can reuse. The test must fail on `856d0729`; say in the handoff that you checked this.
3. **A regression case.** Add `TERM-PERF-02` to `docs/tests/regression/terminal.md` (the format and ID rules are in `docs/tests/regression/README.md`): one visible busy spinner on macOS, app CPU measured with `top` against the idle figure, smoke tier, origin #814. Update the README's per-tier counts.

No design file edits, no README change, and no change to how terminal content is drawn inside the element beyond what the fix needs.

Operator clarification through Runner on 2026-10-08: GPUI 0.3.7 walks ancestors in `mark_view_dirty` and always draws the root in `draw_roots`, so the literal root render-count assertion above is withdrawn. The root's own render may make a light composition pass, with no store reads, list building or terminal work. The test instead asserts that the busy terminal re-renders while the sidebar, tab bar, idle pane and side panel views are reused from cache, and that a live title change still re-renders the sidebar row. The CPU gate remains the deciding check.

## Validation

Run `cargo test --locked --workspace --no-fail-fast --profile ci`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), macOS updater Clippy (the same with `--features updater`), `cargo fmt --all --check` and `git diff --check`. Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`. Gate Windows-only tests and imports with `#[cfg(windows)]`, and anything used only by `cfg(unix)` tests with `#[cfg(unix)]`.

## Review and QA

The reviewer waits for coder's Runner handoff, then reviews the whole working-tree diff against #814 and this brief, must-fix findings first with file:line pointers. Focus: no surface that shows a terminal can go stale (splits, mission workspace, drawers, second window, a session shown twice, tab switches); sidebar titles, statuses and unread markers still update; the test really counts renders and would catch the regression; no caching that hides a real data change. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

After the clean review, coder freezes the source and hands it to qa as the crew conventions describe. Only qa runs the development app in this mission; coder and reviewer never run it. qa follows `docs/tests/full-smoke-test.md` for the macOS launch: the `Runner Smoke Dev.app` wrapper so computer use can select the exact build, `NO_COLOR` and the inherited `RUNNER_CREW_ID`, `RUNNER_MISSION_ID`, `RUNNER_HANDLE` and `RUNNER_EVENT_LOG` removed from the app and development CLI environment, and the development CLI by absolute path (`$HOME/Library/Application Support/com.wycstudios.runner-dev/bin/runner`), checking `status --json` reports the development endpoint. Never use bare `runner` for product checks; it addresses the installed app that hosts this crew. Before launching, check that no development app or daemon is running; if one is, ask Jason through `runner ask --human` and wait instead of stopping it. Verify computer use with a harmless action first; if it is unavailable, mark the visual checks Blocked and still take the CPU measurements.

**Measurement.** Before each measurement, check with `ps` or `top` that no `cargo`, `rustc` or other heavy process is running on the host, because other missions build here; wait until the host is quiet and record its load (`uptime`). Measure the development app's process with `top -l 5 -s 2 -pid <pid> -stats pid,cpu,command`, and `runnerd`'s alongside it:

- Idle: one chat open and idle, sidebar visible.
- Busy shell: a development shell terminal running a bounded spinner, for example `s='|/-\'; for i in $(seq 1 600); do printf '\r%s' "${s:$((i % 4)):1}"; sleep 0.1; done`, kept visible.
- Busy agent: one throwaway direct chat (codex, the cheapest available model, low effort) told "Run `sleep 90` in the shell, then reply exactly DONE.", kept visible while its working indicator animates.
- Busy beside idle: the busy shell in one pane of a two-pane split with an idle chat in the other, sidebar visible.

Pass: each busy case under 10% of a core, against 37–49% in the issue. Report the numbers either way; a figure between 10% and 15% is Passed with a note, above 15% is Failed. Then run these suite cases on the fix build: `TERM-RENDER-01`, `TERM-RESIZE-02`, `WS-SHELL-01`, `WS-SIDEBAR-02` and `WS-WINDOW-01` from `docs/tests/regression/`, using only the sessions listed here, with a screenshot each.

The throwaway direct chat and the development shells are the only sessions qa may start. Keep screenshots, `top` output and the ID ledger in a scratch directory outside the repository, and report the verdict to coder with every check classified Passed, Failed, Blocked or Skipped, the CPU table and the screenshot paths. Then archive the chat and shells qa created, quit only the wrapper app qa launched, and confirm with `ps` that nothing qa started survives. No `docs/tests` run record; the verdict message is the record.

## Authorization

Do not start extra agents, crews or subagents beyond qa's sessions above. Do not touch Jason's installed Runner, its chats or missions, or global agent configuration.

After a clean review and qa's verdict, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the fix (for example `fix(ui): repaint only the terminal that changed on output`), pushing `fix/814-busy-chat-redraw`, and opening a PR against main whose body says `Fixes #814`, carries qa's CPU table and states qa's verdict as it is. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`; a fix that changes rendering goes back through reviewer and the affected qa checks. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff from coder: PR URL, what changed and why that mechanism, tests and exit codes, CI result, the reviewer's verdict, qa's verdict with the CPU table and screenshot paths, and what Jason should check on the Windows PC after the next nightly. Then stand by.
