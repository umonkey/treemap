---
name: feature-docs
description: Audience, structure, and content rules for platform capability documentation under docs/features/<domain>/, written for the adopting organisation's technical decision-maker and the staff who operate a tenant or deployment.
---

# Feature Documentation

Generate and maintain capability documentation for the platform under `docs/features/<domain>/`.

## Audience

Write for two readers:

- The technical decision-maker at the adopting organisation or tenant: CTO, IT manager, digital services lead, GIS coordinator, or NGO technical lead. They decide whether a capability is safe, affordable, and possible to exit.
- The staff who configure and operate a tenant or deployment. They need to know what a capability costs, requires, and produces.

Do not write for:

- Field volunteers and mappers. They are served by in-app help and training sessions.
- The program lead evaluating whether an initiative is worth funding. That brief lives on the landing site.
- Developers. Architecture, setup, and internals belong in `docs/` and service-level `AGENTS.md` files.
- Procurement, legal, and security reviewers. Trust, contract, and compliance material lives on the website and is linked from the capability page when needed.

The test for every paragraph: does it help someone decide whether to adopt or operate this capability? If neither, cut it.

## Documentation Layers

Capability documentation is a pyramid. Each layer adds depth, and a reader can stop at any layer.

1. `docs/features/README.md`: the capability index. One row per capability, grouped under its domain, with a one-line description, availability, and maturity. It is the only place for the cross-domain matrix.
2. `docs/features/<domain>/README.md`: the capability page, or the domain hub when sub-features exist. One to two screens for a capability page, one screen for a hub.
3. `docs/<Topic>.md`: the operator reference. Configuration keys, commands, schemas, algorithms, and recovery procedures. It is shared by every capability that depends on the same infrastructure. Linked from the capability page, never duplicated on it.

Within a capability page, order sections so reading depth increases: decision summary first, links last.

## Domains and Sub-Features

- Split a domain into sub-features when its capabilities have distinct outcomes, prerequisites, configuration, or operational concerns that a reader evaluates separately. They need not be fully independent; one may support another, but each must stand as a coherent capability with its own decision summary.
- When a domain has sub-features, `docs/features/<domain>/README.md` becomes a hub. It contains a one-paragraph domain overview, the shared decision summary, shared requirements and cost, and a one-line link to each sub-feature. It does not repeat sub-feature content.
- Each sub-feature is a full page following the structure below. Sections that are genuinely shared may point to the hub instead of repeating them.
- Allow exactly one level of sub-features. If a sub-feature needs further splitting, the domain model is wrong.
- Keep the operator reference per topic, not per sub-feature. Import and export share one reference when they share infrastructure.

## Document Structure

Every capability page, including each sub-feature, follows this structure. Omit a section only when it is genuinely inapplicable, and say why if the omission is surprising.

1. Decision summary: three to six bullets covering fit, cost, risk, and availability. A decision-maker reads this and stops.
2. What it does: the capability and the outcome it produces, without implementation detail.
3. Requirements and availability: accounts, roles and permissions, device and browser limits, upstream data or state, and the configuration categories a tenant must set. Name the technology or policy behind every restriction. Distinguish tenant configuration from infrastructure configuration.
4. How a program runs: the end-to-end lifecycle at program level, including which steps are automatic and which need a person. Keep it declarative, with no click paths.
5. What the organisation gets: data, exports, publications, and audit history.
6. Cost and effort: compute, storage, and staff time, with realistic units.
7. Operations and limitations: what administrators monitor, how failures behave, where recovery is documented, and the honest constraints, quantified where possible.
8. Questions organisations usually ask: three to five recurring concerns, answered plainly.
9. Learn more: links to the operator reference, security material, and pricing.

## Availability and Delivery Model

- State whether a capability is available in the hosted service, in self-hosted deployments, or both.
- Keep availability coarse and stable. Do not encode plan names, prices, entitlements, or contract terms; those belong to the website.
- State device and browser limitations only with the reason attached, such as a missing browser API or a required permission.

## Writing Rules

- Lead with outcomes, then mechanics. Explain what the organisation can do, not how the code does it.
- Quantify. Costs in currency per unit, accuracy in metres, capacity in trees or sequences, effort in person-days.
- State limitations plainly and explain why they exist. A reader who discovers a limitation after adoption is a lost reference.
- Use the reader's vocabulary: capability, tenant, deployment, program, survey, tree, panorama, species, role, permission. Avoid internal names.
- Link to operator documentation for configuration keys, CLI commands, schemas, and internals. Never inline them.
- Keep every capability page readable in under five minutes and a domain hub to one screen. When a page grows, move depth to the operator reference instead of lengthening it.
- Prefer short paragraphs and flat lists. No marketing superlatives and no unverifiable claims.

Never include:

- UI walkthroughs, click paths, or route paths.
- Status enum names, queue or command names, daemon or worker names, job or function identifiers.
- SQL tables, columns, or schemas.
- Cloud instance families, regions, or account identifiers, unless the user must choose them.
- Configuration key names or file contents.
- Internal service or module names.

## Formatting

Defer to the `markdown` skill for bold usage, headers, list spacing, and colon casing. Additional rules:

- Page and UI names in plain Title Case, not italics and not code formatting.
- Route paths in backticks only when the reader must type or navigate to them.
- Italics only for scientific names, such as _Quercus robur_.
- No decorative emphasis on product or team names.
- Run `make format-docs` after any edit under `docs/` or `.agents/`.

## Directory Pattern

- `docs/features/README.md`: capability index, required once more than one domain page exists.
- `docs/features/<domain>/README.md`: the capability page when the domain is a single capability, or the hub when it has sub-features.
- `docs/features/<domain>/<sub-feature>/README.md`: for capabilities a reader evaluates separately. One level only.
- Directory names are lower-case kebab-case.

## Workflow

1. Identify the domain and whether it contains sub-features. Split when capabilities have distinct outcomes, prerequisites, configuration, or operational concerns, even if one supports another. Do not nest deeper than one level.
2. Write the decision summary first. If it cannot be written, the capability is not ready to document.
3. Map requirements and availability, separating tenant configuration from infrastructure configuration.
4. Map the program lifecycle: what the organisation prepares, what runs automatically, where a person decides, and what remains afterwards.
5. Verify every cost, capacity, and limitation claim against code or operator documentation.
6. Write the body, then move any configuration or debugging detail down to the operator reference.
7. Check the page against the prohibited content list and the five-minute limit.
8. Update the capability index row if the index exists.
9. Run `make format-docs`.

## Out of Scope

This skill does not produce the README or portfolio narrative, architecture documentation, ADRs, tenant onboarding or help-centre material, trust, security, or legal material, operator references, volunteer how-to guides, training material, or adoption briefs and PDFs. Link to those artifacts instead of duplicating them.
