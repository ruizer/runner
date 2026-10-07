# 797 — Replace hook-status file feeds with local IPC

> Tracking issue: [#797](https://github.com/yicheng47/runner/issues/797)
> Priority: P2, 0.13.x (Jason, 2026-10-07: moved up to end the Windows Codex hook stall). Platforms: macOS and Windows.
> Status: direction recorded on 2026-10-03; Windows hook latency added 2026-10-06 from the PC regression (F6); implementation design pending.

## Motivation

Runner's native agent hooks currently report status through per-session NDJSON feeds and payload files. A short-lived reporter writes the event, and Runner watches and reads the files. This works for direct chats and missions, but carries file creation, watcher, partial-read, and cleanup machinery for live status delivery.

Replace this transport with a small Runner-owned local IPC callback while retaining native CLI hooks and the interactive PTY. Jason requested this direction during the runtime walkthrough on 2026-10-03 after reviewing cmux and Orca.

On Windows the file feed is also slow enough to feel. The 2026-10-06 PC regression (F6 in `docs/tests/archive/2026-10-06-pc-full-regression.md`) found that Escape in a Codex chat visibly stalls with Runner's hooks on and is instant with `--disable hooks` or in a plain PowerShell outside Runner; the first Codex spawn is slower than a plain Runner shell for the same reason. Codex waits for every hook, and each Windows hook starts a new PowerShell that runs the ScriptBlock reporter: median 313 ms, p95 322 ms per event on that PC (Windows PowerShell 5.1, 30 samples), of which about 140 ms is PowerShell starting with nothing to do.

## Windows: the shell floor

An IPC reporter alone does not remove that stall. Codex 0.159.3 runs every command hook through the selected local shell, PowerShell on Windows with `cmd /C` as fallback, and never executes the command directly. A `runner hook report` command inside a command hook still pays the shell start, measured at about 150 ms per event on the same PC. cmux and Orca use command hooks on macOS, where starting `sh` costs a few milliseconds, so their design does not carry this cost there.

Removing the shell on Windows needs a reporter that stays alive for the session. Codex 0.159.3 supports `mcp_tool` hook handlers: Codex starts a stdio MCP server once per session and sends each hook to it as a tool call, with no process per event. For Codex on Windows, the reporter is therefore a long-lived `runner` CLI mode (provisionally `runner hook serve`) that Codex launches as a per-invocation stdio server and that forwards each event to the same listener as `runner hook report`. It is a second entry into the one hook transport, not a separate status path, and it exposes no tools beyond hook admission. Agents still control Runner only through the CLI.

Two findings from the regression constrain it:

- **Root and child identity.** In Codex 0.159.3, `session_id` is shared by the root thread and all descendants (`core/src/session.rs`), and the MCP argument template cannot carry the optional `agent_id` that the command reporter uses to drop sub-agent events. The hook MCP executor always attaches the concrete `threadId` in request `_meta` (`core/src/hook_mcp_executor.rs`), and `rmcp_client` forwards it. The reporter binds the root thread from the root-only `SessionStart` and treats any other `threadId` as a child. Missing metadata must not adopt a root.
- **Startup readiness.** Hook MCP calls do not wait for the server to be ready, so the first `SessionStart` can miss a slow reporter. Marking the server `required` with a short startup timeout guarantees the binding but turns a slow reporter into a failed Codex start instead of a missing status. The design has to choose one and measure the cost against the first-spawn budget below.

Other runtimes keep command hooks on Windows unless they offer a similar long-lived handler: Claude Code runs its hooks under its own Git Bash, which starts in about 40 ms on the same PC, and pi reports from Node in-process. GitHub Copilot CLI uses the same per-event PowerShell reporter as Codex (inline in its plugin's `powershell` slot), so it likely pays the same cost per event; measure it and check whether Copilot offers a long-lived handler before deciding its Windows path. Check which shell Codex selects: the regression PC resolves `pwsh` to the Store PowerShell 7.6 as well as the legacy 5.1, and Codex prefers `pwsh` when it finds one, so measured costs there may be higher than the 5.1 numbers above.

## Scope

- Add a hook-reporting CLI entry point, provisionally `runner hook report`, that reads the provider payload from stdin and sends it to a local listener.
- Reuse Runner's Unix-socket / Windows-named-pipe infrastructure in `crates/runner-daemon/src/ipc.rs`. Evaluate reuse of the existing MCP service against a dedicated hook endpoint; the message format, endpoint, and acknowledgment contract remain implementation decisions.
- Carry runtime, Runner session identity, launch generation, event name, and provider payload. Preserve stale-generation rejection and runtime-specific interpretation through the session-state boundary in #791.
- Acknowledge admission promptly and process events in an app/host-owned queue. Bound reporter execution and preserve each provider's neutral hook response so telemetry does not interrupt the agent.
- Cover existing hook-status producers on macOS and Windows. Audit reporters that also emit conversation identity before retiring their file path.
- On Windows, register Codex hooks as `mcp_tool` handlers against the long-lived reporter described in Windows: the shell floor, with `threadId`-based child filtering and a stated startup-readiness policy. Keep the `--disable hooks` and `features.hooks=false` opt-outs.
- Coordinate listener ownership with #645: it must live alongside the agent in the session host and remain available when the cockpit closes. The host issue currently retains file reporters; agree migration sequencing without making this a new release blocker.
- Keep mission coordination logs separate from this status-transport change. No vendor app-server protocol or replacement of the interactive CLI is in scope.

## Implementation phases

1. Define the callback envelope, delivery/ordering contract, listener ownership, and MCP-versus-dedicated-endpoint decision using the peer implementations below.
2. Implement the shared reporter and receiver, then migrate native hooks and extension reporters while preserving runtime observations.
3. Verify parity and failure behavior, then retire superseded status-feed and payload-file machinery.

## Verification

- Hook-derived status and conversation identity behave consistently in direct chats and missions; shell sessions do not require this channel.
- Concurrent sessions route events correctly, stale launch generations are rejected, and event ordering preserves turn boundaries.
- An unavailable or restarting listener does not hang or fail the agent; document the chosen loss/retry behavior and verify it.
- macOS shell reporters and Windows reporters both deliver through local IPC.
- On Windows, a Codex chat with hooks on shows no visible Escape difference from one started with `--disable hooks`, and its first spawn is within a stated small bound of the hooks-off spawn. Measure per-event hook cost before and after with the same harness, sample count and selected shell, and prove from process creation that no shell starts per Codex event.
- A sub-agent's `Stop`, tool or `UserPromptSubmit` event does not change the root session's status.
- Session-host operation keeps reporting while the cockpit is closed and reconciles status on reattach.

## References

- [cmux hook admission](https://github.com/manaflow-ai/cmux/blob/main/CLI/CMUXCLI%2BAgentHookAdmission.swift): shell hook invokes `cmux hooks enqueue`; the CLI sends `agent.hook.enqueue` through the socket, admits to an app-owned queue, and returns `{}`. Some paths also use file spooling.
- [Orca hook POST command](https://github.com/stablyai/orca/blob/main/src/main/agent-hooks/hook-post-command.ts): shell script uses `curl` to POST payload and routing metadata to a local HTTP listener with bounded request time.
- [Orca restart handling](https://www.onorca.dev/docs/agents/hooks-memory): hook invocations reload the published endpoint so long-lived agents can reach the server after restart.
- Windows evidence: F6 in `docs/tests/archive/2026-10-06-pc-full-regression.md` and its retained research files (hook cost table, Codex 0.159.3 source extracts).
- Related: #791 (session state), #645 (session host), #795 (shell/agent boundary), #813 (Windows live-regression input harness).

