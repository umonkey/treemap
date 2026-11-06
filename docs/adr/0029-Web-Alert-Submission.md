# ADR 0029: Web Alert Submission

- Date: 2026-11-06
- Status: accepted

## Context

Citizen feedback alerts could only be submitted through the Telegram bot. The web map interface exposed the alert layer and preview page but no way to create an alert, so signed-in users who do not use Telegram could not report observations. We need submission parity.

The alert table is shared: `chatbot_alerts` is written and read by both the backend and the chatbot, and its `chat_id` column was `NOT NULL`. A web alert has no Telegram chat, so the notification code that relies on a chat id has to account for a missing value.

## Decision

We add `POST /v1/alerts` behind the `alert:create` permission and let signed-in users submit a location, description, and up to ten uploaded photos from the map.

Key characteristics:

- Web alerts are created directly with `status = 'new'`; there is no draft phase, and `chat_id` is `NULL`.
- `chat_id` becomes nullable through migration `024-alert-web-submission.sql`, which rebuilds `chatbot_alerts`. `NULL` means "no Telegram chat".
- The `AlertRead` DTO exposes `source` (`web` or `telegram`) and never exposes the chat id. The preview page shows the Telegram bot link only for Telegram alerts.
- The pinger skips alerts with a `NULL` chat; `dispatch-alerts` sends to configured recipients and does not depend on the chat id.
- The request validates a trimmed description of at most 2,000 characters, finite coordinates, 1 to 10 file ids, and that every upload is complete and owned by the caller.
- One `AddAlertPhoto` queue message is emitted per file. The handler writes `alerts/<alert_id>/<upload_file_id>.jpg` and inserts a `chatbot_alerts_photos` row, so retries are idempotent.
- Photos are decoded, EXIF-oriented, bounded to 2,000 pixels on the longest side, and always re-encoded as JPEG before they are stored in the public files bucket. The source upload is kept.
- `alert:create` is granted to `admin`, `editor`, and `user` by migration `025-alert-create.sql`.

Alternatives considered:

- `chat_id = 0` sentinel: cheaper than a table rebuild, but it creates an invisible invariant that leaks into every consumer and future query.
- Separate table for web alerts: avoids the nullable column, but duplicates the lifecycle, dispatcher, photo, and preview code paths.
- Synchronous photo processing in the request: simpler to reason about, but makes the request as slow as decoding and re-encoding every photo and loses per-photo retry isolation.
- Copying upload bytes verbatim instead of transcoding: faster, but a PNG or HEIC payload could be stored under a `.jpg` key and non-images would not be rejected.

## Consequences

- Migration 024 must run before the backend is deployed, and the nullable-aware chatbot must be deployed before web alerts are dispatched.
- Migration 025 must run before the web entry point is announced, otherwise non-admin users receive 403 responses and do not see the menu item.
- The shared alert model and the pinger now handle a missing chat id, which any future notification or reply feature must also handle.
- Alert photos are public, like Telegram alert photos, and the source upload is retained, which increases storage use.
- The preview can briefly show no photos while queue processing runs; the dispatcher may notify curators before photos are attached and already tolerates an empty photo list.
- There is no rate limiting on the endpoint; abuse requires a follow-up throttle.
- HEIC and HEIF uploads may fail server-side decoding and be dropped with a log entry.
