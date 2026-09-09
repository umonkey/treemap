# Water Sources

The platform maintains a registry of water access points (fountains, hydrants, irrigation points) to support urban forestry planning. Knowing where water is available lets the project plan planting and maintenance, identify dry areas with high risk of tree decline, and automate the selection of spots for civic planting.

## Goals

Water availability is a first-order constraint on tree survival. The water sources feature provides the spatial data needed to:

- Plan planting and maintenance: target watering efforts and prioritize irrigation where water access is scarce.
- Find dry areas with high risk of decline for trees: cross-reference water source proximity with tree health data to identify zones where trees are most at risk.
- Automate selection of spots for civic planting: filter candidate planting locations by proximity to water access.

## Using Water Sources

Water sources are shown on the map as a translucent blue disc showing the 50 meter coverage area of each operational source, with a clickable dot that opens the source details page. The layer can be toggled from the layers panel.

Users with the `water:manage` permission can add a new water source from the map control and move existing sources by re-centering the map. The Move button on the details page is disabled without this permission.

Each source tracks a lifecycle status (`operational`, `dead`, or `gone`); retired sources disappear from the map while dead sources remain visible for maintenance planning.

## Operational Notes

- Data quality depends on field data collection; the `operational`/`dead`/`gone` lifecycle supports maintenance tracking.
- The `water:manage` permission must be seeded for non-admin roles in any new environment.
- Future work implied: proximity queries between trees and water sources, and integration with civic planting workflows.
