# ADR 0012: Adding a Remote MCP Server for Programmatic Database Access

- Date: 2026-06-03
- Status: accepted

## Context

We need to enable AI agents to query the database programmatically to support automation tasks. This will allow mcp-enabled agents to perform data analysis, batch updates, and other maintenance tasks without manual intervention.

Alternatives considered:

- Downloading CSV or database dumps: this approach is too complex as it requires heavy scripting to maintain data synchronization and does not allow for real-time interaction.
- Local MCP installation: this requires manual installation and updates on every agent's machine, which is inconvenient and prone to version drift across different environments.

## Decision

We will implement a remote MCP (Model Context Protocol) server. This server will act as a bridge, providing a standardized interface for AI agents to interact with the project's database securely.

Key characteristics:

- Accessibility: the server will be reachable over the network, allowing agents to connect from anywhere without local setup.
- Standardization: by using the Model Context Protocol, we ensure compatibility with a wide range of AI tools and agents.
- Security: the remote server will allow us to centralize access control and auditing.

## Consequences

- Improved automation: enables mcp-enabled AI agents to perform complex data analysis and updates.
- Ease of use: agents can connect to a central, always-up-to-date server without local configuration.
- Centralized maintenance: simplifies updates as they only need to be applied to the remote server once.
- Security management: provides a single point of control for database access by automated tools.
