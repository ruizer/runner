# 797 — Replace hook-status file feeds with local IPC

> Tracking issue: [#797](https://github.com/yicheng47/runner/issues/797)
> Priority: P2, 0.13. Platforms: macOS and Windows.
> Status: direction recorded on 2026-10-03; implementation design pending.

## Motivation

Runner's native agent hooks currently report status through per-session NDJSON feeds and payload files. A short-lived reporter writes the event, and Runner watches and reads the files. This works for direct chats and missions, but carries file creation, watcher, partial-read, and cleanup machinery for live status delivery.

Replace this transport with a small Runner-owned local IPC callback while retaining native CLI hooks and the interactive PTY. Jason requested this direction during the runtime walkthrough on 2026-10-03 after reviewing cmux and Orca.

## Scope

- Add a hook-reporting CLI entry point, provisionally `runner hook report`, that reads the provider payload from stdin and sends it to a local listener.
- Reuse Runner's Unix-socket / Windows-named-pipe infrastructure in `crates/runner-backend/src/ipc.rs`. Evaluate reuse of the existing MCP service against a dedicated hook endpoint; the message format, endpoint, and acknowledgment contract remain implementation decisions.
- Carry runtime, Runner session identity, launch generation, event name, and provider payload. Preserve stale-generation rejection and runtime-specific interpretation through the session-state boundary in #791.
- Acknowledge admission promptly and process events in an app/host-owned queue. Bound reporter execution and preserve each provider's neutral hook response so telemetry does not interrupt the agent.
- Cover existing hook-status producers on macOS and Windows. Audit reporters that also emit conversation identity before retiring their file path.
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
- Session-host operation keeps reporting while the cockpit is closed and reconciles status on reattach.

## References

- [cmux hook admission](https://github.com/manaflow-ai/cmux/blob/main/CLI/CMUXCLI%2BAgentHookAdmission.swift): shell hook invokes `cmux hooks enqueue`; the CLI sends `agent.hook.enqueue` through the socket, admits to an app-owned queue, and returns `{}`. Some paths also use file spooling.
- [Orca hook POST command](https://github.com/stablyai/orca/blob/main/src/main/agent-hooks/hook-post-command.ts): shell script uses `curl` to POST payload and routing metadata to a local HTTP listener with bounded request time.
- [Orca restart handling](https://www.onorca.dev/docs/agents/hooks-memory): hook invocations reload the published endpoint so long-lived agents can reach the server after restart.
- Related: #791 (session state), #645 (session host), #795 (shell/agent boundary).

