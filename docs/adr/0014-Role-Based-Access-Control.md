# ADR 0014: Role-Based Access Control (RBAC)

- Date: 2026-06-25
- Status: accepted

## Context

As the application grows, the need for fine-grained access control becomes critical. We have different types of users: regular volunteers, scientists, moderators, and system administrators. Hardcoding role-based checks across the codebase is brittle, hard to test, and doesn't scale as new features and specialized roles are added.

We need a system that:

- Decouples the identity from authorization.
- Provides type-safe permission checks at the API boundary.
- Allows dynamic assignment of permissions via roles without code changes.
- Is easy to maintain and audit.

## Decision

We will implement a fine-grained Role-Based Access Control (RBAC) system.

### Core Model

- Permission: an atomic, named capability (e.g., `tree:edit`, `pano:edit`, `user:manage`). Permissions are defined in the backend as zero-sized types implementing a `Permission` trait for compile-time safety and discoverability.
- Role: a collection of permissions.
- Assignment: users are assigned one or more roles. Their effective permissions are the union of all permissions associated with those roles.

### Technical Implementation

- Database schema: roles, permissions, role_permissions, and user_roles tables.
- Backend authorization: we use the `RequirePermission<P>` extractor in Actix-Web actions. This ensures that the permission check happens before the action logic is executed.
- IamService: this service is responsible for resolving the user's effective permissions from the database.
- ActorRights: this model handles the logic of checking if a set of resolved permissions includes the required one.
- Special roles: the admin role remains a super-user role that implicitly possesses all permissions.

## Consequences

- Type safety: developers must use predefined permission types, reducing the risk of typos in authorization logic.
- Declarative security: access requirements are clearly visible in the function signature of API handlers.
- Maintainability: changing what a role can do only requires a database update, rather than a code change across multiple files.
- Reduced boilerplate: the use of extractors removes the need to manually inject services and perform checks inside action bodies.
