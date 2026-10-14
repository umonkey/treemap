# Remote Model Context Protocol (MCP) Server

The application includes a built-in remote Model Context Protocol (MCP) server that exposes the tree database to AI assistants, LLMs, and autonomous agents over standard network protocols. This allows MCP-compliant clients to query tree data, analyze urban forestry metrics, and perform maintenance planning tasks without local setup or direct database access.

## Architecture and Transport

The remote MCP server implements the Streamable HTTP transport over a single endpoint and supports both protocol eras for maximum client compatibility.

- Transport: Streamable HTTP with a single endpoint, `POST /mcp`. It returns `Content-Type: application/json` for requests and `202 Accepted` with an empty body for notifications.
- Modern era: the server implements the `2026-07-28` revision, including per-request `_meta` version and capability data, `server/discover`, required `MCP-Protocol-Version`, `Mcp-Method`, and `Mcp-Name` request headers, `resultType` on every result, and `ttlMs` plus `cacheScope` cache hints on list results.
- Legacy era: the server answers `initialize` and `notifications/initialized` for the `2025-11-25`, `2025-06-18`, and `2025-03-26` revisions.
- Stateless: the server is fully stateless, mints no `Mcp-Session-Id`, and keeps no server-side session state. All supported tools are synchronous request and response calls, so no `text/event-stream` response is ever emitted.
- Unsupported methods: `GET /mcp` and `DELETE /mcp` return `405 Method Not Allowed`, and an unknown modern method returns `404 Not Found` so clients can distinguish this server from the deprecated HTTP+SSE transport.
- Origin validation: the server does not enforce an `Origin` allowlist, matching the public API's permissive CORS posture. This deviates from the specification's MUST requirement for servers that validate origins.

## Available Tools

The MCP server provides several specialized tools for querying and analyzing tree data:

- `list_tallest`: returns a list of the tallest trees in Yerevan sorted by height descending.
  - limit: number of trees to return (default 10, maximum 100).

- `list_widest`: returns a list of the widest trees in Yerevan based on crown canopy diameter. Results are sorted by crown diameter descending. Note that diameter refers to crown diameter in meters and circumference refers to trunk circumference in meters.
  - limit: number of trees to return (default 10, maximum 100).

- `list_streets`: returns a list of streets with aggregated tree counts and data completeness statistics.
  - limit: number of streets to return (default 10, maximum 100).
  - sort: field to sort results by (`count`, `street`, or `completeness`).

- `get_street_stats`: returns detailed tree health counts and attribute update freshness breakdowns for a specific street.
  - street: the name of the street to query (required).

- `list_alerts`: returns a list of citizen feedback alerts with optional filtering by status and ID, ensuring complete anonymity.
  - since_id: only return alerts with ID greater than this value.
  - status: filter by alert status (default new, pass empty string or null for all).
  - limit: number of alerts to return (default 20, maximum 100).
  - order: sort order by ID (`asc` or `desc`, default `asc`).

- `get_alert`: returns details of a specific citizen feedback alert by ID, with complete anonymity.
  - id: the ID of the alert to retrieve (required).

- `get_address`: resolves an address for the given coordinates, including the building number by default.
  - lat: latitude of the location (required).
  - lon: longitude of the location (required).
  - include_building: whether to include the building number (default `true`).

## Client Setup and Configuration

To connect an MCP-compliant client (such as Claude Desktop, OpenCode, Cursor, or Cline) to the remote MCP server, add the server configuration to your client configuration file.

For production use against the official deployment:

- endpoint: `https://api.treemaps.app/mcp`

Example JSON configuration for client applications:

```json
{
  "mcpServers": {
    "treemaps-yerevan": {
      "url": "https://api.treemaps.app/mcp"
    }
  }
}
```

## Common Use Cases

The remote MCP server enables various automated and research workflows:

- data auditing: querying incomplete tree records across streets to identify missing metrics such as height, crown diameter, or photographs.
- urban forestry research: analyzing tree height and canopy width distributions across different neighborhoods or streets.
- maintenance prioritization: identifying streets with high ratios of incomplete data, dead trees, or outdated observations to plan field inspections and maintenance campaigns.
