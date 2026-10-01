# 772 — Start chats, terminals and missions from the keyboard

> Tracking issue: [#772](https://github.com/yicheng47/runner/issues/772)
> Priority: P2, 0.12. Platforms: macOS and Windows.
> Status: draft proposal, 2026-10-01; open questions at the end.

## Motivation

Start a chat (⌘N) is the main way into Runner, and it is built for the mouse. Here is what the keyboard does today, read from `surfaces/start_chat.rs` and `ui/overlay.rs`:

- **Cancel.** Esc closes the modal from any control in it, through `Modal`'s key handler. If a select's menu is open, Esc closes the menu first. Nothing on the Cancel button shows this.
- **Start chat.** Enter starts the chat only when focus is in a text field (Chat name, Working directory, Model), through `on_start_chat_key_down`. On the role or agent picker, or on Runtime, Effort or Speed, Enter opens the menu. On a Direct/Role segment it switches mode, and on Reset it resets. So whether Enter starts the chat depends on where focus is, no key starts it from every control, and the button shows no key.
- **Direct | Role.** These are two separate tab stops, each picked with Enter or Space. Focus opens in Chat name, below the cards, so reaching the switch takes Shift-Tab back through every control and Reset. There are no arrow keys and no shortcut.
- **Workspace keys go through.** GPUI matches key bindings before key-down listeners, and the modal declares no key context. So while it is open, ⌘1–9, ⌘D and ⌘[ still act on the workspace behind it.
- **The other two kinds.** The sidebar's + menu offers New chat, New mission and New terminal, but only New chat has a key. New terminal is a ⌘K palette command, and it opens in the terminal drawer, not as a tab like the + menu's. New mission has neither a key nor a palette command; you reach it with the mouse, from the + menu or a crew page.

## Proposal

### 1. Cancel and Start chat

- **⌘↵ starts the chat from anywhere in the modal** (Ctrl+Enter on Windows): from a text field, a picker, a select with its menu open, Reset, or a footer button. An open menu closes without picking its highlighted row, and the chat starts with the values the controls show. ⌘↵ does nothing when Start chat is disabled (no role, no enabled agent), while the chat is starting, or while an IME is composing.
- **Plain Enter and Esc keep their current behavior.** Enter still starts the chat from a text field, opens a picker, and presses a focused button. Esc still closes an open menu, otherwise the modal. Neither key changes meaning.
- **The footer shows the keys.** Cancel shows `esc` and Start chat shows `⌘↵`, as trailing keycaps in the faint meta style the sidebar's New chat row uses for ⌘N. The labels come from the keymap, so Windows shows `Ctrl+↵`.
- **Start mission gets the same ⌘↵ and keycaps.** It shares `Modal` and has the same split between Enter in a field and Enter on a picker.
- **Mechanism.** The modal root declares a `StartChat` key context (and Start mission a `StartMission` one), with a confirm action bound to `cmd-enter` in it. Bindings dispatch before key-down listeners, so `StyledSelect::on_key_down`, which matches `enter` without checking modifiers, never sees ⌘↵. The binding is a fixed keymap entry, so Settings → Keymap lists it but it cannot be rebound, like Close pane.

### 2. Direct | Role

- **One tab stop.** The switch becomes a segmented control with a single tab stop: ←/→ move between Direct and Role while it has focus, and Enter and Space keep working.
- **⌘1 Direct, ⌘2 Role** from anywhere in the modal, shown as faint keycaps inside the segments. The bindings live in the `StartChat` context, which is deeper than the global Select tab 1/2 bindings, so they win while the modal is open without changing the tab keys. Binding them there also stops ⌘1 and ⌘2 reaching the workspace behind the modal.
- **Switching moves focus to that mode's picker** (the role picker or the agent picker), because picking who the chat is with is why you switched. The same switch by mouse leaves focus where it is.
- **Initial focus goes to the remembered mode's picker,** not Chat name. The quickest paths become: ⌘N ⌘↵ for the defaults, ⌘N ↵ ↓↓ ↵ ⌘↵ for a different role or agent, and ⌘N ⌘2 ↵ ↓ ↵ ⌘↵ from Direct mode. Chat name is optional and comes one Tab later (open question 1).

### 3. Chats, terminals and missions: three keys, no merged form

Recommendation: give each kind its own key, and keep each kind's form or no form. Do not put one "New…" modal with a kind switch in front of them.

| Key (macOS / Windows) | Action | Form |
| --- | --- | --- |
| ⌘N / Ctrl+N | New chat (unchanged) | Start a chat |
| ⌘T / Ctrl+T | New terminal | None: fills the focused empty pane, otherwise opens a new tab, in the active project's directory or the default directory, as the + menu's New terminal does today |
| ⇧⌘M / Ctrl+Shift+M | New mission | Start mission, scoped to the active project, as the + menu's New mission does today |

- All three are rebindable keymap entries (`new-chat` already exists; add `new-terminal` and `new-mission`). The + menu rows show their keys, and ⌘K lists New chat, New terminal and New mission as commands.
- On Windows, Ctrl+T now reaches Runner before the shell, so a Windows shell loses Ctrl+T (transpose characters). Ctrl+N and Ctrl+D already make the same trade, and the key can be rebound.

Why not one modal with Chat | Terminal | Mission at the top:

- A terminal needs no form. A modal would add a step to the quickest action.
- A mission asks for different things (crew, goal) and opens a different surface. A kind switch above Direct | Role would nest one segmented control inside another.
- With three keys, each form stays short. The + menu and the palette list the keys, so they are easy to find.

The alternative was a small chooser on ⌘N (Chat / Terminal / Mission, then the form). It adds a keystroke to the most common action, so the proposal leaves it out.

## Non-goals

- Changing what the forms ask for, how sessions start, or where a chat lands.
- Type-to-filter in the role and agent pickers (open question 5).
- Keyboard work on modals other than Start a chat and Start mission.

## Open questions

1. **Initial focus:** the picker (proposed) or Chat name (today)? The picker makes ⌘N ⌘↵ and picking a role the fast paths. Chat name suits people who name every chat.
2. **⌘T in the mission workspace:** always a chat-surface tab (proposed, consistent with ⌘N), or a shell in the mission's drawer like the palette's New terminal? The drawer already has ⌥F12 and its own +.
3. **The palette's New terminal** opens in the drawer, but the + menu's opens a tab. Should the palette follow ⌘T?
4. **Mission key:** ⇧⌘M (proposed, M for mission) or ⌥⌘N (an alternate New)?
5. **Type-ahead in pickers:** letters jump to the first role whose handle or name starts with them. This is cheap in `StyledSelect` and matters once there are a dozen roles. Add it here, or file it separately?
6. **Report check:** the code says Esc already closes the modal and Enter starts the chat from Chat name, which is where focus opens. If either did nothing when you tried it, that is a bug to reproduce, for example focus not landing in the modal when ⌘N fires from a terminal pane, and it comes first.

## Implementation phases

1. **Design:** `design/specs/772-keyboard-create.pen` in the root checkout, holding only the Start a chat frame with footer keycaps, segment keycaps and the switch's focus ring, and the + menu with keys. Signed off before code.
2. **Modal keys:** the `StartChat` and `StartMission` key contexts with confirm and mode actions, the single-stop segmented switch, initial focus, the focus move on a mode switch, and keycaps from the keymap.
3. **Create shortcuts:** `NewTerminal` and `NewMission` actions with keymap entries, the + menu keys, and palette commands. Keep the empty-pane rule shared with ⌘N.

## Verification

- `runner-app` tests: ⌘↵ starts the chat from a text field, a closed picker, an open menu without taking its highlight, Reset and Cancel; it does nothing when Start chat is disabled, while starting, or while composing. ⌘1/⌘2 switch mode and focus the picker without changing the active tab behind the modal. ←/→ work on the switch, and the tab order holds. ⌘↵ works in Start mission. The keymap defaults include the Windows mapping test. ⌘T fills the focused empty pane, otherwise opens a new tab, and ⇧⌘M opens Start mission for the active project.
- Workspace Clippy is clean.
- Jason's smoke test on macOS and Windows, with no mouse: ⌘N ⌘2, pick a role, ⌘↵; ⌘T; ⇧⌘M, pick a crew, ⌘↵.

## Relevant code

- `crates/runner-app/src/surfaces/start_chat.rs`: `on_start_chat_key_down`, `render_mode_button`, `start_chat_focus_order`, initial focus in `open_start_chat_modal`, `new_terminal` and `new_terminal_tab`.
- `crates/runner-app/src/surfaces/start_mission.rs`: the Start mission modal.
- `crates/runner-app/src/ui/overlay.rs`: `Modal`'s Esc and Tab handling.
- `crates/runner-app/src/ui/select.rs`: `StyledSelect::on_key_down`, where Enter ignores modifiers.
- `crates/runner-app/src/keymap.rs`: keymap entries, Windows defaults, and binding contexts.
- `crates/runner-app/src/surfaces/sidebar/menus.rs`: the + menu's create entries; `surfaces/sidebar/elements.rs` for the New chat row's keycap style.
- `crates/runner-app/src/surfaces/command_palette.rs`: palette commands.
