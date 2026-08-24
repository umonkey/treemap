# Citizen Feedback

The application features an integrated Telegram bot (`services/chatbot`) that enables citizens and activists to report remarkable observations, such as newly planted trees, wildlife sightings, health anomalies, or maintenance needs directly from their mobile devices. Once a report is complete, it is automatically dispatched to project curators and visualized on the main map.

## Overview of the Citizen Feedback Lifecycle

1. Submission via Telegram bot:
   - users interact with the Telegram bot to submit reports.
   - a reporting session allows users to send close-up and wide photos, precise GPS location, and a text description.
   - multiple messages sent within a 10-minute window are intelligently grouped into the active feedback session.

2. Map integration and retention:
   - citizen feedback reports are stored permanently in the database for historical tracking.
   - to maintain map clarity and relevance, reports submitted within the last 7 days are rendered as markers on the active web map.

3. Curator dispatcher workflow:
   - a decoupled background worker daemon (`chatbot dispatch-alerts`, managed via supervisor) runs periodically in the background.
   - eligibility criteria: a report is dispatched to curators only when it meets all of the following requirements:
     - age: created at least 10 minutes ago (giving users time to finish uploading photos, location, and description).
     - photos: contains at least one uploaded photo.
     - location: contains valid latitude and longitude coordinates.
     - description: contains a non-empty text description.
     - status: has not been previously reported (`reported_at` is null).

4. Curator notification delivery:
   - eligible reports trigger private notifications sent to all configured recipients.
   - Telegram delivery is provided as an initial notification channel and is extensible to other channels such as email or LLM processing pipelines.
   - message format:
     ```
     New report available:

     https://yerevan.treemaps.app/alert/:id

     <description text>
     ```
   - reliability and retry semantics: if message delivery fails for any recipient ID, the error is logged and dispatch continues to other recipients. A report is marked as successfully reported (`reported_at = unixepoch()`) only if delivery succeeds for all configured recipients. If any recipient fails, the report remains unreported and will be automatically retried on subsequent background polling cycles.

---

## Configuration and Deployment

The chatbot and its background dispatcher daemon are configured using file-based secrets and environment variables:

- `CHATBOT_TOKEN`: api token for the Telegram bot (obtained via BotFather).
- `CHATBOT_DATABASE`: path to the shared SQLite database file.
- `REPORT_RECIPIENTS`: comma-separated list of Telegram chat IDs (moderators/curators) to receive private report notifications. (Note: Recipients must have previously initiated a chat with the bot by messaging `/start`).
- `WEBSITE_URL`: base URL for report links in notifications (defaults to `http://localhost:5173` in development; configured as `https://yerevan.treemaps.app` in production).
- `FILES_BUCKET`, `FILES_REGION`, `FILES_ENDPOINT`, `FILES_KEY`, `FILES_SECRET`: object storage credentials for uploaded report media.
- `RUST_LOG`: logging level configuration (e.g., `info,chatbot=debug`).

### Production Supervision

In production deployments (`compose.prod.yaml`), two separate processes run inside the chatbot container via Supervisor:

1. `chatbot`: runs the interactive Telegram bot message REPL (`serve`).
2. `dispatch-alerts`: runs the background report dispatcher daemon (`dispatch-alerts`).
