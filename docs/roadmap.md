# Runner roadmap

Snapshot as of 2026-09-27. The live source is the [GitHub milestones page](https://github.com/yicheng47/runner/milestones); this file mirrors it so the state of the project is readable from the repo without a browser. Update it when an issue changes milestone, a release is cut, or a mission lands, and move the date.

## Where the project is

- **Latest release:** [0.12.0](https://github.com/yicheng47/runner/releases/tag/v0.12.0) on 2026-09-27: redesigned role and crew pages and lists, accurate text-field selection, and undo and redo. The nightly feed builds from `main`.
- **In flight:** [Antigravity CLI](https://github.com/yicheng47/runner/pull/714), [OpenCode](https://github.com/yicheng47/runner/pull/716), and the [Codex rollback status fix](https://github.com/yicheng47/runner/pull/737) have open PRs with macOS and Windows checks green. The 0.13 headline specs are missions as containers ([562](./features/562-mission-spawn.md)) and session-to-session prompts ([704](./features/704-session-send.md)).

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
| [0.12](https://github.com/yicheng47/runner/milestone/2) | Patch follow-through: GPUI dependency, two runtimes, tab close and Codex rollback status | #733, #644, #592, #725, #736 |
| [0.13](https://github.com/yicheng47/runner/milestone/3) | Mission coordination and session-to-session prompts, Activity and notifications, PTY evaluation | #562, #704, #552, #701, #709 |
| [0.14](https://github.com/yicheng47/runner/milestone/4) | Session host, shell process status, 简体中文 | #645, #586, #565 |

A minor is a change to the model or a new surface; a patch is fixes and follow-through. Patch releases have carried features before (0.8.4 to 0.8.8), which is fine for small ones, but #562 migrates every mission's roster and moves with #704 to the next minor, 0.13.

## Open work by release

There are 23 open issues: five in 0.12, five in 0.13, three in 0.14, and ten unscheduled. The two 0.12.0 release blockers, [#393](https://github.com/yicheng47/runner/issues/393) and [#699](https://github.com/yicheng47/runner/issues/699), shipped and closed. The remaining 0.12 milestone is for patch follow-through. The mission-roster migration [#562](https://github.com/yicheng47/runner/issues/562) needs a minor release, so it and its paired terminal layer [#704](https://github.com/yicheng47/runner/issues/704) moved to 0.13. The session host moved to 0.14 so the PTY evaluation can inform its process boundary. A milestone is a release track, not a promise that every issue gates its first release.

| Release | Issue | Reason and ordering |
| --- | --- | --- |
| 0.12 | [#733](https://github.com/yicheng47/runner/issues/733) pin GPUI to Zed | The yanked dependency blocks a fresh resolve; spike the upstream port before more UI work |
| 0.12 | [#644](https://github.com/yicheng47/runner/issues/644) Antigravity CLI runtime | P1; [PR #714](https://github.com/yicheng47/runner/pull/714) is open with both platform checks green |
| 0.12 | [#592](https://github.com/yicheng47/runner/issues/592) OpenCode runtime | [PR #716](https://github.com/yicheng47/runner/pull/716) is open with both platform checks green |
| 0.12 | [#725](https://github.com/yicheng47/runner/issues/725) close a single chat or terminal tab with ⌘W | A visible tab-close bug with a narrow fix; it need not wait for the next minor |
| 0.12 | [#736](https://github.com/yicheng47/runner/issues/736) Codex rollback status | A 0.12.0 regression with [PR #737](https://github.com/yicheng47/runner/pull/737) green on both platforms |
| 0.13 | [#562](https://github.com/yicheng47/runner/issues/562) missions as containers | P1 headline; migrate the roster model and settle lifecycle contracts before the dependent UI |
| 0.13 | [#704](https://github.com/yicheng47/runner/issues/704) session-to-session prompts | P1 headline; the terminal layer beside #562, with no bus or new tables |
| 0.13 | [#552](https://github.com/yicheng47/runner/issues/552) Activity / Needs you view | Consolidate working and waiting sessions across windows and missions |
| 0.13 | [#701](https://github.com/yicheng47/runner/issues/701) desktop notifications | Offscreen waits, questions, completions and failures reach the user |
| 0.13 | [#709](https://github.com/yicheng47/runner/issues/709) Alacritty PTY and event loop evaluation | Decide the terminal engine before extracting it into the session host |
| 0.14 | [#645](https://github.com/yicheng47/runner/issues/645) session host | Local host first, then ssh remotes and the Windows host; use #709's PTY decision |
| 0.14 | [#586](https://github.com/yicheng47/runner/issues/586) shell process status | Keep process observation on the host that owns the PTY |
| 0.14 | [#565](https://github.com/yicheng47/runner/issues/565) i18n, 简体中文 first | Extract and translate after the mission and role/crew surfaces settle |

## Decisions that shape the next releases

- **Runner is not an agent development environment.** Worktree isolation ([#403](https://github.com/yicheng47/runner/issues/403)) and the project tree with a read-only diff viewer ([#634](https://github.com/yicheng47/runner/issues/634)) were closed as not planned on 2026-09-22. Checkouts, file trees and diffs belong to git, the agents and the editor; Runner stays what sits between agents.
- **The README is the front door.** The landing page ([#468](https://github.com/yicheng47/runner/issues/468)) was closed as not planned the same day: visitors arrive on GitHub, and the README already explains roles, crews and missions in both languages.
- **One `runner` binary, two modes.** Inside a mission the commands are unchanged and `--mission` defaults to the caller's own; outside, `--mission` is a flag. No second control binary, and no scoping by packaging: a mission agent's shell runs anything on `PATH`. A limit, if ever wanted, is an authorization check on the app side of `mcp.sock`.
- **A caller is the person at the app or a roster handle, never a location.** `human` on the bus means only the person at the app UI. Inside a mission the handle comes from `RUNNER_HANDLE`; outside, `--as <handle>` names a seat the caller holds, and no handle means the person at a terminal. An outside agent that drives a mission takes a seat first. The socket's post and signal tools carry a `from` handle validated against the roster. Seats without a Runner-spawned session are #562's.
- **Runner's MCP integration was removed for 0.11.0, the release that ships the CLI.** Two ways in at once would have been confusing. The `runner-mcp` bridge, the registrations Runner wrote, and the pinned row in Settings → MCP are gone; the socket stays as the CLI's implementation-detail transport, and Settings → MCP remains the catalog of the user's own servers.
- **#648 and #562 stay separate.** 648 is a surface over tools that exist and shipped in 0.11.0; 562 changes the mission model and is scheduled for 0.13. 648 reserves the coordinator verbs (`spawn`, `ps`, `wait`, `stop`, `done`); a `peek` at a worker's terminal is not supported, messages are the contract, and #562 owns the seats and the verbs.
- **pi has no MCP client, by design** (its README: "No MCP. Build CLI tools with READMEs, or build an extension that adds MCP support"). Runner writes no pi MCP entry, the Settings MCP pane has no pi column, and the Skills pane lists pi's two personal roots read-only. Crews coordinate through the bundled `runner` CLI, so nothing is missing, and from 0.11.0 no runtime uses MCP to reach Runner.

## Backlog

These ten open issues have no release commitment. P2 work moved out of 0.12 on 2026-09-27 to keep the shipped minor's follow-through small; setting a milestone schedules it again.

- **P2:** [#559](https://github.com/yicheng47/runner/issues/559) command palette; the existing quick switcher remains.
- **P2:** [#577](https://github.com/yicheng47/runner/issues/577) per-role skills and MCP picks; the global catalogs already work.
- **P2:** [#630](https://github.com/yicheng47/runner/issues/630) token ledger, re-runnable tasks and subscription quota.
- **P2:** [#723](https://github.com/yicheng47/runner/issues/723) Grok Build and Cursor Agent runtimes; evaluate after the two runtime PRs in 0.12.
- **P2:** [#724](https://github.com/yicheng47/runner/issues/724) Codex transcript mouse-wheel scrolling with a pinned header; PageUp works.
- **P2:** [#729](https://github.com/yicheng47/runner/issues/729) remove the write-only crew purpose and goal columns; a schema drop needs a downgrade note.
- **P2:** [#730](https://github.com/yicheng47/runner/issues/730) Japanese kana glyph corruption on macOS; retest after the GPUI migration.
- **P2:** [#731](https://github.com/yicheng47/runner/issues/731) role side panel and [#735](https://github.com/yicheng47/runner/issues/735) Start Chat modal visual alignment; design together when there is capacity.
- **P3:** [#582](https://github.com/yicheng47/runner/issues/582) split files that outgrew the #478 audit; time it around feature work on those files.
