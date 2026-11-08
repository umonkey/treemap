---
name: adr
description: When to write an architecture decision record (ADR) under docs/adr/. Use before creating an ADR, or when a plan or review proposes one.
---

# Architecture Decision Records

## Write an ADR only when

- The decision changes a contract shared by more than one feature or service: an API schema, a shared database table, a queue message, or an auth model.
- The change is hard or costly to reverse: a data migration that drops or rewrites data, a storage or hosting move, or a public API break.
- An obvious alternative is deliberately rejected and future maintainers would otherwise re-litigate it.

## Do not write an ADR for

- Isolated operator or admin tools, scripts, or read-only snapshots that no other feature depends on.
- Additive schema changes that do not change the behavior of an existing feature.
- Routine feature work, bug fixes, refactors, dependency bumps, or documentation.
- Recording a plan's decisions. Keep those in the plan or in code comments.

## Procedure

- When a change might qualify but you are not certain, ask the operator before creating a file under `docs/adr/`.
- If an ADR is not warranted, do not create a placeholder, stub, or empty file.
- Number files as `NNNN-Title.md` with the next free number, include the date, and state the status.
