# ADR 0011: Adding Panoramas for Tree Mapping

- Date: 2026-06-01
- Status: accepted

## Context

Taking individual photos of every tree in the field is a time-consuming process that slows down the overall tree mapping performance. While individual photos provide detailed views of specific defects, many basic attributes like species, size, and general health status can be accurately identified by observing the tree from multiple angles using panoramic imagery.

Furthermore:

- Scalability: drive-by imagery allows for rapid capture of street-level data, potentially covering an entire city 1-2 times per year, which is unfeasible with manual individual tree inspections.
- Automation Path: high-quality street-level imagery is a prerequisite for moving towards automated tree detection and health assessment using machine learning.

## Decision

We will implement a "panoramas" feature, modeled after Google/Yandex Street View. This will involve recording 360-degree imagery in the field and integrating it into our mapping workflow.

Key implementation choices:

- Backend: we will use Mapillary as our initial backend for hosting imagery and managing metadata. This leverages their existing infrastructure for image processing and storage.
- Viewer: we will use Pannellum as our panoramic viewer instead of Mapillary's native viewer. Pannellum is simpler, faster, and allows us to exclude unnecessary navigational features that don't fit our specific triangulation workflow.

The application will support "armchair mapping", where users can use these panoramas to triangulate tree positions and identify species and health status from their computers. Detailed photos will still be taken for specific defects or anomalies when a "signal" is detected from the panoramic view.

## Consequences

- Increased Efficiency: significantly reduces the time required to map trees in the field.
- Improved Temporal Resolution: enables frequent updates of city-wide tree data.
- Automation Readiness: provides the dataset necessary for future AI-driven mapping and monitoring.
- Controlled UX: using Pannellum ensures a focused interface without the overhead of the Mapillary viewer's standard UI.
- Dependency Management: introducing Mapillary (hosting) and Pannellum (rendering) as core components.
