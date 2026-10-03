---
name: mcp-server
description: Architecture and implementation rules for extending the internal Rust remote MCP server. Use ONLY when editing services/backend/src/services/mcp/**. Do NOT use when calling external MCP tools (treemap_*, sentry_*).
---

# Remote Model Context Protocol (MCP) Server

Guidelines and architecture for the built-in remote MCP server in the Rust backend (`services/backend`).

## Core Architecture & Protocol

- Specification: supports the modern `2026-07-28` revision and the legacy `2025-11-25`, `2025-06-18`, and `2025-03-26` revisions (dual-era).
- Transport: Streamable HTTP over a single `POST /mcp` endpoint.
  - `POST /mcp`: receives JSON-RPC 2.0 requests and returns `application/json` responses, or `202 Accepted` with no body for notifications.
  - `GET /mcp` and `DELETE /mcp`: return `405 Method Not Allowed`.
  - Modern era: detected by `params._meta["io.modelcontextprotocol/protocolVersion"]`; requires matching `MCP-Protocol-Version` and `Mcp-Method` headers (and `Mcp-Name` for `tools/call`), implements `server/discover`, and adds `resultType`, `ttlMs`, and `cacheScope` metadata.
  - Legacy era: answers `initialize` and `notifications/initialized`.
- Session Management: none. The server is fully stateless, mints no `Mcp-Session-Id`, and never emits `text/event-stream` because all tool handlers are synchronous request and response calls.

## Adding or Modifying Tools

When adding or updating MCP tools in `services/backend/src/services/mcp/service.rs`:

1. Define tool metadata in `tool_definitions()`:
   - `name`: snake_case identifier (e.g. `list_tallest`, `get_street_stats`).
   - `description`: concise summary of what the tool does and unit details (e.g. crown vs trunk measurements).
   - `input_schema`: JSON schema defining expected parameters, types, descriptions, minimum/maximum constraints, and required fields.
2. Implement handler method in `McpService`:
   - Parse and validate arguments from `JsonValue`.
   - Query domain repositories or database.
   - Return `CallToolResult::success(vec![McpContent::text(...)])` or `CallToolResult::error_text(...)`.
3. Route tool call in `handle_tools_call`:
   - Match on `tool_name` string and dispatch to the corresponding handler.
   - Serialize with `call_tool_result_value(era, ...)` so legacy responses omit the modern `resultType` field.

## Coding Style & Error Handling

- Follow Rust backend coding style (see `rust` skill).
- Return valid JSON-RPC error responses with standard error codes (`INVALID_REQUEST`, `METHOD_NOT_FOUND`, `INVALID_PARAMS`), plus `HEADER_MISMATCH` (`-32020`) and `UNSUPPORTED_PROTOCOL_VERSION` (`-32022`) for modern header and version validation.
- Keep the pure era negotiation, header validation, base64 decoding, and result-shaping helpers in `schemas.rs` so they remain unit-testable without a database.
- Do not log where you throw; log errors at boundaries or in handlers.
- Verify changes by running `make format` and `make check` / `make test` in `services/backend`.
