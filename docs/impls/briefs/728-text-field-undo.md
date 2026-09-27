# 728 — Undo and redo in text fields

Fix [P1 #728](https://github.com/yicheng47/runner/issues/728). Jason asked on 2026-09-27 for a claude pair crew mission that ends in an open PR, not a merge.

Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-728-text-field-undo`, on branch `fix/728-text-field-undo`. The mission's directory is this worktree. Its tip is this brief, on top of main `de1ed8d`. Do not create another branch or checkout, touch the root checkout or another worktree, or share another worktree's target directory. If main moves and you need it, rebase onto `origin/main`; never merge main into the branch.

## Read first

- `AGENTS.md`, including Worktrees and Crew Missions (a mission lands as one commit).
- **Issue #728**: the expected behaviour, which this brief refines.
- `crates/runner-app/src/ui/field.rs` at `de1ed8d`: `TextBuffer` `:53` (`reset` `:62`, `unmark_text` `:77`, `replace_text_in_range` `:104`, `replace_and_mark_text_in_range` `:119`, `replace_selection` `:172`, `delete_range` `:302`), `TextField::reset`/`set_text` `:1247`, `on_key_down` `:1321`, `on_cut`/`on_paste` `:1381`, the `EntityInputHandler` impl `:1709`, the `on_action` registrations `:1891`, `handle_key_down_for_platform` `:2358`, tests `:2542`. Plain typing arrives through the input handler's `replace_text_in_range`, not through `on_key_down`.
- **The design to follow**: `~/repos/gui/gpui-kit/crates/base/src/input/base/undo_manager.rs`, and its tests in `crates/kit/tests/input/history.rs` and `composition.rs`. It is built on another GPUI (`gpui-pre`), so read it for the approach and take no dependency. Leave out what our field does not have: multiple cursors, batches, auto-closed pairs, inline tokens, the per-transaction change cap. Zed agrees on the parts we take (`~/repos/gui/zed/crates/editor/src/editor.rs` `SelectionHistory`, `input.rs` `ime_transaction`); do not copy its time-based grouping, which even Zed turns off in tests.

## Deliverable

1. **History of edits, not snapshots.** Every change to `TextBuffer::text` goes through one helper that records it as a change (byte offset, old text, new text). A step holds its changes in order, the selection before the step (kept from when the step opened, through any merging) and the selection after it (updated with each merged change). Undo applies the inverse changes in reverse and restores the selection before; redo reapplies them and restores the selection after. A new recorded step clears the redo stack. Replaying undo or redo records nothing. Keep at most 1000 steps, dropping the oldest.
2. **When edits merge** (gpui-kit's `is_adjacent`). Each edit carries an intent: Typing, Backspace, DeleteForward or Atomic. An edit merges into the last step only when both have the same non-Atomic intent, nothing broke the run in between, and it continues where the last one stopped:
   - Typing: an insertion at the end of the previous one, with no newline in either.
   - Backspace: a deletion ending where the previous one began.
   - DeleteForward: a deletion starting at the same offset.

   Atomic, so always its own step: paste, cut, anything that replaces or deletes a non-empty selection, a newline, word and line deletes, `set_text`, and an IME commit. Anything that moves the caret or changes the selection without editing breaks the run: arrows, Home/End, Up/Down, clicks, drags, select all, double- and triple-click. So do undo and redo. A no-op edit records nothing.
3. **IME.** Composition edits (`replace_and_mark_text_in_range`) are never recorded one by one. When the composition ends, whether committed through `replace_text_in_range` while text is marked or kept as-is through `unmark_text`, record a single Atomic change from the text the composition replaced (`MarkedText::original`) to the text that remains, and restore the selection from before the composition on undo. A composition that ends with the original text, as a cancel does, records nothing. Undo and redo do nothing while composing. The next keystroke after a commit is a new step.
4. **Resets.** `reset` clears both stacks. `set_text` records one Atomic step when the text changes. The crew add-slot form lowercases its handle by calling `set_text` from an observer (`surfaces/crews/add_slot.rs:81`). With a naive `set_text`, typing `A` gives two steps, and undo goes back to `A`, which the observer lowercases again, recording a new step that clears redo. Make that case behave: typing `A` shows `a`, one undo leaves the field as it was before the keystroke, and redo gives `a`. Pick the smallest change that does it and say which in the PR.
5. **Keys and menu.** macOS: Cmd-Z undoes and Cmd-Shift-Z redoes. Windows: Ctrl-Z undoes, and both Ctrl-Y and Ctrl-Shift-Z redo. Extend `handle_key_down_for_platform`, whose Windows command gate lists only a/c/x/v today. Add `Undo` and `Redo` to `actions!(runner_app_ui, ...)` in `lib.rs`, handle them on `TextField` next to Cut, Copy and Paste, and put Undo and Redo at the top of the macOS Edit menu (`main.rs:1451`) with `OsAction::Undo`/`OsAction::Redo` and a separator, as Zed does. Disabled fields ignore them. An undo or redo reveals the caret, clears `vertical_goal`, marks the field edited and notifies, so each page's dirty checks and Save state follow it.
6. **Tests** in `field.rs`'s test module, mostly at the `TextBuffer` level and a few through `handle_key_down_for_platform` with `windows` both true and false:
   - typing merges;
   - a newline splits a run, and typing after it is a new step;
   - paste, cut and delete-selection are one step each;
   - Backspace and forward-delete runs merge;
   - a caret move splits typing;
   - redo is cleared by a new edit;
   - the selection is restored on undo and on redo;
   - a pinyin-style composition (mark `n`, `ni`, `ni h`, `ni hao`, then commit `你好`) is one step with the right UTF-8 offsets, a cancelled one leaves nothing, and undo while composing is a no-op;
   - `reset` clears the history;
   - `set_text` is one step;
   - the 1000-step cap holds;
   - the platform key mapping works;
   - the add-slot handle case, in the crews tests.

   Imports, statics and helpers used only by `#[cfg(unix)]` tests get `#[cfg(unix)]` themselves; macOS clippy cannot catch this and Windows CI will.

Out of scope: undo across fields or pages, a clean point that clears `edited` when you undo back to the saved text, per-word or time-based merging, and any change to how callers build fields. `TextField`'s public API grows only by what the actions need.

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

- that no edit path bypasses the recorder (typing, Enter, paste, cut, Cmd-X, every delete, `set_text`, IME);
- that merge boundaries match the list above;
- that undo and redo restore both the text and the selection exactly, including after a merged run;
- that an IME composition never leaves partial steps;
- that key interceptors, Enter behaviour, the terminal's own keys and disabled fields are unchanged;
- that the add-slot case does not loop.

Iterate through Runner until the reviewer posts `NO REMAINING MUST-FIX ISSUES`. No extra agents, crews or subagents.

After the clean review, and only then, Jason authorizes:

- **One commit.** Squash everything on the branch, this brief included, into a single commit on top of `main`: imperative subject naming the change (for example `fix(ui): undo and redo in text fields`), no co-author trailers.
- **Push** with `git push -u origin fix/728-text-field-undo`.
- **Open the PR** with `gh pr create --base main`. The body carries `Closes #728`, a summary that credits gpui-kit's undo manager as the model, test evidence, a manual check for Jason, what is unverified, and no agent session links. The manual check covers:
  - typing a sentence then a newline in a role prompt, then undoing twice and redoing;
  - paste then undo;
  - a Chinese IME phrase then undo;
  - Edit → Undo from the macOS menu;
  - typing `A` in a new slot's handle then undoing;
  - Save and Cancel on the role page after an undo.
- **Watch CI** with `gh pr checks <n> --watch` until nothing is pending, on both macOS and Windows. Fold any fix into the commit with `git commit --amend`, have the reviewer check it, and push with `git push --force-with-lease`.

**Do not merge**, delete the branch or worktree, or cut a nightly or release.

The final handoff goes to everyone through Runner: the PR URL and CI result, changed files, checks with exit codes, what is untested or unverified, and the reviewer's verdict. Then both slots stand by.
