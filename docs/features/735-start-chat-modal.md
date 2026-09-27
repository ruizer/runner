# 735 — Start a chat modal redesign

> Tracking issue: [#735](https://github.com/yicheng47/runner/issues/735)
> Priority: P2. Platforms: macOS and Windows.

## Motivation

The redesigned role and crew pages lead with identity and setup: pixel avatars, provider marks, and readable runtime, model, and effort values. Start a chat still presents a dense, generic form. Role mode names choices by `@handle` in a text dropdown, and the selected role's inherited setup is not visible before starting. The Agent field offers an override without showing the role's model, effort, or permissions alongside it. Direct mode uses the same form style and has no visual link to the runtime identity shown elsewhere. The modal should make the chosen chat setup clear at a glance and feel like part of the new Roles and Crews UI.

## Scope

- Design the modal in `design/specs/735-start-chat-modal.pen` using the visual language of the shipped role and crew pages, then implement the approved frames.
- In Role mode, show the chosen role's `RoleAvatar`, display name, and `@handle` in both the picker and selected state. Show its effective setup at a glance, including the provider mark, runtime, model, effort, and permissions. Distinguish inherited values from overrides. Keep the existing ability to switch the agent and set model and effort for that override.
- In Direct mode, show the selected runtime's provider mark and name, with the existing model and effort controls arranged in the same visual language.
- Keep Chat name and Working directory available in both modes, with the effective directory clear when the field is blank. Keep the Direct/Role choice, its remembered mode, role preselection from Chat now, project scope, defaults, and Start chat behavior.
- Preserve loading, empty, disabled, and error states, and keyboard navigation. The modal must scroll vertically and fit Runner's 640 × 480 minimum window without horizontal clipping; long role names and paths truncate or wrap appropriately.
- Presentation only: the existing launch commands and backend behavior stay the same.

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
