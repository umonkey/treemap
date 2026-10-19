---
name: feature-docs
description: Audience, structure, and content rules for platform capability documentation under docs/features/<domain>/, written for the adopting organisation's technical decision-maker and the staff who operate the deployment.
---

# Feature Documentation

Generate and maintain capability documentation organized by domain under `docs/features/<domain>/`.

## Audience

Write for two readers:

- The technical decision-maker at the adopting organisation: CTO, IT manager, digital services lead, GIS coordinator, or NGO technical lead. They decide whether the platform is safe, affordable, and possible to exit.
- The staff who will deploy and operate it. They need to know what a program costs, requires, and produces.

Do not write for:

- Field volunteers and mappers. They are served by in-app help and training sessions, not repository docs.
- The program lead evaluating whether the initiative is worth funding. That brief lives on the landing site, not in this repository.
- Developers. Architecture, setup, and internals belong in `docs/` and service-level `AGENTS.md` files.

The test for every paragraph: does it help someone decide whether to adopt the platform, or run it afterwards? If neither, cut it.

## What the Reader Needs

Cover these concerns, in this order of importance:

- Fit: what the capability does and which problems it solves.
- Commitment: people, skills, hardware, hosting, and data required.
- Cost: compute, storage, and staff effort over years, not just monthly hosting.
- Risk: accuracy, limitations, failure modes, and what happens when something breaks.
- Ownership: who holds the data, which formats it leaves in, and how to migrate away.
- Integration: API, exports, and external systems such as OSM or identity providers.

## Document Structure

Every domain document follows this structure. Omit a section only when it is genuinely inapplicable, and say why if the omission is surprising.

1. Decision summary: three to six bullets covering fit, cost, risk, and exit. A decision-maker reads this and stops; staff read on.
2. What it does: the capability and the outcome it produces, without implementation detail.
3. What you need to run it: people and roles, skills, hardware, hosting, and upstream data or state.
4. How a program runs: the end-to-end lifecycle at program level, from preparation through routine operation, including which steps are automatic and which need a person.
5. What the organisation gets: data, exports, publications, and audit history.
6. Cost and effort: compute and storage with realistic units, plus ongoing staff time.
7. Operations: what administrators monitor, which notifications arrive, and where backup, restore, and upgrade procedures are documented.
8. Integration, ownership, and exit: API surface, export formats, external systems, and portability.
9. Limitations and risks: quantified where possible.
10. Questions organisations usually ask: the recurring concerns of the target readers, answered plainly.

When a domain has more than one document, add `docs/features/README.md` as the platform-level assessment page for the decision-maker: architecture, hosting footprint, security and data handling, integration, support model, and total cost. Link each domain document from it.

## Writing Rules

- Lead with outcomes, then mechanics. Explain what the organisation can do, not how the code does it.
- Quantify. Costs in currency per unit, accuracy in metres, capacity in trees or sequences, effort in person-days.
- State limitations plainly. A reader who discovers a limitation after adoption is a lost reference.
- Use the reader's vocabulary: deployment, program, survey, tree, panorama, species, role, permission. Avoid internal names.
- Link to operator documentation for configuration keys, CLI commands, schemas, and internals. Never inline them.
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

- `docs/features/README.md`: platform-level assessment page, required once a second domain document exists.
- `docs/features/<domain>/README.md`: the domain document when it is a single capability.
- `docs/features/<domain>/<sub-feature>/README.md`: only when a domain has several distinct sub-capabilities.
- Directory names are lower-case kebab-case.

## Workflow

1. Identify the target domain and whether it is a single capability or several sub-capabilities.
2. Map the program lifecycle: what the organisation prepares, what runs automatically, where a person decides, and what remains afterwards.
3. Identify what a deployment must configure and which roles or permissions are needed, then describe them in plain language and link to the operator documentation.
4. Write the decision summary first. If it cannot be written, the capability is not ready to document.
5. Verify every cost, capacity, and limitation claim against code or operator documentation.
6. Write the body, then check it against the prohibited content list.
7. Run `make format-docs`.

## Out of Scope

This skill does not produce volunteer how-to guides, training material, adoption briefs or PDFs, operator runbooks, developer or architecture documentation, or ADRs. Link to those artifacts instead of duplicating them.
