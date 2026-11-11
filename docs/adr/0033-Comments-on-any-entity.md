# ADR 0033: Comments on any entity

- Date: 2026-11-11
- Status: accepted

## Context

Comments belonged to a tree through a mandatory foreign key, a tree-scoped repository, and tree-scoped endpoints. The frontend kept the comment components inside the tree route.

Water sources, panoramas, alerts, and future objects need the same discussion thread. A table per entity would duplicate the storage, the repository, the API, and the UI, and every new entity would need a migration and a new set of queries. One table keyed by an entity type and id keeps one implementation and one moderation surface.

## Decision

Store comments against a polymorphic entity type and entity id, and expose them through one generic API.

- The column stays open text with no database-level enumeration, so a new entity type needs no schema migration. The backend maps the known values to a closed enum, which currently contains only trees. Writing an unknown type is rejected as a bad request; reading an unknown stored value fails as a data error rather than silently defaulting.
- Reads are public. Posting reuses the existing comment-creation permission instead of introducing a second name for the same action.
- The tree comment count stays tree-specific: a comment affects it only for trees.
- A comment may have no author, which marks it as written by the system. Public posting still records the authenticated user; only internal callers can omit the author.
- The comment components become reusable and take an entity type and id, so any surface can render or post a thread without tree-specific code.

## Rejected alternatives

- One comment table per entity: typed foreign keys, but it duplicates the storage, repository, API, and UI for every entity and turns a global feed into a union of tables.
- A database check constraint for the entity type: enforces the known values in the schema, but every new entity needs a table rebuild. The backend enum gives the same rejection without touching the schema.
- A new write permission: clearer name, but it describes the same operation as the existing one and would leave two permissions to grant.
- Keeping the tree endpoints as aliases: avoids the break, but keeps a tree-specific URL and a second code path for a generic resource.

## Consequences

- One table, one repository, one API, and one set of components serve every entity. Adding a type is a backend change, not a migration.
- The generic API is a breaking change for clients of the tree comment endpoints, with no compatibility alias.
- The database cannot prove that an entity id still exists. Deleting an entity leaves its comments orphaned, as deleting a tree already did.
- Entity types are a closed set in the backend until a surface is added. The frontend may name future types, but calling the API with one fails until the backend accepts it.
- A system comment has no author: it does not affect any user's count and leaves no actor in the audit trail, but it still counts toward the tree's comment count.
- The tree comment count is denormalized. Any future entity that needs a count must add its own column or count on demand.
