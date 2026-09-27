# 726 — Text fields paint and hit-test one layout

Fix [P1 #726](https://github.com/yicheng47/runner/issues/726). Jason asked on 2026-09-27 for a claude pair crew mission that ends in an open PR, not a merge.

Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-726-text-field-element`, on branch `fix/726-text-field-element`. The mission's directory is this worktree. Its tip is this brief, on top of main `6c073f5`. Do not create another branch or checkout, touch the root checkout or another worktree, or share another worktree's target directory. If main moves and you need it, rebase onto `origin/main`; never merge main into the branch.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions (a mission lands as one commit).
- **Issue #726**: the cause analysis and direction. Its line numbers are from `c6ef31e`; `crates/runner-app/src/ui/field.rs` has moved since (`TextFieldLayout` `:341`, mouse handlers `:757`, `render_text`/`render_text_line` `:837`, `bounds_for_range` `:1023`, `Render` `:1057`, `input_caret` `:1799`). Its caller list is stale too: `rg 'TextField::textarea'` gives the live one (7 files).
- `gpui-ce` 0.3.3 `examples/input.rs` in `~/.cargo/registry/src/index.crates.io-*/gpui-ce-0.3.3/`: the pattern, single-line. Also `src/elements/text.rs` (`request_measured_layout`, and the width ceil at `:419` that causes the drift).
- `~/repos/gui/gpui-kit/crates/base/src/input/base/element.rs`: `layout_selections`, `layout_cursors`, the window-level `MouseMoveEvent` listener and `auto_scroll` (`:469`–`:500`). It targets another GPUI fork; read for the approach, take no dependency.

## Deliverable

1. **One custom `Element` renders editable text** for both kinds, textarea and single-line input. It shapes the text once per layout with the style it inherits (font, size, line height, colour come from the parent `div` as today) and wraps at the width it is given, then uses that one layout for everything: painting glyphs, selection quads, the marked-text underline and the caret, and every index↔point mapping. Nothing in it hard-codes padding or row height: the element's bounds are the content box. Delete `render_text_line`, the per-grapheme divs, the inline `input_caret`, `TextFieldLayout` and the hit-test `canvas`; the input-handler registration moves into the element's paint.
2. **Sizing keeps every caller working unchanged.** The textarea's height comes from wrapping at the real width, so use `request_measured_layout` (as `StyledText` does) and cache the shaped lines per `(text, width, style)`. The scroll container, `auto_grow(max_rows)`, `fill_height()`, `with_scrollbar` and the base `rows` must behave as today. The placeholder, `compact_path` and `truncate_unfocused` displays are not editable views and may stay as they are.
3. **Selection and caret.** Selection paints one quad per visual row, including empty lines and a small block at a selected line end, in today's colour (`theme::with_alpha(theme::accent(), 0.267)`). The caret is a 1 px painted quad at the position the layout gives, so it never pushes glyphs. Click, shift-click, double-click word and triple-click line land on the grapheme under the pointer everywhere: along long lines, after soft wraps, at line ends, on empty lines, below the last line.
4. **Drag past the edge.** While a selection drag is in progress, a window-level mouse-move listener keeps extending the selection when the pointer leaves the field, and a textarea scrolls toward the pointer when it is above or below the field, continuing while the button stays down, and stops on mouse up anywhere.
5. **Keyboard, now that the layout exists.** Up and Down in a textarea move by visual row, keeping the caret's x (Shift extends); at the first or last row they go to the start or end. The textarea scrolls to keep the caret visible after typing, pasting and moving. The single-line input keeps today's behaviour; horizontal scrolling there is out of scope.
6. **IME.** `bounds_for_range` returns the range's bounds from the layout instead of the whole field, and `character_index_for_point` uses the same mapping as the mouse.
7. **Cost.** No per-grapheme elements; a long prompt shapes once per layout, not per mouse move.
8. **Tests** in `field.rs`'s test module or a sibling. The GPUI test platform's `NoopTextSystem` gives every character the same advance on every OS, so assert exact indices: click round-trips at line ends, after a soft wrap, on an empty line and below the text; double- and triple-click; a drag that leaves the field keeps extending; Up/Down across a soft wrap and at the edges; `auto_grow` height for 1 and many rows; `bounds_for_range` inside the field. Drive them through a test window with simulated events where you can. Imports, statics and helpers used only by `#[cfg(unix)]` tests get `#[cfg(unix)]` themselves; macOS clippy cannot catch this and Windows CI will.

Out of scope: undo/redo (#728, a separate PR; do not restructure `TextBuffer` beyond what this needs), horizontal scrolling in single-line inputs, callers' layouts, and any new public API. `TextField`'s public API stays as it is.

## Validation

Run each of these and report its exit code:

- `cargo test --locked -p runner-app --profile ci --no-fail-fast`
- `cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`

Log each to a file and read `$?` from the command itself; never pipe a gate through `tail` or `grep`.

Crews never run the dev app. Do not start, stop, restart or type into Jason's Runner apps, chats or missions, and never open Jason's real Runner database (`~/Library/Application Support/com.wycstudios.runner*`). Jason smoke-tests the UI. Native Windows is unavailable; say what is unverified there.

## Crew handoff and authorization

The coder owns implementation, tests and fixes. The reviewer waits for an explicit Runner handoff, then reviews the whole working-tree diff against the issue and this brief, must-fix findings first with file:line pointers. It checks in particular:

- that paint and hit-testing read the same layout, with no leftover second shaping path or hard-coded metrics;
- that the textarea's height, scrolling, `auto_grow`, `fill_height` and scrollbar still behave for every caller (mission composer, crew conventions, role prompt, Settings MCP and Skills, Start mission, overlay);
- that the drag listener is removed or inert once the button is up and never leaks across fields;
- that key interceptors, Enter behaviour, IME composition and disabled fields are unchanged.

Iterate through Runner until the reviewer posts `NO REMAINING MUST-FIX ISSUES`. No extra agents, crews or subagents.

After the clean review, and only then, Jason authorizes:

- **One commit.** Squash everything on the branch, this brief included, into a single commit on top of `main`: imperative subject naming the change (for example `fix(ui): paint and hit-test text fields from one layout`), no co-author trailers.
- **Push** with `git push -u origin fix/726-text-field-element`.
- **Open the PR** with `gh pr create --base main`. The body carries `Closes #726`, a summary, test evidence, a manual check for Jason (the issue's four repro steps on a role prompt; drag past the bottom of a scrolled prompt; Up/Down across a wrapped paragraph; typing at the end of a long prompt keeps the caret in view; the mission composer growing and submitting on Enter; a Chinese IME candidate window sitting at the caret; a single-line field such as crew name or search), what is unverified, and no agent session links.
- **Watch CI** with `gh pr checks <n> --watch` until nothing is pending, on both macOS and Windows. Fold any fix into the commit with `git commit --amend`, have the reviewer check it, and push with `git push --force-with-lease`.

**Do not merge**, delete the branch or worktree, or cut a nightly or release.

The final handoff goes to everyone through Runner: the PR URL and CI result, changed files, checks with exit codes, what is untested or unverified, and the reviewer's verdict. Then both slots stand by.
