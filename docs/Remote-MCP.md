# Remote Model Context Protocol (MCP) Server

The application includes a built-in remote Model Context Protocol (MCP) server that exposes the tree database to AI assistants, LLMs, and autonomous agents over standard network protocols. This allows MCP-compliant clients to query tree data, analyze urban forestry metrics, and perform maintenance planning tasks without local setup or direct database access.

## Architecture and Transport

The remote MCP server adheres to the Model Context Protocol specification version `2024-11-05` and implements Server-Sent Events (SSE) combined with HTTP POST for bidirectional communication.

- SSE transport endpoint: `GET /mcp` initiates a new session, sends an initial `endpoint` event containing the message dispatch URL (`/mcp/message?session_id=<uuid>`), and maintains a 15-second keep-alive heartbeat.
- JSON-RPC message endpoint: `POST /mcp/message?session_id=<uuid>` receives JSON-RPC 2.0 requests (such as `initialize`, `ping`, `tools/list`, and `tools/call`) and returns synchronous JSON-RPC responses.
- Session management: active sessions are tracked in memory and cleaned up automatically via drop handlers when the client disconnects.

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

## Client Setup and Configuration

To connect an MCP-compliant client (such as Claude Desktop, OpenCode, Cursor, or Cline) to the remote MCP server, add the server configuration to your client configuration file.

For production use against the official deployment:

- endpoint: `https://yerevan.treemaps.app/mcp`

Example JSON configuration for client applications:

```json
{
  "mcpServers": {
    "treemaps-yerevan": {
      "url": "https://yerevan.treemaps.app/mcp"
    }
  }
}
```

## Common Use Cases

The remote MCP server enables various automated and research workflows:

- data auditing: querying incomplete tree records across streets to identify missing metrics such as height, crown diameter, or photographs.
- urban forestry research: analyzing tree height and canopy width distributions across different neighborhoods or streets.
- maintenance prioritization: identifying streets with high ratios of incomplete data, dead trees, or outdated observations to plan field inspections and maintenance campaigns.
