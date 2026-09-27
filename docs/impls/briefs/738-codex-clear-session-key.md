# 738 — Rekey a Codex chat after `/clear`

Fix [P1 #738](https://github.com/yicheng47/runner/issues/738). Jason asked for a codex pair mission ending in an open PR, not a merge.

Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-738-codex-clear-session-key`, on the existing branch `fix/738-codex-clear-session-key`. This brief is the branch's first commit on `origin/main` `02ce436`. Do not create another branch or checkout, touch the root checkout or another worktree, or share another worktree's target directory. Rebase onto `origin/main` if it moves; never merge main into the branch.

## The bug and boundary

After Codex `/clear`, the same Runner pane can have a new Codex conversation ID while its saved `agent_session_key` still names the old conversation. Resume or fork can then reopen the old history. The issue has the reproduction and the source inspection. Claude Code already handles this through `session/claude_rekey.rs`. Codex's root `SessionStart` reaches `session/codex_status.rs` for status, but `session/codex_capture.rs` only captures a key while the database column is null. `repo/session.rs::rekey_agent_session_key` already guards an overwrite by the running row's `started_at`.

This is distinct from [#736](https://github.com/yicheng47/runner/issues/736), whose [PR #737](https://github.com/yicheng47/runner/pull/737) is open and edits `codex_status.rs` to recover status after a rollback with the same Codex session ID. Keep that behavior. If #737 lands during this mission, rebase and resolve against it before final review and CI. Do not include #736's commit as this PR's own change.

## Deliverable

1. Trace the Codex root `SessionStart` report through its generation-scoped hook feed, the owning Runner session, database row and update event. Use that path to update the saved key when the root conversation ID changes. Do not treat child-agent hooks, delayed hooks from a prior spawn or a stale initial rollout capture as the current conversation. Reuse `repo::session::rekey_agent_session_key` and the existing update event where they fit; keep the implementation small.
2. Preserve initial key capture for a new Codex session. A same-ID `SessionStart`, including #736's rollback path, must not churn the saved key. Resume and fork after `/clear` must read the new key through the normal session row; avoid a second source of truth.
3. Add regression tests proving: a keyed running Codex row rekeys after a new root `SessionStart`; stale reports from an older row incarnation, child hooks and stopped rows cannot rekey it; initial capture cannot overwrite the new key; same-ID starts do not change it; and the session update event fires on a real rekey. Cover both direct chats and mission slots where the shared path applies. Check the existing #736 rollback-status tests after a rebase.

Out of scope: changing Codex status semantics beyond the hook plumbing needed here, changing Claude Code's rekey path, a new database column, and UI changes. Do not launch live agents or use Jason's Runner database for tests.

## Review and validation

The coder implements, then asks the reviewer through Runner to inspect the working-tree diff before any code commit or PR. The reviewer returns must-fix findings with file:line pointers; iterate until it reports `NO REMAINING MUST-FIX ISSUES`. No extra agents, crews or subagents.

Run and report exit codes for `cargo test --locked -p runner-backend --profile ci --no-fail-fast`, `cargo clippy --locked --workspace --all-targets --profile ci -- -D warnings`, `cargo fmt --all --check`, and `git diff --check`. CI must be green on macOS and Windows. The crew does not run the dev app or touch Jason's live chats; Jason can smoke-test `/clear`, Stop → Resume and Fork after the PR is ready.

## Authorization and handoff

After clean review, squash the brief and implementation into **one commit** on top of current `origin/main`, with an imperative subject and no co-author or session-link trailers. Push `fix/738-codex-clear-session-key`, open a PR against `main` with `Closes #738`, the root cause, changed paths, test evidence and Jason's smoke steps. Do not put a Claude session URL in the PR body. Watch `gh pr checks <n> --watch`; resolve any failures on the branch, route non-trivial fixes back through review, and keep the PR at one commit with `--force-with-lease` if amending. Do not merge, delete the branch or worktree, cut a nightly or release.

The final Runner handoff includes the PR URL, macOS and Windows CI result, review verdict, changed files, tests with exit codes, and anything Jason still needs to verify.
