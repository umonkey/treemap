# ADR 0021: Tree State Revamp

- Date: 2026-08-31
- Status: accepted

## Context

The tree mapping application previously stored tree states as arbitrary strings (such as healthy, sick, deformed, dead, stump, gone, replaced), leading to inconsistent queries, lack of compile-time safety, and poor handling of malformed or unrecognized database values. Furthermore, fine-grained health conditions like sick and deformed added operational complexity without providing consistent arboreal value. We need a robust, strongly typed tree state model with well-defined lifecycle phases and safe error handling.

## Decision

We have replaced string-based tree states with a strongly typed TreeState enum and standardized the state lifecycle with the following design:

- tree state enum definition: strongly typed enum supporting alive, dead, stump, gone, replaced, error, placeholder, and unknown variants.
- state consolidation: consolidating sick, deformed, and healthy states into alive.
- error state introduction: introducing error state for invalid or unprocessable tree records.
- safe fallback: mapping unrecognized strings, garbage values, and null database entries to treestate::unknown.
- domain encapsulation: helper methods such as is_alive and is_existing on treestate and tree models.

This decision is based on:

- type safety: compile-time guarantees preventing invalid state strings across backend services and API handlers.
- data cleanliness: simplified lifecycle tracking by merging redundant health categories into alive.
- resilience: graceful fallback parsing ensuring database corruption or legacy values do not panic the backend.

## Consequences

- database migration: migration script 008-tree-state-alive.sql updating legacy sick, deformed, and healthy records to alive.
- model updates: tree and treelocation models updated to use treestate instead of string.
- service and query adaptations: api schemas, search queries, spatial services, tree mergers, and osm writers updated to handle the new treestate variants.
