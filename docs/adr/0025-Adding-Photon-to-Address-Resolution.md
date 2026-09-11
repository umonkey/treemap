# ADR 0025: Adding Photon for MCP Building Lookups

- Date: 2026-09-11
- Status: accepted

## Context

AI agents use the remote MCP server to compose official requests and reports based on an alert's coordinates. These documents need the closest building address, including the house number. The existing `get_address` MCP tool resolved addresses with Nominatim only, which at zoom=16 returns a street but no building number, and at zoom=18 often returns the wrong adjacent street. Photon (photon.komoot.io) resolves the closest building directly and returns the house number, so we add it as an additional source for the MCP tool. Tree handling is not affected: tree addresses keep using Nominatim at zoom=16 exactly as before.

## Decision

We add Photon as a building-level address source used only by the MCP `get_address` tool, with Nominatim kept as a street-level fallback:

- scope: used only for on-demand MCP address requests, while tree addresses continue to be resolved by Nominatim at zoom=16.
- provider order: the MCP tool queries Photon first for a building number, and falls back to Nominatim at fixed zoom=16 when Photon errors or finds no building.
- address format: street first, for example `"Azatutyan Avenue 13/1"`.
- client addition: a new `PhotonClient` is added under `src/infra/photon/`, following the existing injectable client pattern.

This decision is based on:

- accuracy: building-level house numbers are available that Nominatim cannot reliably provide without the adjacent-street confusion of zoom=18.
- isolation: the new source is confined to the MCP path, so tree address behavior stays unchanged.
- reliability: falling back to Nominatim keeps MCP address resolution working when Photon is unavailable or has no data.

## Consequences

- new external dependency: the Komoot Photon service becomes part of the MCP address lookup path, with its own availability and usage policy considerations.
- rate and usage: the public Photon instance has no hard documented rate limit, but production traffic should remain modest; Nominatim's existing 1 rps usage policy still applies to the fallback.
- unchanged trees: automated tree address updates remain Nominatim street-level and are not affected.
- consistent formatting: addresses are formatted street first, which differs from the typical Nominatim `display_name` ordering.
- graceful degradation: failures are logged and do not break the MCP lookup as long as Nominatim can return a street.
- testability: both clients are injectable via the existing dependency injection pattern, so fallback behavior can be tested without external calls.

## Alternatives considered

- Nominatim zoom=18 only: rejected because it often returns the wrong adjacent street's building address.
- Nominatim zoom=16 only: rejected because it never returns a building number, so agents could not identify the closest building.
- switching tree address resolution to Photon: rejected because trees do not need building numbers and should keep their current street-level behavior.
