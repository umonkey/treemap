# ADR 0015: Multi-tenancy Support

- Date: 2026-07-19
- Status: accepted

## Context

As the application expands to serve different cities and communities, we need a way to support multiple "instances" or "tenants" using a single deployment. Each instance (e.g., Yerevan, Dilijan, Gyumri) should have its own domain, branding, and support channels, while sharing the same underlying backend infrastructure to minimize maintenance and operational costs.

We need a system that:

- Allows a single backend to serve multiple frontend domains.
- Validates that a domain is an authorized and enabled instance of the platform.
- Identifies the tenant context in every authenticated request.
- Scales without requiring separate database or server deployments for each new city.

## Decision

We will implement multi-tenancy support using a domain-based instance identification mechanism.

### Data Model

- Instances table: a new table in the SQLite database to store authorized domains and their metadata (name, description, tech support email, and enabled status).
- Unique domain constraint: ensures each domain maps to exactly one instance.

### Authentication & Identification

- Domain validation: during the login process, the backend extracts the domain name from the `state` parameter and verifies it against the `instances` table.
- JWT instance claim: the validated domain name is included in the JWT as a custom `instance` claim.
- Token validation: authentication fails if the user attempts to log in via an unauthorized or disabled domain.

### Infrastructure

- Single database: all instances share the same database and server process, allowing for unified maintenance and data analysis.

## Consequences

- Operational efficiency: adding a new community only requires adding a record to the `instances` table with no additional infrastructure.
- Tenant isolation: the `instance` claim provides the foundation for future data scoping (e.g., instance-specific tree lists).
- Domain-driven branding: frontends can use instance info to present correct names and support contacts.
- Security: unauthorized domains are prevented from using the API.
