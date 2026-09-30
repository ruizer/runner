# 735 — Start a chat modal redesign

> Tracking issue: [#735](https://github.com/yicheng47/runner/issues/735)
> Priority: P2. Platforms: macOS and Windows.

## Motivation

The redesigned role and crew pages lead with identity and setup: pixel avatars, provider marks, and readable runtime, model, and effort values. Start a chat still presents a dense, generic form. Role mode names choices by `@handle` in a text dropdown, and the selected role's inherited setup is not visible before starting. The Agent field offers an override without showing the role's model, effort, or permissions alongside it. Direct mode uses the same form style and has no visual link to the runtime identity shown elsewhere. The modal should make the chosen chat setup clear at a glance and feel like part of the new Roles and Crews UI.

## Scope

- Design the modal in `design/specs/735-start-chat-modal.pen` using the visual language of the shipped role and crew pages, then implement the approved frames.
- In Role mode, show the chosen role's `RoleAvatar`, display name, and `@handle` in both the picker and selected state. Show its effective setup at a glance: the provider mark, runtime, model and effort. Distinguish inherited values from overrides. Keep the existing ability to switch the agent and set model and effort for that override.
- In Direct mode, show the selected runtime's provider mark and name, with the existing model and effort controls arranged in the same visual language.
- Keep Chat name and Working directory available in both modes, with the effective directory clear when the field is blank. Keep the Direct/Role choice, its remembered mode, role preselection from Chat now, project scope, defaults, and Start chat behavior.
- Preserve loading, empty, disabled, and error states, and keyboard navigation. The modal must scroll vertically and fit Runner's 640 × 480 minimum window without horizontal clipping; long role names and paths truncate or wrap appropriately.
- Presentation only: the existing launch commands and backend behavior stay the same.

## Design

Signed off 2026-09-30 in `design/specs/735-start-chat-modal.pen`. Each mode is one card: who the chat is with on top, how it runs underneath.

- **Role mode** (`MPgHD`). The role card's top row is the role picker: `RoleAvatar` at 40 px, display name, `@handle` and an up-down chevron; its menu lists roles the same way. Under it, read-only Runtime (provider mark and name), Model and Effort in three columns: the values the chat will run with. A bar at the bottom, "Run on another agent, model or effort", opens the overrides.
- **Overrides open, model only** (`GdboP`). The three values become the Runtime select, the Model combo and the Effort select in the same columns. A changed value carries an amber dot with `role: <value>` under it; an untouched value shows the role's value dimmed as `role (xhigh)`. The bar reads "Overrides apply to this chat only", with "Use role settings", which clears every override, and a chevron that folds the controls away.
- **Overrides open, agent changed** (`T2swh9`). Runtime shows the new agent with a dot and `role: Claude Code`; Model and Effort show that agent's own defaults, dimmed; Speed appears for Codex; a note says the role's model and effort belong to its own agent and don't carry over.
- **Direct mode** (`ouFT7`). The agent card's top row is the agent picker: the provider mark on a 40 px tile, display name and command. Under it, Model, Effort and Speed (Codex only) as controls in one row.
- **Both modes.** The Direct | Role switch stays under the header. Chat name carries an "optional" tag and the derived label as its placeholder. Working directory keeps Browse, with a hint naming where a blank field starts: "Blank starts in the role's directory." or "Blank starts in your default directory." Footer: Cancel and Start chat.

Decisions:

- **No permissions value.** Direct chats, whether started from a role or an agent, strip the role's permission flags and run with the agent's own default (feature 596, `strip_permission_flags` in `session/manager/spawn.rs`), so a role's permission mode never applies to a chat. Showing it would be wrong.
- **The request does not change.** Opening the overrides and changing nothing sends no override. Changing only the model or effort on the role's own agent sends the role's runtime as the override plus that value, which `resolve_runtime_override` already treats as keeping the role's other values; the dimmed `role (…)` placeholders say so, where today's fields show the agent's defaults. Choosing another agent sends it with its own defaults, as today. Speed keeps today's options, with inherited values in the same `role (…)` form.
- **Folding keeps the overrides.** Folded, the card shows the effective values with an amber dot on each overridden one, as the crew page's slot rows do, and the bar keeps "Overrides apply to this chat only".
- **States not drawn keep today's copy** in the new layout: no roles yet, detecting agents, no enabled agents with the Settings → Agents link, and the error banner above the form.

## Non-goals

- Changing how direct chats start or resolve role and runtime defaults.
- Editing roles, creating roles, or changing the Role and Crew pages.
- Changing the mission start flow.

## Implementation phases

1. **Design**: Pencil frames for Role and Direct modes, including selected and empty states, reviewed before code.
2. **Modal**: Update `crates/runner-app/src/surfaces/start_chat.rs` and only the shared UI pieces needed for the approved design.

## Verification

- `runner-app` tests pass, with focused coverage for any changed selection or override behavior; workspace Clippy is clean.
- Manual pass on macOS and Windows: start a preselected role chat, switch roles and modes, override the agent/model/effort, start a direct chat, and check empty runtime/role and launch-error states.
- At 640 × 480, the form remains usable by keyboard and mouse without horizontal scrolling.
