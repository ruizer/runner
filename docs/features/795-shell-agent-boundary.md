# 795 — Separate shell lifecycle from agent orchestration

> Tracking issue: [#795](https://github.com/yicheng47/runner/issues/795)
> Priority: P1, 0.16 with remote machines (moved from 0.13 on 2026-10-07). Platforms: macOS and Windows.
> Status: problem and direction recorded on 2026-10-03. On 2026-10-04 it moved to 0.13 as part of the [#645](https://github.com/yicheng47/runner/issues/645) session host design; [the issue comment](https://github.com/yicheng47/runner/issues/795#issuecomment-5980139947) rechecks the evidence below against the code after #791 and lists the launch steps a remote host must run itself. It closes when the #645 spec covers them.

## Motivation

Shell terminals and agent chats share `SessionManager` and the PTY implementation, which is appropriate for process lifecycle, input/output, resize, and exit handling. However, the shared session layer also owns agent launch and conversation orchestration. A plain shell traverses that workflow through no-op adapter methods and shell-specific exemptions.

The #777 runtime adapter refactor localizes differences between agent CLIs but deliberately preserves this boundary. This issue tracks the remaining separation between shell/process management and agent orchestration. Code inspection confirms structural coupling; no new user-visible shell regression is claimed.

## Evidence in the current code

- `crates/runner-backend/src/ops/session.rs`: `session_start_shell_in` represents the shell as a synthetic `Role` through `runtime_direct_role("shell", ...)`, including agent-oriented fields such as system prompt, model, effort, and Codex speed.
- `crates/runner-backend/src/session/manager/spawn.rs`: `spawn_direct_inner`, shared by shells and agents, runs runtime/model override resolution, permission stripping, conversation resume planning, prompt-channel handling, key-capture preparation, launch gates, and project trust. Most of those operations are no-ops for shells via `NoAgent`.
- The same file's `resume_with_fresh_fallback` processes prior conversation identity and missing-history policy. Its rejection of a launch that cannot resume a conversation needs an explicit `!shell` exemption. Restoring a terminal instead launches a new shell; it has no agent conversation to restore.
- `crates/runner-backend/src/session/manager/mod.rs`: every session uses `SessionState`, which combines a process handle and resize state with `AgentStatus`, hook authority flags, completion tracking, compaction failure state, and a delivery gate.
- `crates/runner-backend/src/session/runtime.rs` and `session/pty_runtime.rs`: the process-layer launch specification and idle detector also contain Codex-specific startup state (`codex_pending_turn`, title/startup classifiers).

## Why it matters

Reasoning about a plain terminal requires navigating agent-only concepts. Agent launch or status changes can affect shared shell paths, and the separation depends on scattered guards and default no-op behavior rather than an explicit boundary. `NoAgent` makes the combined workflow work but does not establish that boundary.

## Scope

- Keep a common process lifecycle and PTY implementation for shells and agents.
- Separate shell launch/relaunch preparation from agent prompt, permission, model, trust, and conversation preparation; both should produce the inputs needed by the common process layer.
- Make agent-specific state and coordination explicit so ordinary shell lifecycle operations do not depend on agent conversation or turn semantics.
- Distinguish a valid shell runtime from an unrecognized runtime key. Check existing compatibility requirements before changing unknown-key behavior.
- Coordinate with #791, which centralizes status, draft, and conversation decisions, without assuming that a reducer alone separates shell lifecycle from agent orchestration.

This is a problem statement and bounded direction, not a settled implementation design. Separate managers or duplicate PTY implementations are not required.

## Implementation phases

Proposed sequencing only; implementation design is not settled.

1. Define the shared process lifecycle boundary and its shell/agent launch inputs, coordinating with #791.
2. Separate shell launch and relaunch preparation from agent policy while preserving current behavior.
3. Separate agent state and status interpretation from common process handling, and verify both paths.

## Verification

- Existing shell launch and relaunch behavior remains intact: login-shell arguments, working-directory fallback, shell integration, resize, input/output, foreground-process handling, and close behavior.
- Agent chat and mission launch/resume behavior remains intact, including prompts, permissions, models, keys, hooks, and completion handling.
- Shell launch preparation does not invoke agent conversation or prompt policy merely to receive no-op answers.
- Relevant shell coverage starts in `crates/runner-backend/src/session/manager/tests/runtime_direct.rs`, including `shell_runtime_spawns_and_resumes_as_plain_login_shell`.

## Context

- Milestone: 0.12, requested by Jason during the runtime code walkthrough on 2026-10-03.
- Related: #777 (landed adapter refactor), #791 (session-state refactor).
- Platforms: macOS and Windows.
