# Feature specs

Active feature specs, one per open tracking issue. A spec moves to [`archive/`](./archive/) once its issue closes, shipped or dropped: the implementation is the source of truth and the archived spec is the "what we were going for" record. Priorities and milestones live on the issues; [`../roadmap.md`](../roadmap.md) mirrors them.

Since 2026-09-01 a spec's number **is** its tracking issue number: file the issue first, then name the spec after it, so gaps in the sequence belong to bugs and PRs. Specs 01–64 predate the alignment and keep their numbers in `archive/`; the then-active ones were renumbered to their issues (05→73, 58→393, 61→403), and 60 (fork a chat) kept its number and shipped 2026-09-01.

## Active

- [645 — runnerd](./645-session-host.md) — a background daemon, `runnerd`, owns the database, sessions, routers and the CLI socket, and the app becomes a client, so sessions and mission coordination outlive quitting or crashing the app; remote machines over ssh and the Windows PC as a crew slot follow ([#645](https://github.com/yicheng47/runner/issues/645), P1, 0.13, the sole release blocker; spec and [plan](../impls/645-runnerd/plan.md) landed 2026-10-05).
- [797 — Replace hook-status file feeds with local IPC](./797-hook-status-ipc.md) — native hooks report through a Runner-owned callback using local IPC, with bounded acknowledgment and listener ownership aligned with the session host; cmux and Orca references included ([#797](https://github.com/yicheng47/runner/issues/797), P2, 0.13; implementation design pending).
- [795 — Separate shell lifecycle from agent orchestration](./795-shell-agent-boundary.md) — shell sessions traverse agent launch and conversation policy through no-op adapters; keep shared process lifecycle while making the shell/agent boundary explicit ([#795](https://github.com/yicheng47/runner/issues/795), P1, 0.13; part of the [#645](https://github.com/yicheng47/runner/issues/645) session host design since 2026-10-04).
- [748 — Mission updates reach the chat that started the mission](./748-mission-watch-delivery.md) — Runner types a one-line notice into the starting chat for messages to the person, broadcasts, questions, failures and the end, through the slot delivery gate, so Codex chats watch missions with no host facility ([#748](https://github.com/yicheng47/runner/issues/748), P1, 0.13; spec under review).
- [562 — Missions as containers](./562-mission-spawn.md) — the mission owns its roster, a mission starts from a crew or a role, and the lead or an outside seat spawns, lists, waits on and stops slots ([#562](https://github.com/yicheng47/runner/issues/562), P1, 0.13, non-blocking; plan under review).
- [704 — Send a prompt from one session to another](./704-session-send.md) — `runner session send` and `session wait`, the terminal layer beside 562's missions: types into a running chat or terminal when it is idle, with a reply line, and stays off the bus ([#704](https://github.com/yicheng47/runner/issues/704), P1, 0.13, non-blocking; spec under review).
- [813 — Live-regression input harness](./813-windows-input-harness.md) — development-only `session key`, `session type` and busy send on top of 704, plus a scripted R1–R13/I1–I6 run, so a Codex QA can send input on Windows where computer use may only observe terminals ([#813](https://github.com/yicheng47/runner/issues/813), P2, 0.13; draft).
- [764 — Support MCP management for Pi](./764-pi-mcp-management.md) — proposed catalog, editing, multi-client sync and conflict detection for pi's user MCP servers; verify native config support and reconcile Runner registration with the shipped CLI-only coordination contract ([#764](https://github.com/yicheng47/runner/issues/764), P2, 0.13; spec under review).
- [559 — Command palette on ⌘⇧P](./559-command-palette.md) — the ⌘K overlay in command mode over the keymap ([#559](https://github.com/yicheng47/runner/issues/559), P2, 0.14).
- [565 — i18n, 简体中文 first](./565-i18n.md) — a language setting, a compile-time catalog, a live switch ([#565](https://github.com/yicheng47/runner/issues/565), P2, 0.14).
- [586 — Shell status: process detection first](./586-shell-status-detection.md) — foreground-process detection for shell panes before semantic shell integration ([#586](https://github.com/yicheng47/runner/issues/586), P2, 0.14).
- [701 — Desktop notifications](./701-desktop-notifications.md) — a popup Runner draws itself, following Zed's agent notification, when a session off screen waits on you, finishes or fails ([#701](https://github.com/yicheng47/runner/issues/701), P2, 0.13).
- [793 — Liquid glass appearance](./793-liquid-glass.md) — Diri-inspired layered glass chrome and floating controls, with readable terminal surfaces, a solid appearance option and Pencil design before implementation ([#793](https://github.com/yicheng47/runner/issues/793), P2, 0.13; draft).

## Recently shipped

Reconciled on 2026-10-04 against closed issues and published releases. These specs are archived; their bodies preserve the original scope and dated design decisions.

| Spec | Shipped in |
| --- | --- |
| [706 — Plan usage for Claude Code and Codex](./archive/706-agent-usage.md) | 0.11.4, 2026-09-23 |
| [533 — Update agent CLIs from Settings → Agents](./archive/533-agent-cli-updates.md) | 0.11.5, 2026-09-26 |
| [575 — Splits follow the shell's live cwd](./archive/575-live-cwd.md) | 0.11.5, 2026-09-26 |
| [393 — Role page redesign](./archive/393-role-page.md) | 0.12.0, 2026-09-27 |
| [699 — Crew page redesign](./archive/699-crew-page.md) | 0.12.0, 2026-09-27 |
| [740 — Codex speed in role settings](./archive/740-codex-role-speed.md) | 0.12.1, 2026-09-28 |
| [724 — Preserve terminal wheel coordinates](./archive/724-terminal-wheel-coordinates.md) | 0.12.2, 2026-09-28 |
| [644 — Antigravity CLI runtime](./archive/644-antigravity-runtime.md) | 0.12.3, 2026-09-29; native Windows smoke pending |
| [735 — Start a chat modal redesign](./archive/735-start-chat-modal.md) | 0.12.5, 2026-09-30 |
| [756 — Model and effort in the chat side panel](./archive/756-chat-side-panel-model-effort.md) | 0.12.5, 2026-09-30 |
| [731 — Role side panel redesign](./archive/731-role-side-panel.md) | 0.12.6, 2026-10-01 |
| [768 — New role and New crew open their pages](./archive/768-new-role-crew-pages.md) | 0.12.6, 2026-10-01 |
| [772 — Start chats, terminals and missions from the keyboard](./archive/772-keyboard-create.md) | `main` ([PR #776](https://github.com/yicheng47/runner/pull/776)), 2026-10-02; next 0.12.x |
| [777 — Runtime adapter trait](./archive/777-runtime-adapter.md) | `main` ([PR #779](https://github.com/yicheng47/runner/pull/779), [#780](https://github.com/yicheng47/runner/pull/780), [#792](https://github.com/yicheng47/runner/pull/792)), 2026-10-03; next 0.12.x, no user-visible change |
| [791 — Session state: one model for agent status, drafts and conversation identity](./archive/791-session-state.md) | `main` ([PR #796](https://github.com/yicheng47/runner/pull/796), [#798](https://github.com/yicheng47/runner/pull/798), [#800](https://github.com/yicheng47/runner/pull/800)), 2026-10-04; next 0.12.x; fixes #784, #785 and #786, #781 open pending a live check |

## Dropped

Considered and deliberately not built; the spec stays in `archive/` as the record.

- [19 — Mission split view](./archive/19-mission-split-view.md) ([#255](https://github.com/yicheng47/runner/issues/255)): missions coordinate turn-based, so side-by-side slot PTYs mostly show one busy terminal next to an idle one.
- [21 — Import native agent sessions](./archive/21-import-native-sessions.md) ([#176](https://github.com/yicheng47/runner/issues/176)): the CLIs' own resume pickers from a pane cover the need.
- [24 — Cronjobs](./archive/24-cronjobs.md) ([#193](https://github.com/yicheng47/runner/issues/193)): a resident scheduler is always-on machinery inside a cockpit; `mission start` from an external scheduler covers it.
- [53 — Session fork](./archive/53-session-fork.md) ([#348](https://github.com/yicheng47/runner/issues/348)): zero reaches in months of use; the native tier returned as spec 60 ([#398](https://github.com/yicheng47/runner/issues/398)).
- [466 — Sessions outlive the app](./archive/466-sessions-outlive-the-app.md) ([#466](https://github.com/yicheng47/runner/issues/466), 2026-09-02): PTYs stay in-process; the direction returned as the session host ([#645](https://github.com/yicheng47/runner/issues/645), now the sole 0.13 release blocker; moved forward from 0.14 on 2026-10-03).
- [491 — Confirm quit while work is running](./archive/491-confirm-quit-running-work.md) ([#491](https://github.com/yicheng47/runner/issues/491), 2026-09-07): a stopgap the session host makes obsolete.
- [511 — Ask about a selection](./archive/511-ask-about-selection.md) ([#511](https://github.com/yicheng47/runner/issues/511), 2026-09-10): no side thread forked from a selection.
- [557 — Translucent window backdrop](./archive/557-window-backdrop.md) ([#557](https://github.com/yicheng47/runner/issues/557), 2026-09-11): not important enough to carry.
- [403 — Opt-in worktree isolation per mission](./archive/403-mission-worktree-isolation.md) ([#403](https://github.com/yicheng47/runner/issues/403), 2026-09-22): Runner is not an agent development environment; a crew that needs its own checkout makes one from its brief.
- [468 — Landing page](./archive/468-landing-page.md) ([#468](https://github.com/yicheng47/runner/issues/468), 2026-09-22): visitors land on the README, which already explains the model; a short mission demo covers the rest.
