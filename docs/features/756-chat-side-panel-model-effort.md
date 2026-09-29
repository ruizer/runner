# 756 — Show model and effort in direct chat side panel meta info

> Tracking issue: [#756](https://github.com/yicheng47/runner/issues/756)
> Priority: P2. Platforms: macOS and Windows.

## Motivation

When viewing a direct chat (either role-backed or runtime-direct), the right-hand side panel (`render_chat_side_panel` in `crates/runner-app/src/surfaces/panes.rs`) displays a metadata card with `cmd`, `cwd`, and `session_key`.

However, it does not show the model or thinking effort that the chat is running with. For role-backed chats, model and effort may come from the role template or an override; for direct runtime chats, they are chosen in the Start Chat modal and stored on the session row (`agent_model`, `agent_effort`). Adding `model` and `effort` rows to the side panel's meta info provides visibility into the active execution settings without waiting for the full side panel redesign in #731.

## Scope

- **Backend**: Include `agent_model` and `agent_effort` on `DirectSessionEntry` (resolving from session row `agent_model` / `agent_effort` or role defaults).
- **Frontend**: Add `model` and `effort` rows to the direct chat right side panel meta section in `crates/runner-app/src/surfaces/panes.rs` alongside `cmd`, `cwd`, and `session_key`.
- Only render rows when the value is present (`Some`).
- Support both role-backed and runtime-only direct chats.

## Non-goals

- Redesigning the side panel's role card to match the role page (tracked separately in #731).
- Editing model or effort from the side panel.

## Implementation phases

1. **Backend**: Extend `DirectSessionRow` / `DirectSessionEntry` in `crates/runner-backend` to expose `agent_model` and `agent_effort`.
2. **Frontend**: Update `render_chat_side_panel` in `crates/runner-app/src/surfaces/panes.rs` to render `model` and `effort` rows in the metadata card.

## Verification

- `runner-app` and `runner-backend` tests pass; workspace clippy is clean.
- Manual pass on macOS and Windows:
  - Role chat with model and effort configured shows both in the side panel.
  - Direct runtime chat with model and effort configured shows both.
  - Chat with default/unspecified model and effort leaves the rows hidden.
