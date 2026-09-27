# 736 — Codex status recovers after conversation rollback

Fix [P1 #736](https://github.com/yicheng47/runner/issues/736). Jason authorized a `codex pair` crew mission on 2026-09-27. Work only in `/Users/jason/repos/yicheng47/runner/.worktrees/fix-736-codex-rollback-status` on branch `fix/736-codex-rollback-status`; the root checkout stays on `main`. This branch already exists for the mission: do not create another branch, checkout or worktree. Do not touch other worktrees or share their `target/` directories.

## Bug and evidence

After Jason rolled back an active Codex conversation, the live hook feed showed `UserPromptSubmit → Interrupt → SessionEnd → SessionStart → UserPromptSubmit` with the same Codex session ID. The rollout forked and Codex kept handling prompts and tools, but Runner 0.12.0 remained at `unavailable` / `interrupted` while the session lifecycle was `running` and byte activity was `busy`. In `crates/runner-backend/src/session/codex_status.rs`, `SessionEnd` sets `ended = true`; only a `SessionStart` with a *different* session ID clears it. A same-ID start is ignored, then all later hook events are dropped. This is a distinct path from #670.

## Deliverable

1. Read `AGENTS.md`, issue #736, `codex_status.rs`, and its surrounding hook-feed and session-manager paths. Confirm the same-ID restart sequence and identify the smallest safe state reset. Do not assume every delayed `SessionStart` should reset status: a start during an active turn must remain harmless.
2. Make a root `SessionStart` after a real `SessionEnd` able to re-arm observation even when Codex reuses the session ID. Clear stale turn guards, pending tool/compaction state, the ended flag, and transcript path as appropriate, while preserving protection from foreign sessions, child hooks, and delayed starts during live turns. Do not broaden scope to other runtimes without evidence.
3. Add regression tests for `Interrupt → SessionEnd → SessionStart` on the same ID followed by a new prompt/tool/stop, including recovery from the old interrupted outcome. Keep existing distinct-ID handover and stale-start behavior covered. Consider whether old turn hooks can arrive after the restart and guard them appropriately.
4. Run relevant `runner-backend` tests, workspace Clippy and formatting checks. Report exact commands and exit codes. Do not run or restart Jason's installed Runner app or access its real database; use test fixtures/temp state. Jason can smoke-test the UI after the PR.

Out of scope: changing the Codex hook-trust prompt, altering release 0.12.0, status UI redesign, and unrelated status adapters.

## Crew handoff and authorization

The coder implements and tests, then asks the reviewer through Runner to inspect the **working-tree diff before any PR**. The reviewer reports concrete must-fix findings with file:line pointers, or `NO REMAINING MUST-FIX ISSUES`. Iterate until clean; do not spawn extra agents or crews.

After a clean review, this mission explicitly authorizes the coder to squash the brief and implementation into **one focused commit** on this branch, push it, open a PR against `main` with `Closes #736`, and drive macOS and Windows CI green. CI fixes that materially change code go back through review, then are amended into the single commit and pushed with `--force-with-lease` if needed. Do not put any agent session link in the PR body. Do not merge, remove the branch/worktree, or cut a release. The final Runner handoff includes the PR URL, CI results, checks, and reviewer verdict.
