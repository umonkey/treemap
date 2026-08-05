# ADR 0018: Use OpenSfM for Street View Accuracy

- Date: 2026-07-27
- Status: accepted

## Context

We record 360 videos of driving through streets and extract frames at fixed intervals using consumer-grade hardware like mobile phones. The GPS data from these devices is highly inaccurate, making it impossible to pinpoint tree locations through triangulation or trilateration.

The project aims to work with affordable hardware, so expensive RTK/PPK solutions are not an option.

Initial testing shows that OpenSfM can provide good relative coordinates for images in a sequence, but the entire sequence may still have a global offset from the real world or the base map.

Our primary base map is OpenStreetMap (OSM). Since OSM data itself can have inaccuracies, we need our street view sequences to align with the OSM base layer rather than absolute real-world coordinates.

## Decision

We will use OpenSfM to reconstruct the scenery from extracted video frames and correct the input image coordinates.

This decision is based on:

- Cost effectiveness: it eliminates the need for expensive high-precision GPS hardware.
- Pipeline simplicity: software-based correction is easier to manage than specialized hardware logistics.
- Manual alignment: we will implement a feature in the admin area to manually offset entire sequences to match the OSM base layer.
- Automated tree detection: it opens the door for automated tree detection using machine learning, which can drastically improve the speed of data collection and potentially automate it.

## Consequences

- Improved relative accuracy: tree positions will be much more consistent within a sequence.
- Computational overhead: running OpenSfM adds a significant processing step.
- Manual effort: some human intervention is required to perform the initial alignment of new sequences in the admin area.
- Alignment focus: we explicitly prioritize alignment with OSM over absolute geographic accuracy.
- Automated data collection: using 3D reconstruction opens the door for machine learning-based tree detection, drastically increasing data collection speed.
