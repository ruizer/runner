# 791 — Session state: one model for agent status, drafts and conversation identity

> Tracking issue: [#791](https://github.com/yicheng47/runner/issues/791)
> Priority: P1, 0.12 (moved from 0.13 on 2026-10-03; its phase 3 bugs are in 0.13). Platforms: macOS and Windows. Unblocked: [#777](https://github.com/yicheng47/runner/issues/777) landed on 2026-10-03.
> Status: decisions settled with Jason on 2026-10-03; PR 1 (phases 0 and 1) is in progress.

## Motivation

Session state is the least reliable part of Runner. Here it means three things about a running agent: what it is doing (Working, Ready, Needs you, Interrupted, Failed), whether the person has a draft in its composer, and which of the agent's conversations the session is attached to. Of the 67 bugs filed since the 0.6.0 cutover on 2026-08-23, 18 are in this area, and so are 5 of the 8 open bugs. Among backend files, `session/pty_runtime.rs` and `session/manager/spawn.rs` have had the most bug-fix commits, 13 each. The main ones:

| Bug | What went wrong |
| --- | --- |
| [#459](https://github.com/yicheng47/runner/issues/459), [#738](https://github.com/yicheng47/runner/issues/738) | `/clear` left the old conversation key, so resume restored the pre-clear history (Claude, then Codex) |
| [#583](https://github.com/yicheng47/runner/issues/583) | An animated composer pinned the session Busy, because the idle detector counts bytes |
| [#623](https://github.com/yicheng47/runner/issues/623), [#659](https://github.com/yicheng47/runner/issues/659), [#670](https://github.com/yicheng47/runner/issues/670), [#736](https://github.com/yicheng47/runner/issues/736) | Status unavailable: on the first turn, after a Claude `/clear`, after a relaunch, after a Codex rollback |
| [#687](https://github.com/yicheng47/runner/issues/687) | Codex stayed Working at an untouched prompt |
| [#753](https://github.com/yicheng47/runner/issues/753), [#766](https://github.com/yicheng47/runner/issues/766) | Crew deliveries piled up unsubmitted, or were reported held at an empty Codex composer (#766 closed by #794 without a reproduction) |
| [#783](https://github.com/yicheng47/runner/issues/783) | A cancel left Claude Code, Antigravity and Copilot Working |
| [#781](https://github.com/yicheng47/runner/issues/781) (open) | Codex key capture ignores `CODEX_HOME`, so the chat is never keyed |
| [#784](https://github.com/yicheng47/runner/issues/784) (open) | A pi cancel is reported as Response failed |
| [#785](https://github.com/yicheng47/runner/issues/785) (open) | Claude resume starts fresh when the cwd is a symlink alias |
| [#786](https://github.com/yicheng47/runner/issues/786) (open) | Copilot `/clear` keeps the old key |

Each of these was fixed, or will be, by a local patch at the point where it showed. #783 added a third way of recognizing a cancel. The bugs keep coming because no one place decides a session's state: several writers on several threads each apply part of the rules. #777 puts each runtime's code in one module but deliberately changes no logic, so it does not touch this. After #777, the runtime-specific parts sit in one place each, which makes this cleanup tractable.

## How it works today

### Status writers

| Writer | Where | What it does |
| --- | --- | --- |
| Spawn seed | `session/manager/spawn.rs`, source `spawn` | Seeds mission slots and direct chats Busy, so a session reads Working · estimated until other evidence arrives |
| Byte idle detector | `session/pty_runtime.rs` `IdleDetector::on_output`, `tick`; source `forwarder` | Busy on output, Idle after 2 s of silence, with a 500 ms resize grace |
| Codex title hint and startup | `pty_runtime.rs` `CodexTitleHint`, `CodexStartup` | Reads Codex's OSC title spinner as a Busy or Idle hint; owns readiness at an untouched prompt (#687) |
| Local submit and typing | `session/manager/output.rs` `classify_local_input`, `update_local_input_state`; source `input-submit` | Enter on an idle session marks Busy. Typing into an idle session sets `suppress_local_input_busy`, so the echo does not read as Working |
| Local cancel | `pty_runtime.rs` `interrupt_key` sets the `hook_interrupt` `AtomicU8`, which the Claude Code, Copilot and Antigravity watchers drain; sources `input-interrupt` and `input-escape` | Interrupted, with a provisional Idle after Esc |
| Agent watchers | `runtimes/*/*_status.rs` through `HookWatcher::drain_observations` | Each produces a complete `AgentObservation` snapshot from the hook feed plus the agent's own records: Claude's transcript, Codex's rollout, Copilot's `events.jsonl`. pi and Antigravity read only the feed |
| Bridge failure | `manager/mod.rs` `status_bridge_failed` | Falls back to the last byte-detector activity |
| Router wake | `router/mod.rs` calls `SessionManager::synthesize_wake_busy` | Marks Busy after a delivery and appends it to the mission log |
| Exit | `manager/mod.rs` `record_exit_status` | Lifecycle Stopped or Error |

### Where precedence lives

- `IdleDetector` decides which byte and title transitions are emitted (`hook_owned`, `codex_startup`, `codex_title`, `input_transition`), and `accept_hook` filters the watchers' observations.
- `SessionManager::note_forwarder_transition` (`manager/mod.rs`) matches on `source` strings and reads or writes `hook_status_armed`, `suppress_local_input_busy`, `provisional_idle`, `completion_armed`, `baseline_activity`, `activity`, `status` and `activity_revision`.
- `publish_observation` (`manager/mod.rs`) applies a watcher's snapshot. It derives busy/idle, `failed_since` and the compaction carry-over of a failure, arms completion and releases the delivery gate.
- The router keeps its own busy/idle projection from the `session_status` rows in the mission log.

`SessionState` holds twelve fields that together encode this: `activity`, `status`, `baseline_activity`, `activity_revision`, `suppress_local_input_busy`, `hook_status_armed`, `provisional_idle`, `local_input_pending`, `observed_input`, `last_local_input_at`, `completion_armed` and `compaction_failed_since`. They are written from the PTY reader thread, the idle monitor thread, the manager's forwarder thread, the input path and the router.

There are two status vocabularies. `SessionActivityState` (busy or idle) is what the mission log, the router's projection and `session/status` events carry. `AgentStatus` (lifecycle; activity Working, Idle, Ready or Unavailable; outcome; interactions; detail) is what the UI shows. `note_forwarder_transition` writes both, while `publish_observation` derives one from the other. Sources are free strings: `spawn`, `forwarder`, `input-submit`, `input-interrupt`, `input-escape`, `hook`, `baseline` and `unavailable`.

Because a watcher's output is a full snapshot, each watcher re-implements turn state, interaction bookkeeping and outcomes. That is why `claude_status.rs` is 2,069 lines and `copilot_status.rs` 1,722, and why #783 needed both a Copilot approval-correlation fix and a Claude rejected-tool settle. The cancel constants are defined in `claude_status.rs` and imported by the Copilot watcher, `pty_runtime.rs` and `manager/output.rs`.

### Draft state

Two signals decide whether the person has a draft. The first is a byte latch, `local_input_pending`: printable input and pastes set it, and Enter and Ctrl+C clear it. The second is `observed_input`, from the composer detector in `runner-terminal/src/input_state.rs`, which compares composer rows and cell styles before and after typing. `input_quiescent` and `reserve_delivery` combine the two with `last_local_input_at` and a recent-input window. The detector shipped without recorded evidence. #766 reported it reading an idle Codex composer as a draft; QA's live baseline for [#794](https://github.com/yicheng47/runner/pull/794) did not reproduce that, so #794 added hold and release logging and four recorded composer fixtures, and left the detector unchanged.

### Conversation identity

| Runtime | Key at spawn | Key changes during a run | Resume probe |
| --- | --- | --- | --- |
| Claude Code | Assigned (`--session-id`) | Rekey drop file written by its hook (`session/claude_rekey.rs`) | `claude_code_conversation_exists(cwd, key)`, with the cwd as given, so a symlink alias misses (#785) |
| Codex | None until the first prompt | Rollout scan under a hard-coded `~/.codex/sessions` (`session/codex_capture.rs`, #781), plus the `SessionStart` hook (`CodexSessionStart` in `manager/output.rs`) | Rollout lookup |
| GitHub Copilot CLI | Assigned (`--session-id`) | None (`KeyCapture::None`), so `/clear` keeps the old key (#786) | `copilot_conversation_exists_with_home` |
| pi | Assigned (`--session-id`) | Rekey drop from its extension | Session file under `PI_CODING_AGENT_DIR` |
| Antigravity | None | Log tail (`runtimes/antigravity/agy_capture.rs`) | `--conversation <key>` |
| TRAE | None | Rollout scan under `~/.trae/cli/sessions` | Subcommand resume |

Keys reach the database three ways. Spawn, resume and fork write the assigned key directly. `rekey_agent_session_key`, guarded by the row's start time, serves the rekey drops, the Codex hook and the Antigravity log tail. `capture_agent_session_key`, guarded by `agent_session_key IS NULL`, serves the rollout scans.

## Proposal

### One reducer per session

A new `session/state/` module holds a `SessionModel` and one pure function, `apply(event, now) -> Effects`. It does no IO and takes no locks. It lives inside `SessionState` under the existing per-session mutex. Every writer in the table above becomes a producer of a `SessionEvent`, and nothing else mutates status, draft or key fields. The effects are what the manager does afterwards: emit `session/status`, append a `session_status` row, notify the delivery gate, persist a key, and arm or clear completion and unread.

```rust
// crates/runner-backend/src/session/state/: shape, not final signatures
pub enum SessionEvent {
    Spawned { resuming: bool },
    Output { quiet: bool },              // quiet = inside the resize grace
    Silence,                             // the idle monitor passed the threshold
    Title(TitleHint),                    // Codex title classifier result
    Input(LocalInput),                   // Submit, Interrupt, Escape, Draft, Paste, Other
    Composer(InputState),                // composer detector observation
    Agent(AgentEvent),                   // from the runtime adapter
    BridgeFailed,
    Delivered,                           // the router's wake
    Exited { code: Option<i32>, killed: bool },
}

pub enum AgentEvent {
    TurnStarted,
    Working { detail: Option<WorkDetail> },
    TurnEnded { outcome: TurnOutcome },
    InteractionOpened { id: String, reason: WaitReason, owner: String },
    InteractionClosed { id: String },
    Ready,                               // idle with no turn: startup, after /clear
    ConversationChanged { key: String, cause: KeyCause },
}

pub struct SessionModel {
    lifecycle: Lifecycle,
    authority: Authority,                // Baseline, Hook, or BridgeFailed
    activity: Activity,
    outcome: Option<TurnOutcome>,
    interactions: Vec<HumanInteraction>,
    detail: Option<WorkDetail>,
    failed_since: Option<i64>,
    draft: DraftState,
    key: KeyState,
    completion: Completion,
    revision: u64,
}
```

### The rules, written down once

Phase 1 reproduces today's behavior exactly. These rules are read out of the current code, and each row becomes a reducer test:

1. **Baseline authority** holds before any accepted agent event. Output means Working unless it is quiet; Silence means Idle. A confirmed Codex title hint overrides bytes. Codex startup readiness owns the untouched prompt.
2. **The first accepted agent event** sets Hook authority. From then on Output, Silence and Title no longer change activity, and quiet output never ends a healthy hook-driven turn.
3. **BridgeFailed** returns to Baseline with the last baseline activity. A title hint needs fresh evidence after the failure.
4. **Interrupt or Escape under Hook authority** while Working gives outcome Interrupted with activity Unavailable, provisionally, and disarms completion. A later agent event settles it.
5. **Submit on an idle session** means Working. **Typing on an idle session** suppresses baseline Working until the next Idle or a submit.
6. **An open interaction** means Needs you, which counts as busy for the router. Closing the last one releases the delivery gate.
7. **`failed_since`** is set on the first Failed, carried across a compaction, and cleared otherwise.
8. **Completion** is armed on Working with no outcome (compaction excluded), disarmed on Interrupted or Failed, and consumed by unread.
9. **Delivered** marks busy if nothing changed the model since the delivery was reserved.
10. **Exited** ends the session, and later events are ignored.

### Adapters translate, the reducer decides

`HookWatcher::drain_observations` becomes `drain_events`, which yields `AgentEvent`s. A watcher keeps the correlation that is specific to its CLI, such as matching a Copilot `permission.completed` to its request or recognizing Claude's user-rejected `tool_result`. It stops building snapshots and deciding outcomes. A watcher that needs to know a local cancel happened, to read a later record correctly, receives it as an argument to `drain_events` instead of through a shared `AtomicU8` whose constants live in another runtime's module.

### One vocabulary

`AgentStatus` is the model's published form. Busy/idle is derived from it (busy means Working or Needs you) for `session_status` rows and the router, so the rows on disk keep their shape. `source` becomes a `StatusSource` enum that serializes to today's strings.

### Draft state

`DraftState` (Idle, Drafting, Submitted) lives in the model, fed by `Input` and `Composer` events with one rule table. Phase 1 keeps today's combined rule. The draft rows are pinned by the composer fixtures already in `crates/runner-terminal/fixtures/`, including #794's `input-codex-*` and `input-claude-control-submit` recordings. A draft bug that reproduces later is recorded with the existing `RUNNER_RECORD_INPUT_FIXTURE` before it is fixed, not reasoned about.

### Conversation identity

`KeyState` in the model holds the current key, its origin (assigned, captured or rekeyed) and the spawn generation. Every capture source emits `ConversationChanged`. The reducer drops reports from an earlier spawn and returns a `PersistKey` effect, and one repository function writes it with one guard (the row plus its spawn generation). That replaces both `capture_agent_session_key` and `rekey_agent_session_key`. The adapter supplies capture locations from the runtime's own home settings (`CODEX_HOME`, Copilot's config home, `PI_CODING_AGENT_DIR`) instead of `home_dir()`. Resume probes canonicalize the cwd.

### A scenario corpus

A scenario is one NDJSON file of timestamped inputs to a session: PTY output and titles, local input, hook feed lines, and the agent records a watcher reads. Scenarios are scripted from the rule rows and the bug evidence, with agent records copied from samples that already exist: the watchers' test fixtures, the hook-feed files Runner writes per session, and the CLIs' own transcripts and rollouts, with throwaway prompts and no account tokens. They live under `crates/runner-backend/src/session/fixtures/scenarios/<runtime>/`. A replay harness feeds a scenario through the adapter and the state code on a fake clock and asserts the published status and key timeline. No new recorder is built and no live session is recorded for the corpus; a real CLI's behavior is checked by the smoke test on each PR.

Scenarios, for each runtime where the CLI supports them: a fresh first turn; tool use; an approval approved and one denied; a question answered; Esc mid-reply and mid-tool; Ctrl+C; approve then cancel; an API error; compaction; `/clear` or `/new` then resume; resume with missing history; a resume after a Runner relaunch; a crew message to an idle slot; a typed draft followed by a crew message; and a synthetic bridge failure. Each bug in the Motivation table maps to at least one scenario.

## Phases

0. **Corpus and goldens of current behavior.** The replay harness, the scripted scenarios, and goldens produced by running today's code over them, with no live step. Known-wrong timelines are marked with their bug number. The harness drives existing seams (`IdleDetector`'s `_at` methods, `drain_observations` over temp files, `note_forwarder_transition`, `publish_observation`). Where a timer reads the wall clock, phase 0 injects a clock first, a small change with no behavior effect.
1. **The reducer.** Every writer is routed through it, the twelve flags leave `SessionState`, `StatusSource` replaces the strings, and the cancel constants leave `claude_status.rs`. The goldens are identical.
2. **Adapters translate.** Watchers emit `AgentEvent`s and shrink to parsing and correlation. The goldens are identical.
3. **Fixes,** one bug per commit, each flipping its known-wrong golden: #784, #785, #786 and #781.

Phases 0 and 1 land as one PR, then phase 2, then phase 3, one mission each. Each PR leaves behavior identical except phase 3.

## Rules

- No UI change: the same states, copy and pills.
- `session_status` rows and `session/status` payloads keep their shape.
- The goldens do not change during phases 1 and 2.
- The Windows reporters (Git Bash `sh` and PowerShell) are unchanged. The replay is platform-independent, and the Windows-only paths stay covered by their existing tests.

## Non-goals

- [#787](https://github.com/yicheng47/runner/issues/787) (Antigravity tools run in the hooks folder), which is a spawn cwd issue.
- Shell status ([#586](https://github.com/yicheng47/runner/issues/586)) and hooks for TRAE.
- New states, new copy or a status redesign.
- The delivery gate's timing: cooldown, reconciliation and the outbox.
- Mission notices into chats ([#748](https://github.com/yicheng47/runner/issues/748)).

## Decisions

Proposed 2026-10-02, settled with Jason on 2026-10-03.

1. **Adapters emit events and the reducer owns the snapshot.** The alternative keeps snapshots and only centralizes precedence. It is smaller, but it leaves every watcher re-implementing turn state, which is where most of the #783-style bugs came from.
2. **The open bugs are fixed inside this program, in phase 3,** not as separate patches first, unless one becomes P0. Patching them now adds to the code that phase 1 has to reproduce.
3. **No recording step; QA smoke-tests.** The corpus is scripted (see A scenario corpus), so phases 0 to 2 need no live sessions and run on the `codex duo` crew, coder and reviewer with no QA slot, with Jason's smoke test on each PR. Phase 3 changes real behavior, so its mission adds a QA slot for smoke and regression tests on the five runtimes, under live-test authorization in bounded test chats.
4. **Three PRs,** as in Phases.

## Verification

- The corpus replays in CI on both platforms.
- The reducer has one test per rule row.
- The goldens are unchanged across phases 1 and 2.
- Jason's smoke test on each PR, on Codex, Claude Code, Antigravity, Copilot and pi: a turn with a tool, an approval, Esc and Ctrl+C mid-reply, `/clear` then resume, a crew message to an idle slot, and a typed draft held against a delivery.
- The [full smoke test](../tests/full-smoke-test.md) before the release that carries phase 3.

## Relevant code

- `crates/runner-backend/src/session/status.rs`: `AgentStatus`, `AgentObservation`, `Activity`, `Lifecycle`.
- `crates/runner-backend/src/session/runtime.rs`: `SessionActivityState`, `RuntimeOutput`.
- `crates/runner-backend/src/session/pty_runtime.rs`: `IdleDetector`, `CodexTitleHint`, `CodexStartup`, `interrupt_key`, the reader and idle monitor threads.
- `crates/runner-backend/src/session/manager/mod.rs`: `SessionState`, `note_forwarder_transition`, `publish_observation`, `status_bridge_failed`, `input_quiescent`, `reserve_delivery`, `report_input_state`, `synthesize_wake_busy`.
- `crates/runner-backend/src/session/manager/output.rs`: the forwarder thread, `classify_local_input`, the `input-submit` path, `CodexSessionStart` handling.
- `crates/runner-backend/src/session/hook_feed.rs`: `HookWatcher`.
- `crates/runner-backend/src/runtimes/*/*_status.rs`: the five watchers; `runtimes/mod.rs`: `KeyCapture`, `StatusHooks`.
- `crates/runner-backend/src/session/claude_rekey.rs`, `session/codex_capture.rs`, `runtimes/antigravity/agy_capture.rs`: key capture.
- `crates/runner-backend/src/repo/session.rs`: `rekey_agent_session_key`, `capture_agent_session_key`.
- `crates/runner-backend/src/router/mod.rs`, `router/handlers.rs`: the busy/idle projection and the wake.
- `crates/runner-terminal/src/input_state.rs`: the composer detector.
- [`docs/arch/arch.md` §5.10](../arch/arch.md#510-busy--idle-inference) and §8.5, and [`docs/arch/runtime-integration.md`](../arch/runtime-integration.md) P0.7 and P0.8, which phase 1 updates to describe the reducer.
