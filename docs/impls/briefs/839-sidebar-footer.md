# 839 — One-row sidebar footer and the update icon in the header

Build [#839](https://github.com/yicheng47/runner/issues/839) (P2, 0.13). Jason requested this codex trio mission on 2026-10-08: coder → reviewer → qa, with qa checking the result visually in the live development app against the design. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/feat-839-sidebar-footer`, on the existing branch `feat/839-sidebar-footer`, created from `origin/main` at `888af6bf`. The root checkout stays on main. Do not create another branch or worktree, and do not share a Cargo target directory. `.worktrees/fix-814-busy-chat-redraw` belongs to another mission that is finishing its PR; treat it as another machine's checkout.

## The change

Follow the design. It comes from the liquid glass exploration (`design/specs/793-liquid-glass.pen`, footer node `NjWRU`, solid frame `J8vo2`), but only its solid form ships here: no glass material, translucency or blur. `.pen` files are not readable outside Pencil, so the design is exported as PNGs under `target/design-ref/839/` in this worktree:

- `solid-dark-full.png`: the whole window in the solid appearance, the reference for this change.
- `solid-dark-header-3x.png`, `solid-dark-footer-3x.png`: close-ups of the sidebar header and footer.
- `footer-2x.png`: the footer component at 2x.
- `glass-dark-*.png`, `glass-light-*.png`: the same layout on glass, for placement in light and dark only; ignore the material.
- `usage-popover-unchanged.png`: the usage popover, which does not change.

What changes:

1. **One footer row** replaces the bordered **WEEKLY USAGE** pill and the separate **Settings** row. A soft divider sits above it: a 1 px line that fades out at both ends.
2. **Usage** on the left: each enabled runtime's mark and weekly percentage, as today, with no label, border or fill. Transparent at rest, a soft fill on hover, and the hit area the same size in both states. Clicking it opens the existing usage popover. Percentage colors and the em dash for an unknown value stay as they are.
3. **Settings** at the right end of the row: a 32 px gear icon button that opens Settings as the old row did, with a tooltip.
4. **App update icon**: the green download icon shown when a Runner update is available moves from the footer's Settings row to the sidebar header, right after the **Runner** name, as in `solid-dark-header-3x.png`. Its tooltip and click behavior stay the same, including the separate macOS and Windows paths (`platform_ui::activate_update_hint`, `update_hint_tooltip`).

Collapsed-sidebar and narrow-width behavior must stay sensible: usage stays hidden while the sidebar is collapsed, as today, and at the narrowest sidebar width the entries and the gear do not overlap or clip.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions; issue #839; the PNGs above.
- `crates/runner-app/src/surfaces/app_shell.rs`: `usage_pill_element` (about :238), `render_usage_popover` (about :524), the update hint (about :981–:1025) and the footer's Settings row (about :1080–:1120); `update_dot_visible` (the agent-update dot on the usage popover's settings button) is unrelated and stays.
- `crates/runner-app/src/surfaces/sidebar/` and its `tests.rs`, which use `VisualTestContext`, `simulate_resize` and `debug_bounds` for layout checks; follow those patterns.
- `crates/runner-app/src/ui/tooltip.rs` and `ui/button.rs` for the existing icon button and tooltip.

## Deliverables

1. The change above, on macOS and Windows, matching the design's spacing, sizes and order.
2. Tests: the footer is one row; usage entries and the gear are present and the **WEEKLY USAGE** label and Settings text row are gone; clicking usage opens the popover; the gear opens Settings; with an update available the icon renders in the header and not in the footer; at a narrow sidebar width nothing in the footer overlaps. Reuse or update the existing `USAGE_PILL_*` debug selectors rather than leaving dead ones.

No design file edits, no README change.

## Validation

Run `cargo test --locked -p runner-app --profile ci --no-fail-fast`, workspace Clippy with warnings denied (`cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`), macOS updater Clippy (the same with `--features updater`), `cargo fmt --all --check` and `git diff --check`. Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`. If the default 256 file-descriptor limit fails unrelated tests, raise it only for the validation shell and say so. Gate Windows-only code and imports with `#[cfg(windows)]`.

## Review and QA

The reviewer waits for coder's Runner handoff, then reviews the whole working-tree diff against #839, this brief and the design PNGs, must-fix findings first with file:line pointers. Focus: the result matches the design; no behavior lost (popover, Settings, the update hint on both platforms, collapsed sidebar); the hit area does not change on hover; no unrelated layout changes. Iterate until the reviewer posts `NO REMAINING MUST-FIX ISSUES`.

Jason changed the order through Runner on 2026-10-08: after the clean review, coder squashes, pushes, opens the PR with **QA pending**, and starts CI before the QA handoff. Coder then freezes that exact PR head and hands it to qa as the crew conventions describe; qa tests the PR head commit. Only qa runs the development app; coder and reviewer never run it. qa follows `docs/tests/full-smoke-test.md` for the macOS launch: the `Runner Smoke Dev.app` wrapper, `NO_COLOR` and the inherited `RUNNER_CREW_ID`, `RUNNER_MISSION_ID`, `RUNNER_HANDLE` and `RUNNER_EVENT_LOG` removed, the development CLI by absolute path (`$HOME/Library/Application Support/com.wycstudios.runner-dev/bin/runner`), and `status --json` confirming the development endpoint. Never use bare `runner` for product checks. Before launching, check that no development app or daemon is running; if one is, ask Jason through `runner ask --human` and wait instead of stopping it. Launch with `RUNNER_DEV_UPDATE_AVAILABLE=0.99.0` in the app's environment so the update icon shows (a debug-build override in `updater.rs`). Verify computer use with a harmless action first.

qa starts no agent sessions. Checks, each with a screenshot compared against the design PNGs:

- Dark and light: the footer is one row matching `solid-dark-footer-3x.png` in order, spacing and icon size; the update icon sits right after **Runner** in the header as in `solid-dark-header-3x.png`, and is absent from the footer.
- Hovering usage shows the soft fill without moving or resizing anything; clicking it opens the usage popover, unchanged.
- The gear shows its tooltip and opens Settings; the update icon shows its tooltip (do not click through to an update).
- Collapsed sidebar: no broken footer. Narrowest sidebar width: no overlap or clipping; if computer use cannot drag the sidebar divider, mark that check Blocked and rely on the layout test.

Keep screenshots and the ID ledger in a scratch directory outside the repository, and report the verdict to coder with every check Passed, Failed, Blocked or Skipped and the screenshot paths. Then quit only the wrapper app qa launched and confirm with `ps` that nothing qa started survives. No `docs/tests` record; the verdict message is the record.

## Authorization

Do not start extra agents, crews or subagents. Do not touch Jason's installed Runner, its chats or missions, or global agent configuration.

After a clean review, Jason authorizes squashing all work on this branch, this brief included, into one commit on top of current `origin/main` with a subject that names the change (for example `feat(ui): one-row sidebar footer with the update icon in the header`), pushing `feat/839-sidebar-footer`, and opening a PR against main whose body says `Fixes #839` and is marked **QA pending**. Start CI, then hand the frozen PR head to qa. After qa's verdict, update the PR body with qa's screenshots or their paths and state qa's verdict as it is. If main has moved, rebase; never merge main into the branch. Review or CI fixes after the push are amended into the same commit and pushed with `git push --force-with-lease`; a fix that changes the layout goes back through reviewer and the affected qa checks. Drive CI green on macOS and Windows. Do not merge, delete the branch or worktree, or cut a nightly or release. Final Runner handoff from coder: PR URL, what changed, tests and exit codes, CI result, the reviewer's verdict, qa's verdict with screenshot paths, and what Jason should check by eye, including Windows after the next nightly. Then stand by.
