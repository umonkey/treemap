# ADR 0024: Water Sources

- Date: 2026-09-09
- Status: accepted

## Context

Water availability is a first-order constraint on tree survival in urban environments. The platform previously had no representation of water infrastructure, so tree health analysis could not factor in water proximity, and planting decisions were made without knowledge of where water is accessible.

We need water source information to:

- Plan planting and maintenance: target watering efforts and prioritize irrigation where water access is scarce.
- Find dry areas with high risk of decline for trees: cross-reference water source proximity with tree health data to identify zones where trees are most at risk.
- Automate selection of spots for civic planting: filter candidate planting locations by proximity to water access.

## Decision

We will introduce a water sources domain: a point-based registry of water access points (fountains, hydrants, irrigation points) with a lifecycle status, exposed as a map layer and managed through permission-gated CRUD.

Key implementation choices:

- Data model: a `water_source` table (id, created_at, created_by, updated_at, lat, lon, status) with a `WaterStatus` enum (`operational`, `dead`, `gone`), created by migration `015-water-source.sql`.
- Domain: `WaterService` and `WaterRepository` handle add, update, get by id, and get by bounds; `get_sources` filters out `gone` sources so retired points disappear from the map.
- API: `POST /v1/water`, `PATCH /v1/water/{id}`, `GET /v1/water/{id}`, and `GET /v1/water/geo.json` (GeoJSON for the map layer); the `WaterSourceRead` DTO exposes ids as strings.
- Authorization: the `water:manage` permission (new `WaterManage` permission type) is required for add and update actions; reads remain public. The seed grants `water:manage` to the `editor` role, and the `admin` role bypasses the check.
- Frontend: a `WaterSourceLayer` map layer renders 50 meter radius discs and dots, keeps its enabled state from `mapLayerStore.water`, and publishes sticky points to `mapPoiStore.water`; an `AddWater` control (visible only with `water:manage`) opens the add flow; water details, move, and add pages are permission-aware.

## Consequences

- Enables dry-area risk analysis and automated planting-site selection, the stated goals of the feature.
- Introduces a new `water:manage` permission that must be seeded for non-admin roles.
- Data quality depends on field data collection; the `operational`/`dead`/`gone` lifecycle supports maintenance tracking.
- Future work implied: proximity queries between trees and water sources, and integration with civic planting workflows.
