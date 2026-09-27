# 731 — Role side panel redesign

> Tracking issue: [#731](https://github.com/yicheng47/runner/issues/731)
> Priority: P2, milestone 0.12 (not a release blocker). Platforms: macOS and Windows.
> History: filed 2026-09-27 after the role page (#393, PR #720) and crew page (#699, PR #727) redesigns shipped.

## Motivation

The role page was redesigned in #393 (PR #720) and the crew page followed in #699 (PR #727): a pixel avatar, the setup (runtime, model, effort, permissions) up front, and the system prompt rendered as markdown and clamped. The chat side panel, the right-hand panel of a role chat (`render_chat_side_panel` in `crates/runner-app/src/surfaces/panes.rs`), still shows the role the old way:

- a **Role** card with the `@handle` in mono, a runtime text badge and the display name, but no avatar and no provider mark;
- no model, effort or permissions, although they decide how the chat runs;
- `cmd`, `cwd` and `session_key` rows, which stay useful;
- the whole system prompt dumped as plain text under **System prompt**, however long, with no markdown;
- no way to open the role's page.

The panel should read like a compact version of the new role page, so a role looks the same wherever it appears.

## Scope

- **Identity**: the role's `RoleAvatar` seeded with its handle, the display name and `@handle`, and an **Open role** link to the role page.
- **Setup**: Runtime (provider mark and name), Model and Effort, Permissions, laid out like the role page's setup rows.
- **Session**: `cmd`, `cwd` and `session_key` with its copy button stay.
- **System prompt**: rendered as markdown and clamped with Show all, with `N lines · X KB`, through the shared helpers in `surfaces/profile_page.rs` that the role and crew pages use.
- **Runtime chats** (no role) keep a matching layout: the runtime's mark and name as the identity, no avatar and no prompt.
- **Width**: the panel is resizable (`chat_panel_width`); the layout holds at its narrowest, with long values truncating.
- Presentation only: same data, same commands, no backend changes.

## Non-goals

- Editing the role from the panel; Open role goes to the page, which edits in place.
- Terminal panes, which keep the panel closed.
- The mission workspace's rail and slot cards.

## Implementation phases

1. **Design**: a Pencil frame for the panel, in the role page's layout language, reviewed before any code.
2. **Panel**: the identity, setup and markdown prompt, reusing `profile_page.rs`.

## Verification

- `runner-app` tests pass, extended where the panel's structure changes; workspace clippy is clean.
- Manual pass: a role chat whose prompt runs to several hundred lines (collapsed by default, Show all expands), a runtime chat with no role, and the panel dragged to its narrowest width.
- macOS and Windows.
