# Trilateration and Triangulation Feature

## Overview

Urban tree mapping frequently encounters challenging environmental conditions where direct GNSS or GPS locking at the tree trunk is degraded, obstructed, or entirely impossible. Dense tree canopies, narrow alleys, building canyons, courtyards, and overhead foliage obstruct satellite visibility, leading to high coordinate variance and positioning errors.

The trilateration tool solves this problem by allowing arborists and field volunteers to measure distances from known reference points using a laser range finder. By combining known ground control point coordinates with measured radial distances from multiple vantage points, the application computes precise tree positions via multi-point triangulation.

## Step-by-step Workflow

- Setting GCPs: the user begins at the setup page (`/tools/range`) where up to four ground control points (GCPs) can be defined. GCPs represent easily identifiable physical landmarks such as building corners, light poles, or permanent street infrastructure. Coordinates can be entered manually or picked directly using an interactive map dialog with location assistance.
- Measuring distances: once GCPs are configured, the user proceeds to the measurement entry page (`/tools/range/enter`). Here, the user sights trees from the reference points using a laser range finder and inputs the measured radial distances.
- Real-time calculation and live preview: as distance measurements are entered, the system instantly computes the estimated tree coordinates and renders a live interactive map preview (`MapPreview`) displaying GCP markers, intersection circle radii, and the resulting candidate tree position.
- Recording trees: the user can record multiple calculated tree positions locally into a batch session as they survey along streets or parks.
- Adjusting GCPs mid-survey: when moving along long avenues or broad survey areas, the user can seamlessly update or change active GCPs on the fly without losing recorded session data.
- Batch submission: once field data collection is complete, the user navigates to the submission page (`/tools/range/submit`) to review recorded trees and batch-submit them to the central database as default blank tree entries for subsequent detailed profiling.

## Technical Architecture

- Routes: the feature is organized under the `/tools/range` namespace, comprising three primary pages:
  - Setup route (`/tools/range` or setup view) for managing GCP reference points.
  - Measurement entry route (`/tools/range/enter`) for capturing laser distances and viewing the live map preview.
  - Submission route (`/tools/range/submit`) for reviewing and batch-submitting recorded trees.
- Storage and state management: the tool utilizes reactive Svelte 5 runes state classes (`RangeToolStore`) combined with `localStorage` persistence to maintain GCP configuration and recorded tree batches across navigation steps and browser sessions.
- Security permissions: creating trees via batch submission requires the `tree:create` permission, enforced by the backend API and frontend role-based access control.
- Mathematical triangulation algorithm: located in the triangulation utility (`triangulation.ts`), the algorithm performs the following steps:
  - Metric Cartesian projection: projects WGS84 geographic coordinates of GCPs into local metric Cartesian coordinates relative to a reference origin.
  - Circle intersection: computes intersections and closest approach points between distance circles centered at the GCP coordinates.
  - Residual error minimization: scores candidate intersection points by minimizing residual sum-of-squares error across all provided distance measurements.
  - WGS84 reprojection: converts the optimized Cartesian coordinates back into standard WGS84 latitude and longitude values for map rendering and database storage.
