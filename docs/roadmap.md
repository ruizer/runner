# Runner roadmap

Snapshot as of 2026-10-01. The live sources are [GitHub milestones](https://github.com/yicheng47/runner/milestones), [open issues](https://github.com/yicheng47/runner/issues), and [published releases](https://github.com/yicheng47/runner/releases); this file mirrors them so the state of the project is readable from the repo without a browser. Issue state, milestone assignments and release-blocker labels take precedence over milestone descriptions, which can lag. Update it when an issue changes milestone, a release is cut, or a mission lands, and move the date.

## Where the project is

- **Latest release:** [0.12.5](https://github.com/yicheng47/runner/releases/tag/v0.12.5) on 2026-09-30: the Start a chat redesign, model and effort in the chat side panel, removal of unused crew fields, and model-default, usage, keyboard and font fixes. Antigravity CLI and Codex Speed shipped in earlier 0.12.x patches.
- **Merged since that release:** the GPUI move to `gpui-pre =0.3.7` ([#767](https://github.com/yicheng47/runner/pull/767), closing [#733](https://github.com/yicheng47/runner/issues/733)), sidebar label/action fixes ([#769](https://github.com/yicheng47/runner/pull/769)), the New role / New crew pages with the role side panel prompt and Open role ([#770](https://github.com/yicheng47/runner/pull/770), closing [#768](https://github.com/yicheng47/runner/issues/768) and [#731](https://github.com/yicheng47/runner/issues/731)), and ⌘W closing the tab instead of the window, with ⇧⌘W (Alt+F4 on Windows) for the window ([#773](https://github.com/yicheng47/runner/pull/773), closing [#725](https://github.com/yicheng47/runner/issues/725)). These are on `main`; the nightly feed builds from `main`.
- **In flight:** mission monitoring from a Runner chat ([748](./features/748-mission-watch-delivery.md)) remains a P1 spec under review in 0.12.
- **Next minor:** missions as containers ([562](./features/562-mission-spawn.md)) and session-to-session prompts ([704](./features/704-session-send.md)) remain the 0.13 headline specs. #704 currently carries the `release-blocker` label.

## Releases

| Release | Content | Issues |
| --- | --- | --- |
| 0.11.0 | Shipped 2026-09-20: pi runtime, and the general `runner` CLI with its agent skill as Runner's external control surface; the MCP integration removed | #539, #648 |
| 0.11.1 | Shipped 2026-09-21: session status, Windows agent discovery, splitter feedback, mission accents, project inference, and default crew naming | #659, #670, #672, #673, #653, #680, #676 |
| 0.11.2 | Shipped 2026-09-21: Settings → Agents Installed / Not installed split, Codex stuck on Working at an untouched prompt | #617, #687 |
| 0.11.3 | Shipped 2026-09-22: title-aware Codex status fallback, pi resume with custom session directories, Not installed card layout | #688, #666, #695 |
| 0.11.4 | Shipped 2026-09-23: plan usage, shortcut controls, and the black-pane fix | #706, #697, #647 |
| 0.11.5 | Shipped 2026-09-26: agent CLI updates and terminal splits following the shell's live directory | #533, #575 |
| [0.12.0](https://github.com/yicheng47/runner/releases/tag/v0.12.0) | Shipped 2026-09-27: role and crew redesigns, text-field hit testing and undo and redo | #393, #699, #726, #728 |
| [0.12.1](https://github.com/yicheng47/runner/releases/tag/v0.12.1) | Shipped 2026-09-28: Codex Speed in the UI and CLI, correct conversation after `/clear`, rollback status recovery | #740, #743, #738, #736 |
| [0.12.2](https://github.com/yicheng47/runner/releases/tag/v0.12.2) | Shipped 2026-09-28: Codex transcript wheel scrolling and copied-file path paste on Windows | #724, #742 |
| [0.12.3](https://github.com/yicheng47/runner/releases/tag/v0.12.3) | Shipped 2026-09-29: Antigravity CLI runtime, live models, quota, managed skill and conversation/status follow-ups | #644, #747 |
| [0.12.4](https://github.com/yicheng47/runner/releases/tag/v0.12.4) | Shipped 2026-09-29: submit Codex inbox nudges correctly after Windows paste bursts | #753 |
| [0.12.5](https://github.com/yicheng47/runner/releases/tag/v0.12.5) | Shipped 2026-09-30: Start a chat and chat side panel redesigns, crew field removal, model defaults, usage visibility, pi multiline input and macOS kana fixes | #735, #756, #729, #762, #752, #755, #730 |
| [0.12](https://github.com/yicheng47/runner/milestone/2) | Remaining patch follow-through: idle Codex message delivery, mission monitoring and keyboard-driven creation | #766, #748, #772 |
| [0.13](https://github.com/yicheng47/runner/milestone/3) | Mission coordination and session-to-session prompts, Activity and notifications, PTY evaluation, runtime additions and pi MCP management | #562, #704, #552, #701, #709, #723, #764 |
| [0.14](https://github.com/yicheng47/runner/milestone/4) | Session host, shell process status, 简体中文 | #645, #586, #565 |

A minor is a change to the model or a new surface; a patch is fixes and follow-through. Patch releases have carried features before (0.8.4 to 0.8.8), which is fine for small ones, but #562 migrates every mission's roster, so it and the paired terminal layer #704 are planned for 0.13.

0.12.5 removes the crew purpose and goal columns. Older versions cannot read crews after that migration; the [release notes](https://github.com/yicheng47/runner/releases/tag/v0.12.5) carry the backup and downgrade guidance. The Windows Antigravity integration shipped in 0.12.3, but its native smoke test remains unverified in the [validation record](./tests/747-antigravity-followups.md).

## Open work by release

There are 19 open issues: three in 0.12, seven in 0.13, three in 0.14, and six unscheduled. The 0.11 milestone is closed. The remaining 0.12 milestone is for patch follow-through and has no open release blockers; #704 is the only open issue currently labeled `release-blocker`, in 0.13. The session host stays in 0.14 so the PTY evaluation can inform its process boundary. A milestone is a release track, not a promise that every issue gates its first release.

| Release | Issue | Reason and ordering |
| --- | --- | --- |
| 0.12 | [#766](https://github.com/yicheng47/runner/issues/766) idle Codex message delivery | P1; fix crew messages held at an empty composer in this patch cycle |
| 0.12 | [#748](https://github.com/yicheng47/runner/issues/748) mission monitoring from Codex | P1; spec under review for Runner-delivered mission notices in the starting chat, with idle delivery still to verify |
| 0.12 | [#772](https://github.com/yicheng47/runner/issues/772) keyboard-driven Start a chat, new terminal and new mission | P2; draft [spec](./features/772-keyboard-create.md) with open questions on initial focus and the new keys |
| 0.13 | [#562](https://github.com/yicheng47/runner/issues/562) missions as containers | P1 headline; migrate the roster model and settle lifecycle contracts before the dependent UI |
| 0.13 | [#704](https://github.com/yicheng47/runner/issues/704) session-to-session prompts | P1 release blocker; the terminal layer beside #562, with no bus or new tables |
| 0.13 | [#552](https://github.com/yicheng47/runner/issues/552) Activity / Needs you view | Consolidate working and waiting sessions across windows and missions |
| 0.13 | [#701](https://github.com/yicheng47/runner/issues/701) desktop notifications | Offscreen waits, questions, completions and failures reach the user |
| 0.13 | [#709](https://github.com/yicheng47/runner/issues/709) Alacritty PTY and event loop evaluation | Decide the terminal engine before extracting it into the session host |
| 0.13 | [#723](https://github.com/yicheng47/runner/issues/723) Grok Build and Cursor Agent runtimes | P2; evaluate against the [runtime integration checklist](./arch/runtime-integration.md) |
| 0.13 | [#764](https://github.com/yicheng47/runner/issues/764) pi MCP management | P2; proposed Settings → MCP integration; verify the native config contract and align the spec with Runner's CLI-only coordination |
| 0.14 | [#645](https://github.com/yicheng47/runner/issues/645) session host | Local host first, then ssh remotes and the Windows host; use #709's PTY decision |
| 0.14 | [#586](https://github.com/yicheng47/runner/issues/586) shell process status | Keep process observation on the host that owns the PTY |
| 0.14 | [#565](https://github.com/yicheng47/runner/issues/565) i18n, 简体中文 first | Extract and translate after the mission and role/crew surfaces settle |

## Decisions that shape the next releases

- **Runner is not an agent development environment.** Worktree isolation ([#403](https://github.com/yicheng47/runner/issues/403)) and the project tree with a read-only diff viewer ([#634](https://github.com/yicheng47/runner/issues/634)) were closed as not planned on 2026-09-22. Checkouts, file trees and diffs belong to git, the agents and the editor; Runner stays what sits between agents.
- **The README is the front door.** The landing page ([#468](https://github.com/yicheng47/runner/issues/468)) was closed as not planned the same day: visitors arrive on GitHub, and the README already explains roles, crews and missions in both languages.
- **OpenCode is no longer scheduled.** [#592](https://github.com/yicheng47/runner/issues/592) was closed as not planned on 2026-09-28; it is not shipped runtime support. New runtimes follow the [integration checklist](./arch/runtime-integration.md), with platform verification recorded separately from CI.
- **One `runner` binary, two modes.** Inside a mission the commands are unchanged and `--mission` defaults to the caller's own; outside, `--mission` is a flag. No second control binary, and no scoping by packaging: a mission agent's shell runs anything on `PATH`. A limit, if ever wanted, is an authorization check on the app side of `mcp.sock`.
- **A caller is the person at the app or a roster handle, never a location.** `human` on the bus means only the person at the app UI. Inside a mission the handle comes from `RUNNER_HANDLE`; outside, `--as <handle>` names a seat the caller holds, and no handle means the person at a terminal. An outside agent that drives a mission takes a seat first. The socket's post and signal tools carry a `from` handle validated against the roster. Seats without a Runner-spawned session are #562's.
- **Runner's MCP integration was removed for 0.11.0, the release that ships the CLI.** Two ways in at once would have been confusing. The `runner-mcp` bridge, the registrations Runner wrote, and the pinned row in Settings → MCP are gone; the socket stays as the CLI's implementation-detail transport, and Settings → MCP remains the catalog of the user's own servers.
- **#648 and #562 stay separate.** 648 is a surface over tools that exist and shipped in 0.11.0; 562 changes the mission model and is scheduled for 0.13. 648 reserves the coordinator verbs (`spawn`, `ps`, `wait`, `stop`, `done`); a `peek` at a worker's terminal is not supported, messages are the contract, and #562 owns the seats and the verbs.
- **pi MCP management is proposed, not shipped.** [#764](https://github.com/yicheng47/runner/issues/764) tracks adding pi to the user's MCP server catalog in 0.13. Runner currently writes no pi MCP entry and has no pi column in Settings → MCP; the Skills pane lists pi's two personal roots read-only. Mission coordination continues through the bundled `runner` CLI. The new spec's proposal to register Runner needs reconciling with the shipped MCP removal before implementation.

## Backlog

These six open issues have no release commitment; setting a milestone schedules them.

- **P2:** [#559](https://github.com/yicheng47/runner/issues/559) command palette; the existing quick switcher remains.
- **P2:** [#577](https://github.com/yicheng47/runner/issues/577) per-role skills and MCP picks; the global catalogs already work.
- **P2:** [#630](https://github.com/yicheng47/runner/issues/630) token ledger, re-runnable tasks and subscription quota.
- **P2:** [#774](https://github.com/yicheng47/runner/issues/774) a `claude_status` test can hang the Windows CI test step; a hang blocks nightly and release gating until the run is cancelled and rerun.
- **P3:** [#582](https://github.com/yicheng47/runner/issues/582) split files that outgrew the #478 audit; time it around feature work on those files.
- **P3:** [#771](https://github.com/yicheng47/runner/issues/771) evaluate selective `gpui-base` adoption for shared UI controls; start with one ordinary field and resolve the Windows manifest dependency before integration.
