# ADR 0023: Tree Triangulation With Ground Control Points (GCP)

- Date: 2026-09-03
- Status: accepted

## Context

Field arborists and mapping volunteers often need to map trees in difficult or obstructed positions where direct GPS locking at the tree trunk is inaccurate or impossible (e.g., dense canopy, courtyards, narrow alleys). Using laser range finders from known Ground Control Points (GCPs) allows accurate distance measurements to trees. We need a frontend feature that facilitates GCP coordinate selection on a map, distance data entry, trilateration/triangulation calculation, and visual verification.

## Decision

We have implemented the Tree Range Finder Triangulation feature in `services/frontend`:

- **Triangulation Utility (`triangulation.ts`)**: Projects WGS84 coordinates of GCPs and measured laser distances into local metric Cartesian space, computes circle intersections and closest approach points, scores candidates by residual error minimization, and reprojects back to WGS84.
- **Location Input & Picker Dialog (`LocationInput`, `LocationPickerDialog`)**: Enhanced location input component supporting coordinate display, clearing, and an interactive MapLibre map dialog with click-to-pick marker and "Use my location" functionality.
- **Tools Layout & Routes (`/tools/range`, `/tools/range/enter`)**: Dedicated layout for tools without full-screen background map, setup page for configuring up to 4 GCPs persisted in `localStorage`, and data entry page with live interactive map preview (`MapPreview`), distance inputs, and real-time triangulation calculation.
- **Unit Testing & Verification**: Comprehensive unit tests for triangulation logic in Vitest, alongside strict linting, formatting, and type-checking.

## Consequences

- **Improved Accuracy**: Enables precise tree positioning via multi-point triangulation when GPS reception is poor.
- **Enhanced UX**: Interactive map picker and visual map preview with radius circles and suggested tree markers.
- **Maintainability**: Modular utility functions, reactive Svelte 5 runes state classes, and comprehensive test coverage.
