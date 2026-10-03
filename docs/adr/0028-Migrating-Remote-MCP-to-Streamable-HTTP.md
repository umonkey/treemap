# ADR 0028: Migrating the Remote MCP Server to Streamable HTTP

- Date: 2026-10-03
- Status: accepted

## Context

The remote MCP server originally implemented the `2024-11-05` HTTP+SSE transport: `GET /mcp` opened a Server-Sent Events stream and emitted an `endpoint` event pointing at `POST /mcp/message?session_id=<uuid>`, responses were pushed over SSE, and sessions lived in an in-memory `McpSessionManager` with a 15-second heartbeat. That transport is deprecated and is not supported by many modern MCP clients.

The Model Context Protocol has since standardized the Streamable HTTP transport on a single endpoint, and later revisions removed server-side sessions entirely. The backend's MCP tools are all fast, synchronous request and response calls against SQLite, Nominatim, and Photon, so no server-initiated streaming is required.

Alternatives considered:

- Adopting an SDK such as `rmcp` or `rust-mcp-sdk`: adds a dependency and a framework or DI refactor that does not fit the backend's hand-rolled serde and JSON-RPC Action-Domain-Responder style.
- Modern-era only: some clients still open with `initialize`, and a modern-only server would reject them. The specification explicitly permits a dual-era server.
- Retaining HTTP+SSE alongside Streamable HTTP: the old transport is deprecated, and keeping it would preserve the session and channel machinery we want to delete.

## Decision

We replace the deprecated HTTP+SSE transport with a single-endpoint Streamable HTTP server that supports both protocol eras, implemented by hand with no new dependencies.

Key characteristics:

- Single endpoint: `POST /mcp` returns `application/json` for JSON-RPC requests and `202 Accepted` with no body for notifications. `GET /mcp` and `DELETE /mcp` return `405 Method Not Allowed`.
- Dual era: the modern `2026-07-28` revision is detected by the presence of `params._meta["io.modelcontextprotocol/protocolVersion"]`, and legacy clients negotiate `initialize` against the `2025-11-25`, `2025-06-18`, and `2025-03-26` revisions.
- Modern validation: requests must carry matching `MCP-Protocol-Version`, `Mcp-Method`, and, for `tools/call`, `Mcp-Name` headers. Header mismatches return `-32020 HeaderMismatch`, unsupported versions return `-32022 UnsupportedProtocolVersionError` with `data`, and unknown modern methods return `404 Not Found` with `-32601`.
- Modern metadata: every result carries `resultType`, list results carry `ttlMs` and `cacheScope`, and `server/discover` advertises the supported versions, capabilities, and server info.
- Stateless: sessions, the `Mcp-Session-Id` header, the `mcp-channel` message endpoint, the SSE stream, and `McpSessionManager` are removed. All seven tools and their behavior are preserved.
- Origin posture: the server continues to allow any origin through the global CORS layer and does not enforce an `Origin` allowlist, documenting the deviation from the specification's MUST requirement.

This ADR supersedes the transport and session portions of ADR 0012; the decision to provide a remote MCP server remains in force.

## Consequences

- Modern clients can use the current Streamable HTTP revisions, while initialize-based legacy clients continue to work.
- The codebase shrinks: the SSE action, the message action, the session manager, and the `Context::mcp` accessor are deleted.
- Clients that only speak the deprecated `2024-11-05` HTTP+SSE transport must upgrade; they now fail by design.
- The MCP endpoint stays public and read-only, consistent with ADR 0012.
- Because the server is stateless, there is no idle-heartbeat traffic and no session cleanup to maintain.
