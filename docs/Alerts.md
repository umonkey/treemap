# Citizen Feedback

The application collects citizen feedback reports from two channels: an integrated Telegram bot (`services/chatbot`) and the web map interface. Citizens and activists can report remarkable observations, such as newly planted trees, wildlife sightings, health anomalies, or maintenance needs. Once a report is complete, it is automatically dispatched to project curators and visualized on the main map.

## Overview of the Citizen Feedback Lifecycle

1. Submission via Telegram bot:
   - users interact with the Telegram bot to submit reports.
   - a reporting session allows users to send close-up and wide photos, precise GPS location, and a text description.
   - multiple messages sent within a 1-week window while in `draft` status accumulate into the active feedback session.
   - draft alerts track a `ping_at` timestamp (initially set to 10 minutes after creation or update). If incomplete, the `dispatch-pings` daemon enqueues reminder messages into `chatbot_outbox` for delivery via `dispatch-outbox`.
   - draft alerts remain in `draft` status indefinitely until the user actively provides all required fields.
   - once completed and transitioned to `new`, the alert is locked and further communication creates a new alert.

2. Submission via the web interface:
   - signed-in users with the `alert:create` permission submit a report from the map.
   - the report uses the current map centre as its location, requires a non-empty description, and accepts between 1 and 10 uploaded photos.
   - there is no draft phase: the alert is created with status `new` and `chat_id` `NULL` as soon as the request succeeds, and its photos are processed asynchronously.

3. Map integration and retention:
   - citizen feedback reports are stored permanently in the database for historical tracking.
   - to maintain map clarity and relevance, reports submitted within the last 7 days are rendered as markers on the active web map.

4. Curator dispatcher workflow:
   - a decoupled background worker daemon (`chatbot dispatch-alerts`, managed via supervisor) runs periodically in the background.
   - report status: reports start with status `draft` and accumulate subsequent photos, location updates, and descriptions only while in `draft` status within a 10-minute window; once complete, they automatically transition to `new` and are locked against further updates. Web submissions skip the draft phase and start as `new`.
   - eligibility criteria: a report is dispatched to curators when it meets all of the following requirements:
     - status: has status `new` (meaning it contains at least one uploaded photo, valid latitude and longitude coordinates, and a non-empty text description). Web submissions are validated before insertion and are dispatched with the same criteria.
     - reported: has not been previously reported (`reported_at` is null).

5. Curator notification delivery:
   - eligible reports trigger private notifications sent to all configured recipients.
   - Telegram delivery is provided as an initial notification channel and is extensible to other channels such as email or LLM processing pipelines.
   - message format:
     ```
     <description text>

     https://yerevan.treemaps.app/alert/:id/preview

     #alerts
     ```
   - reliability and retry semantics: if message delivery fails for any recipient ID, the error is logged and dispatch continues to other recipients. A report is marked as successfully reported (`reported_at = unixepoch()`) only if delivery succeeds for all configured recipients. If any recipient fails, the report remains unreported and will be automatically retried on subsequent background polling cycles.

---

## Web Submission

### Permission and entry point

Web submissions require the `alert:create` permission. Migration `025-alert-create.sql` grants it to the `admin`, `editor`, and `user` roles. Admins bypass permission checks at runtime, but the explicit admin grant is persisted so that `/v1/me` reports it. The map `(+)` menu shows the "Add alert" item only with this permission; the API enforces the same permission, so hiding the menu is not a security boundary.

### Submission page

The `/add/alert` page reuses the standard add panel. It shows the current map centre coordinates and the distance to the two nearest trees, and contains a multiline description, a horizontal queue of photo previews, a camera button, a gallery button on touch devices, and a Send button. Send stays disabled until the description is non-empty and at least one photo is queued. Files remain local until Send is pressed; the page then uploads them one by one through `/v1/upload`, collects the returned upload ids in order, and calls the create endpoint. Cancelling or leaving the page discards the queue.

### API contract

`POST /v1/alerts` is added to the existing `alert_router`, so it shares the `/v1/alerts` prefix, the `Cache-Control: no-store` header, and the transaction middleware with the read endpoints. The request body is:

```json
{
  "lat": 40.1811,
  "lon": 44.5136,
  "description": "Newly planted tree needs a support stake.",
  "files": ["1234567890123456789"]
}
```

Validation rules:

- the description is trimmed; an empty description or one longer than 2,000 characters is rejected;
- latitude and longitude must be finite and within ±90 and ±180 degrees respectively;
- between 1 and 10 file ids are required;
- duplicate file ids are removed while preserving order;
- every id must resolve to a completed upload owned by the authenticated user; a missing upload returns `FileNotFound`, an unfinished upload returns a bad-request error, and another user's upload is rejected as access denied.

The response is the created `AlertRead` (`id`, `lat`, `lon`, `created_at`, `description`, `source`). `source` is `web` when `chat_id` is `NULL` and `telegram` otherwise; the chat id itself is never exposed. Read endpoints `GET /v1/alerts/active`, `GET /v1/alerts/{id}`, and `GET /v1/alerts/{id}/photos` are unchanged.

### Alert record and notification semantics

A web submission is inserted with `status = 'new'`, `chat_id = NULL`, and `created_by` set to the authenticated user. The insert uses `INSERT ... RETURNING *` so the generated id is read without relying on `last_insert_rowid()`, which is unsafe across pooled connections outside a transaction. `message_id`, `username`, `language_code`, `response_text`, `responded_at`, `reported_at`, and `ping_at` stay `NULL`.

`chat_id` is nullable after migration `024-alert-web-submission.sql`; `NULL` means the alert has no Telegram chat. The draft pinger filters such alerts out of `get_pending_pings` and defensively skips any it receives. `dispatch-alerts` does not read `chat_id`: it sends to the configured recipients, so web alerts are dispatched like any other new alert. The preview page renders the `@kanach_yerevan_bot` link only for Telegram submissions; web submissions omit it.

### Photo processing

Each submitted file id produces one `AddAlertPhoto` queue message:

```json
{
  "command": "AddAlertPhoto",
  "params": { "alert_id": 42, "file_id": 1234567890123456789 }
}
```

For each message the handler:

1. verifies that the alert exists; a missing alert is treated as a retryable error, which covers the race where the queue message becomes visible before the request transaction commits;
2. builds the deterministic path `alerts/<alert_id>/<upload_file_id>.jpg` and returns early if a `chatbot_alerts_photos` row already exists for that path;
3. reads the uploaded source object, decodes it, applies EXIF orientation, bounds the longest side to 2,000 pixels, and always re-encodes it as JPEG;
4. writes the JPEG to the public files bucket and inserts the `chatbot_alerts_photos` row.

Re-encoding guarantees that a `.jpg` key contains JPEG even when the source upload was PNG, WebP, or another accepted format, and it rejects non-image payloads. The deterministic path makes retries idempotent: a retry overwrites the same key instead of creating a duplicate row. A file that cannot be decoded is logged and dropped from the queue after the first failure; transient storage or database errors are retried. The original upload record and object are kept, so the same upload id can be attached to more than one alert at the cost of extra storage.

Because photo processing is asynchronous, the preview at `/alert/<id>/preview` can load before photos exist. They appear after the queue handler completes, and refreshing the preview shows them. `dispatch-alerts` may also notify curators before all photos are attached; the dispatcher already tolerates an empty photo list.

### Deployment

Apply migration `024-alert-web-submission.sql` first, then deploy the backend that writes `NULL` chat ids, then apply `025-alert-create.sql` before announcing the web entry point. Migration 024 must run before the backend because the previous schema declares `chat_id` as `NOT NULL`. Migration 025 must run before users can submit: without the grant, every non-admin request returns 403 and the menu item stays hidden. Deploy the chatbot build that reads a nullable chat id before web alerts are dispatched, since the dispatcher maps rows from the shared table.

Migration 024 rebuilds `chatbot_alerts` while `chatbot_alerts_photos.alert_id` references it with `ON DELETE CASCADE`. Apply the whole file as one transaction on a connection where foreign-key enforcement is off, which is the application default. `PRAGMA defer_foreign_keys` defers constraint checks but does not suppress `ON DELETE CASCADE`; if your migration client enables `PRAGMA foreign_keys`, disable it for the rebuild or back up `chatbot_alerts_photos` first. Take a database backup and rehearse on a copy that contains alerts with photos: confirm that `chatbot_alerts_photos` rows survive (ids are copied unchanged) and that `GET /v1/alerts/{id}/photos` returns the same paths.

---

## Configuration and Deployment

The chatbot and its background dispatcher daemon are configured using file-based secrets and environment variables:

- `CHATBOT_TOKEN`: api token for the Telegram bot (obtained via BotFather).
- `CHATBOT_DATABASE`: path to the shared SQLite database file.
- `REPORT_RECIPIENTS`: comma-separated list of Telegram chat IDs (moderators/curators) to receive private report notifications. Each entry may optionally target a forum topic using the `chat_id:topic_id` format. (Note: Recipients must have previously initiated a chat with the bot by messaging `/start`).
- `WEBSITE_URL`: base URL for report links in notifications (defaults to `http://localhost:5173` in development; configured as `https://yerevan.treemaps.app` in production).
- `FILES_BUCKET`, `FILES_REGION`, `FILES_ENDPOINT`, `FILES_KEY`, `FILES_SECRET`: object storage credentials for uploaded report media.
- `RUST_LOG`: logging level configuration (e.g., `info,chatbot=debug`).

### Production Supervision

In production deployments (`compose.prod.yaml`), separate processes run inside the chatbot container via Supervisor:

- `chatbot`: runs the interactive Telegram bot message REPL (`serve`).
- `dispatch-alerts`: runs the background report dispatcher daemon (`dispatch-alerts`).
- `dispatch-outbox`: runs the outbox queue worker daemon (`dispatch-outbox`).
- `dispatch-pings`: runs the background draft alert pinger daemon (`dispatch-pings`).
