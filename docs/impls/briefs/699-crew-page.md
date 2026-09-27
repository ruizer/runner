# 699 — Crew page and crew list

Implement [P1 #699](https://github.com/yicheng47/runner/issues/699), the second 0.12 release blocker. Jason asked on 2026-09-27 for a claude pair crew mission that ends in an open PR, not a merge.

Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/feat-699-crew-page`, on branch `feat/699-crew-page`. The mission's directory is this worktree. Its tip is this brief, on top of main `b3696cb` (the design and spec). Do not create another branch or checkout, touch the root checkout or another worktree, or share another worktree's target directory. If main moves and you need it, rebase onto `origin/main`; never merge main into the branch.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions (a mission lands as one commit).
- **`docs/features/699-crew-page.md`**: the spec. It wins over the issue and over this brief on any detail. The issue body still calls the crew list a non-goal; the spec, updated 2026-09-27, puts it in scope.
- **The design**, exported as PNGs in this worktree's ignored `target/design-ref/`: `crew-page.png`, `crew-page-missions-expanded.png`, `crew-page-editing.png`, `slot-popup.png`, `slot-popup-editing.png`, `crew-list.png`. Match their layout, hierarchy, spacing and colours with the existing theme tokens; the sample data is illustrative. Do not delete them, and do not open or edit `.pen` files.
- **The role pages that shipped in #720**, which this work mirrors and should share code with rather than copy: `surfaces/roles/detail.rs` (profile split, markdown prompt clamp, edit in place, Markdown | Preview), `surfaces/roles/list.rs` (the table, hover action, narrow-width column dropping), `ui/list.rs` (`PaginatedListPage` header and count).
- `crates/runner-app/src/surfaces/crews/`: `editor.rs`, `editor_sections.rs`, `slots.rs` (rows, drag reorder, slot menu, `set_crew_lead`), `list.rs`, `create.rs`, `add_slot.rs`, `overlays.rs`, `logic.rs`, `tests.rs`; `surfaces/start_mission.rs`; `surfaces/roles/{forms,edit,delete}.rs` (the role edit drawer, now reached only from a crew slot).
- Backend: `ops/mission.rs` (the `crew.goal` fallback at `:298` and on resume near `:690`), `repo/crew.rs` (`SEARCH_PREDICATE`), `ops/slot.rs` (`reorder`, slot override updates).

## Deliverable

1. **Crew page, profile split** (`crew-page.png`). Left column: the crew picture at 95 px, name, `5 slots · lead @lead`, Start mission (primary, opens the existing Start mission dialog for this crew) and Edit; then Slots with Add slot (the existing add-slot modal); then created/updated and the short id. Right: the Team conventions card, its caption, and the Missions card.
2. **Crew picture**, one helper used at 95 px and 25 px: one slot fills the tile, two sit side by side centred vertically, three are two over one with the third centred, four or more are a 2×2 grid of the first four. Nothing drawn for a missing slot.
3. **Slot rows.** `RoleAvatar` seeded with the slot handle, the handle, LEAD, the effective runtime (mark and name), model and effort with an amber dot on each overridden value, and `role @x`; an "overridden for this slot" legend when any slot overrides. Drag to reorder stays: a grip in the left gutter on the hovered row only, the whole row as the drag source, a click opening the popup, and saving through the existing reorder path.
4. **Slot popup** (`slot-popup.png`, `slot-popup-editing.png`). Clicking a slot opens a popover beside it and marks the row selected: avatar, handle, the role's name and Open role, close; Runtime, Model and Effort with the role's value beside each; the command; a clamped prompt preview from the role; and Edit overrides, Set as lead, Remove. Edit overrides turns the three values into selects with Reset per overridden value, "Saves to this slot only.", Cancel and Save, and saves the slot's overrides only. Esc and a click outside close it. This replaces the slot menu's Edit role drawer; once nothing reaches the role edit drawer, remove its code, keeping the role page's in-place edit working.
5. **Team conventions.** Markdown through the shared renderer, `N lines · X KB`, clamped with Show all like the role prompt. Edit in place (`crew-page-editing.png`): EDITING tag, the name as a field, Save and Cancel, "Unsaved changes. Slots save on their own.", the conventions as a mono editor with Markdown | Preview opening on Markdown and a neutral border, the missions card dimmed. Name and conventions save together; slot changes keep saving on their own.
6. **Missions card** from the missions the app store already holds, filtered by crew (the store excludes archived missions; count what it has and do not add a query). Header `Missions` with `N run · M live`. Rows: status icon, title, `aborted` in red when aborted, duration, date; a row opens the mission. The four latest show; a footer "Show all N missions ⌄" expands the full list in place, newest first, and then reads "Show less ⌃" (`crew-page-missions-expanded.png`). No footer at four or fewer. Collapsed whenever a crew opens.
7. **Purpose and default goal leave the app.** The page, the list, the create form and the search hint stop showing or asking for purpose; crew search stops matching purpose and goal (`SEARCH_PREDICATE`); the Start mission dialog stops using `crew.goal`; `ops::mission` stops falling back to `crew.goal` on start and on resume. The columns and the CLI's `--purpose` and `--goal` stay. The search change is a second backend touch beside the fallback: correct the spec's "only backend change" sentence on this branch.
8. **Crew list as a table** (`crew-list.png`, spec "Crew list as a table"): the picture, name and `N slots · lead @x`; Runtimes as marks with slot counts; Missions; Last mission (the latest start as `Sep 23, 09:40`, `—` if none, or `1 mission running` in the accent). A row opens the crew; hovering turns its play icon into a Start mission button for that crew; ⋯ keeps its menu. New subtitle and search placeholder. Narrow widths drop Missions, then Runtimes, then Last mission.
9. **Narrow windows** as on the role pages: the page stacks the right column under the left below the same breakpoint, nothing scrolls sideways, long text truncates.
10. **Tests** in `surfaces/crews/tests.rs` and wherever shared helpers land: the picture layout for one to five slots; the list cells and the running state; the missions card counts, the four-row cut, the footer text and the reset on crew change; the popup opening, override save with Reset, Set as lead and Remove; edit in place save and cancel; reorder still saving; the goal fallback gone on start and resume (backend tests); search no longer matching purpose or goal. Update assertions the change invalidates. Gate any test import or helper used only by `cfg(unix)` tests with `cfg(unix)`, since Windows clippy fails on unused ones.
11. **Docs, same diff.** If implementation forces a deviation from the spec, update the spec on this branch and say why in the handoff.

Out of scope: the role pages beyond shared helpers, the add-slot form beyond visual consistency, a missions page, database or CLI changes, new fields, `.pen` files, README screenshots.

## Validation

Run each of these and report its exit code:

- `cargo test --locked -p runner-app --profile ci --no-fail-fast`
- `cargo test --locked -p runner-backend --profile ci --no-fail-fast`
- `cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`

Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`.

Crews never run the dev app. Do not start, stop, restart or type into Jason's Runner apps, chats or missions, and never open Jason's real Runner database (`~/Library/Application Support/com.wycstudios.runner*`). Jason smoke-tests the UI. Native Windows is unavailable; say what is unverified there.

## Crew handoff and authorization

The coder owns implementation, tests and fixes. The reviewer waits for an explicit Runner handoff, then reviews the whole working-tree diff against the spec, the PNGs and this brief, must-fix findings first with file:line pointers. It checks in particular:

- that a slot override saves to that slot only, Reset restores the role's value, and the role itself never changes from the popup;
- that no mission start or resume path still reads `crew.goal`;
- that the missions card and list read the store's missions correctly for running, aborted and never-run crews;
- that removing the drawer left the role page's edit in place intact.

Iterate through Runner until the reviewer posts `NO REMAINING MUST-FIX ISSUES`. No extra agents, crews or subagents.

After the clean review, and only then, Jason authorizes:

- **One commit.** Squash everything on the branch, this brief included, into a single commit on top of `main`: imperative subject naming the change (for example `feat(ui): redesign the crew page and crew list`), no co-author trailers.
- **Push** with `git push -u origin feat/699-crew-page`.
- **Open the PR** with `gh pr create --base main`. The body carries `Closes #699`, a summary, test evidence, a manual check for Jason (crews of one, two, three and five slots; a slot override saved and reset; Set as lead; a crew with more than four missions expanded and folded; a crew with a running mission in the list; editing name and conventions; search without purpose; both pages at the minimum window width), what is unverified, and no agent session links.
- **Watch CI** with `gh pr checks <n> --watch` until nothing is pending, on both macOS and Windows. Fold any fix into the commit with `git commit --amend`, have the reviewer check it, and push with `git push --force-with-lease`.

**Do not merge**, delete the branch or worktree, or cut a nightly or release.

The final handoff goes to everyone through Runner: the PR URL and CI result, changed files, checks with exit codes, any spec deviation, what is untested or unverified, and the reviewer's verdict. Then both slots stand by.
