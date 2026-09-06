---
name: mcp-server
description: Architecture and implementation rules for extending the internal Rust remote MCP server. Use ONLY when editing services/backend/src/services/mcp/**. Do NOT use when calling external MCP tools (treemap_*, sentry_*).
---

# Remote Model Context Protocol (MCP) Server

Guidelines and architecture for the built-in remote MCP server in the Rust backend (`services/backend`).

## Core Architecture & Protocol

- Specification: adheres to Model Context Protocol specification version `2024-11-05`.
- Transport: Server-Sent Events (SSE) combined with HTTP POST for bidirectional communication.
  - `GET /mcp`: initiates session, sends initial `endpoint` event containing message dispatch URL (`/mcp/message?session_id=<uuid>`), maintains 15s heartbeat.
  - `POST /mcp/message?session_id=<uuid>`: receives JSON-RPC 2.0 requests (`initialize`, `ping`, `tools/list`, `tools/call`) and returns synchronous JSON-RPC responses.
- Session Management: active sessions are tracked in memory via `McpSessionManager` and cleaned up automatically via drop handlers on client disconnect.

## Adding or Modifying Tools

When adding or updating MCP tools in `services/backend/src/services/mcp/service.rs`:

1. Define tool metadata in `handle_tools_list`:
   - `name`: snake_case identifier (e.g. `list_tallest`, `get_street_stats`).
   - `description`: concise summary of what the tool does and unit details (e.g. crown vs trunk measurements).
   - `input_schema`: JSON schema defining expected parameters, types, descriptions, minimum/maximum constraints, and required fields.
2. Implement handler method in `McpService`:
   - Parse and validate arguments from `JsonValue`.
   - Query domain repositories or database.
   - Return `CallToolResult::success(vec![McpContent::text(...)])` or `CallToolResult::error_text(...)`.
3. Route tool call in `handle_tools_call`:
   - Match on `tool_name` string and dispatch to the corresponding handler.

## Coding Style & Error Handling

- Follow Rust backend coding style (see `rust` skill).
- Return valid JSON-RPC error responses with standard error codes (`INVALID_REQUEST`, `METHOD_NOT_FOUND`, `INVALID_PARAMS`).
- Do not log where you throw; log errors at boundaries or in handlers.
- Verify changes by running `make format` and `make check` / `make test` in `services/backend`.
