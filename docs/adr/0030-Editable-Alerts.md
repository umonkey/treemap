# ADR 0030: Editable Alerts

- Date: 2026-11-06
- Status: accepted

## Context

Citizen feedback alerts could be created from the web map or the Telegram bot, but a submitted description was final. A reporter who mistyped text, or a curator who needed to clarify a report, had no way to correct it. The web preview also rendered a Telegram bot link for Telegram-origin alerts; that note is removed in the same change, so the preview needs an action for viewers who are allowed to edit.

The alert table is shared with the chatbot, and `created_by` has different meanings by channel. For web alerts it is a platform user id. For Telegram alerts it is a Telegram user id, which is unrelated to platform users and must not be compared with the authenticated user id. Photos are separate records written by a queue handler and are not addressed by this change.

## Decision

We add `editable` to `AlertRead` and a `PATCH /v1/alerts/{id}` endpoint that replaces an alert's description. The permission rule is `(creator && alert:create) || alert:manage`:

- creator: the alert was created through the web interface, its `created_by` equals the requesting user id, and the user still holds `alert:create`;
- manager: the requesting user holds `alert:manage`, including through the `admin` role, which bypasses permission checks at runtime.

Key characteristics:

- `GET /v1/alerts/{id}` computes `editable` for the requesting user. Anonymous requests receive `false`, so a client must send the `Authorization` header to receive `true`.
- `PATCH` accepts `{"description": "..."}`, applies the creation-time trim, rejects empty text and text longer than 2,000 characters, and returns `200` with the updated `AlertRead`. It returns `401` without a token, `403` when the user may not edit the alert, and `404` when the alert does not exist.
- Ownership is restricted to web alerts because `created_by` on a Telegram alert is a Telegram user id. A coincidental match with a platform user id must not grant edit rights. Holders of `alert:manage` can edit Telegram alerts.
- Migration `026-alert-manage.sql` adds `alert:manage` and grants it to `admin`. Admins already bypass checks at runtime, so the grant does not change enforcement; it makes `/v1/me` report the permission and allows later grants to other roles.
- The preview shows an Edit action only when `editable` is true and links to a new edit page that mirrors the add-alert panel: a multiline text area and Save, with no file controls.
- Only the description is mutable. Photos are separate queue-written records, and replacing them would need object cleanup, orphan handling, and notification semantics that a text correction does not justify. Coordinates, status, timestamps, and the channel are also untouched, and the schema has no audit columns for edits.
- Editing is allowed in any alert status.

## Alternatives considered

- Admin-only editing: simplest to enforce, but a reporter could not fix a typo without a curator, and every correction would depend on staff availability.
- A separate edit token or endpoint: a signed per-report link could allow edits outside an authenticated session, but it adds token issue, storage, expiry, and revocation concerns that the product does not need yet.
- Full CRUD in this change, including deletion and photo replacement: broader API, audit, and photo-lifecycle work that is not required to correct a description. Deferred to a later change.

## Consequences

- `OptionalUserId` now treats malformed, expired, or otherwise invalid bearer tokens as anonymous, returning user id 0. Without this, adding the authorization header to the public alert read would turn a stale browser token into a `401` on the preview. Other public reads that use the extractor inherit the lenient behaviour.
- `editable` adds two IAM reads per authenticated preview load. This is negligible compared with the existing alert and photo reads.
- A creator who loses `alert:create` loses edit rights to their own alerts immediately, which matches the requested rule. Holders of `alert:manage` can edit any alert, including Telegram alerts.
- There is no edit history. The schema has no `updated_at` or `updated_by` columns for alerts, so corrections are not auditable; a follow-up migration is needed if accountability is required.
- Photos remain immutable. A wrong photo still needs a separate workflow that does not exist in this release.
- The POST response reports `editable: true` because the creation permission was already proven. If the policy later gains status gating, that call site must be revisited.
- Migration 026 must be applied for `/v1/me` to report `alert:manage`; admin enforcement continues to work without it.
- An administrator can edit a Telegram draft, and the change does not refresh the draft's reminder timer. Restricting edits by status remains a possible follow-up.
