# 764 — Support MCP management for Pi

> Tracking issue: [#764](https://github.com/yicheng47/runner/issues/764)
> Priority: P2, 0.18 (moved from 0.13 to 0.15 on 2026-10-07 and to 0.16 on 2026-10-08, renumbered 0.18 on 2026-10-10). Platforms: macOS and Windows.

## Motivation

Pi has added native Model Context Protocol (MCP) support, storing user-level MCP configuration in `~/.pi/agent/mcp.json`.

Runner currently coordinates MCP management across Claude Code (`~/.claude.json`), Codex (`~/.codex/config.toml`), Antigravity CLI (`~/.gemini/config/mcp_config.json`), GitHub Copilot CLI (`~/.copilot/mcp-config.json`), and TRAE CLI (`~/.trae/traecli.toml`) under Settings → MCP. Users can inspect registered servers, resolve cross-client conflicts, edit server definitions natively, sync configs across multiple installed agents with "Also update", and toggle Runner's own MCP server registration.

Adding Pi as a first-class MCP client brings it to parity with Runner's other supported agent runtimes.

## Scope

- **Backend (`crates/runner-backend/src/ops/mcp.rs`)**:
  - Add `McpClientId::Pi` variant to `McpClientId`.
  - Add Pi mappings:
    - Key: `"pi"`.
    - Label: `"Pi"`.
    - Config file display: `"~/.pi/agent/mcp.json"`.
    - Config path: `home.join(".pi/agent/mcp.json")`.
    - Format: JSON `mcpServers` object (`is_json() == true`).
    - `for_runtime(Runtime::Pi)` and `runtime()` mapping.
  - Include `McpClientId::Pi` in `McpClientId::ALL`.
  - Catalog aggregation: parse and aggregate entries from `~/.pi/agent/mcp.json`.
  - Support stdio and streamable HTTP server entries.
  - Write and remove operations: support saving server definitions and toggling Runner's own MCP server in `~/.pi/agent/mcp.json`.
- **Frontend (`crates/runner-app/src/surfaces/settings/mcp.rs`)**:
  - Include Pi in the Settings → MCP client dropdown when Pi is installed and enabled in Settings → Agents.
  - Support Pi in the "Also update" multi-select list.
  - Show Pi in the server detail panel with native JSON representation.
  - Report conflict states and transport badges matching Pi's capabilities.
- **Documentation**:
  - Track in `docs/features/README.md`.

## Non-goals

- Managing project-local `.pi/mcp.json` files (Runner MCP management focuses on user-level agent configs across all clients).
- Legacy SSE transport (Pi only supports `stdio` and `streamable-http`).

## Implementation phases

1. **Backend**: Add `McpClientId::Pi` to `crates/runner-backend/src/ops/mcp.rs`, implementing config path resolution, JSON reading, writing, and Runner MCP registration.
2. **Frontend**: Extend Settings → MCP in `crates/runner-app/src/surfaces/settings/mcp.rs` to support Pi in client pickers, detail views, and edit flows.
3. **Verification**: Unit and UI tests covering Pi MCP catalog parsing, conflict checking, and config mutation.

## Verification

- `runner-backend` and `runner-app` tests pass; workspace clippy is clean.
- Unit tests:
  - Reading valid `~/.pi/agent/mcp.json` with stdio and http servers.
  - Writing and removing servers while preserving unrelated JSON fields and comments/formatting where possible.
  - Conflict detection between Pi and other clients.
  - Registration toggle for Runner's internal MCP server.
- UI tests:
  - Settings → MCP shows Pi when installed and enabled, filters it out when disabled or unavailable.
  - Multi-client save updates Pi's config when selected.
