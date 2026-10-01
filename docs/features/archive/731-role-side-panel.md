# 731 — Role side panel redesign

> Status: merged to `main` in [PR #770](https://github.com/yicheng47/runner/pull/770) on 2026-10-01, unreleased; archived 2026-10-01. The original scope and dated design decisions follow.

> Tracking issue: [#731](https://github.com/yicheng47/runner/issues/731)
> Priority: P2, milestone 0.12. Platforms: macOS and Windows.
> History: filed 2026-09-27 after the role page (#393, PR #720) and crew page (#699, PR #727) redesigns shipped. #756 (PR #765, 2026-09-30) took the identity and setup rows; this spec keeps the rest. Design signed off 2026-09-30.

## Motivation

The role page was redesigned in #393 (PR #720) and the crew page followed in #699 (PR #727): a pixel avatar, the setup up front, and the system prompt rendered as markdown and clamped. The chat side panel, the right-hand panel of a role chat (`render_chat_side_panel` and `chat_panel_content` in `crates/runner-app/src/surfaces/panes.rs`), now leads with the role's avatar, its runtime, model and effort (#756), but still:

- dumps the whole system prompt as plain text under **System prompt**, however long, with no markdown;
- has no way to open the role's page.

The panel should read like a compact version of the new role page, so a role looks the same wherever it appears.

## Design

`design/specs/archive/731-role-side-panel.pen`, frame `l20LyZ`: a role chat with the panel open, built on `cmp/ChatSidePanel`. The shipped design is `cmp/ChatSidePanel` (`WlZj2`) in `design/runner.pen`.

## Scope

- **Open role**: a text link, "Open role" with an `arrow-up-right` glyph, at the right end of the **ROLE** section label row, in the same place and style as the crew page's "+ Add slot" in its Slots header. It opens the role's page (`open_role_detail`). Runtime chats have no role and show no link; their section label stays **RUNTIME**.
- **System prompt**: the section label row gains the prompt's `N lines · X KB` at its right end (`prompt_meta`). The box renders the prompt as markdown and clamps it with **Show all N lines** / **Show less**, through `clamped_markdown` in `surfaces/profile_page.rs`, the helper the role and crew pages use. The clamp's fade must end in the prompt box's own background, not the page's panel colour. The panel keeps its own expanded flag, which resets when the selected chat changes. An empty or whitespace-only prompt shows no section, as today.
- **Unchanged from #756**: the identity (avatar or provider mark, display name, `@handle`), the setup rows (Runtime, Model, Effort) and the `cmd`, `cwd` and `session_key` rows with the copy button.
- **Width**: the panel is resizable (`chat_panel_width`); at its narrowest the label row, the link and the meta fit on one line, and markdown wraps inside the box.
- Presentation only: same data, same commands, no backend changes.

## Non-goals

- **Permissions**: dropped from the original scope. Direct chats strip permission flags at spawn (`strip_permission_flags`, feature 596), so a role's permission mode never applies to a chat, and showing it would describe a setting the chat does not use.
- Editing the role from the panel; Open role goes to the page, which edits in place.
- Terminal panes, which keep the panel closed.
- The mission workspace's rail and slot cards.

## Verification

- `runner-app` tests pass, extended where the panel's structure changes; workspace clippy is clean.
- Manual pass: a role chat whose prompt runs to several hundred lines (collapsed by default, Show all expands, Show less collapses), a short prompt (no toggle), Open role landing on the role page, a runtime chat with no role (no link, no prompt), and the panel dragged to its narrowest width, in both themes.
- macOS and Windows.
