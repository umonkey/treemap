# ADR-0006: Tree Observations and Revisioning

- Date: 2026-03-09
- Status: Accepted

## Context

We need a structured way to track tree health, damage, and beneficial features (like nests). Previously, this information was captured in free-text comments (e.g., "has a nesting box") or ambiguous top-level statuses like "sick" or "deformed."

The old status system had several flaws:

- Ambiguity: terms like "deformed" were not clearly defined.
- Exclusivity: a tree could not be marked as both "sick" and "deformed" because the status was a single choice.
- Complexity: unqualified volunteers struggled to accurately categorize trees using abstract status labels.

The goal is to provide a "Data Science friendly" tool that even unqualified volunteers can use. This requires a simplified version of professional standards like the [Visual Tree Assessment (VTA) Checklist](https://chainsawacademy.husqvarna.com/wp-content/uploads/2022/06/VTA.pdf), focusing only on traits that are easy to identify without specialized training.

## Decision

We will implement a structured "observations" system with the following characteristics:

- Simplified statuses: top-level tree statuses are restricted to basic physical existence (alive, dead, or gone). All health and condition details are moved to granular observation flags.
- Structured schema: observations are stored as a set of boolean flags in a dedicated `observations` table. This allows combining multiple traits (e.g., a tree can be "alive" while having both "bark damage" and an "inclined" trunk).
- Snapshot-based revisioning: every significant change creates a new record in the database, providing a full historical timeline of a tree's state.
- UI preloading: to facilitate updates and corrections, the observation form is pre-loaded with the values from the most recent record for that tree.
- Deduplication and grace period:
  - if a user submits an observation identical to the latest one, no new record is created.
  - if the latest record was created by the same user within the last hour, it is updated in place instead of creating a new revision. This allows users to correct misclicks without bloating the database with near-identical entries.
  - if a different user submits a change, or more than an hour has passed, a new revision is always created.
- Indefinite retention: all historical records are kept indefinitely to allow for long-term health trend analysis.

## Consequences

### Positive

- Analyzability: observations are now structured data, allowing for easy SQL queries and data science applications.
- Combinability: trees can now have multiple conditions recorded simultaneously, providing a more accurate picture of their health.
- Audit trail: we maintain a complete history of how a tree's condition has changed over time.
- User experience: pre-filling the form makes it much faster for users to record small changes or corrections.
- Data cleanliness: the 1-hour grace period reduces "noise" from accidental submissions or quick corrections.

### Negative

- Database growth: storing full snapshots for every revision uses more disk space than storing only deltas, though the records are small enough that this is not an immediate concern.
- Simplified data: by omitting complex VTA metrics, we may lack the depth required by professional arborists, though this is a deliberate trade-off for volunteer engagement.
