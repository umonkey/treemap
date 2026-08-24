# Tree Damage Alerts & Curator Notifications

The application features an integrated Telegram bot (`services/chatbot`) that enables citizens and activists to report tree damage, health anomalies, or illegal logging directly from their mobile devices. Once a report is complete, it is automatically dispatched to project curators via private Telegram messages and visualized on the main map.

## Overview of the Alert Lifecycle

1. **Submission via Telegram Bot**:
   - Users interact with the Telegram bot to submit reports.
   - A reporting session allows users to send close-up and wide photos, precise GPS location, and a text description.
   - Multiple messages sent within a 10-minute window are intelligently grouped into the active alert session.

2. **Map Integration & Retention**:
   - Damage alerts are stored permanently in the database for historical tracking.
   - To maintain map clarity and relevance, alerts submitted within the last 7 days are rendered as red circles on the active web map.

3. **Curator Dispatcher Workflow**:
   - A decoupled background worker daemon (`chatbot dispatch-alerts`, managed via supervisor) runs periodically in the background.
   - **Eligibility Criteria**: An alert is dispatched to curators only when it meets all of the following requirements:
     - **Age**: Created at least 10 minutes ago (giving users time to finish uploading photos, location, and description).
     - **Photos**: Contains at least one uploaded photo.
     - **Location**: Contains valid latitude and longitude coordinates.
     - **Description**: Contains a non-empty text description.
     - **Status**: Has not been previously reported (`reported_at` is null).

4. **Curator Notification Delivery**:
   - Eligible alerts trigger private Telegram messages sent to all chat IDs configured in `REPORT_RECIPIENTS`.
   - Message format:
     ```
     New report available:

     https://yerevan.treemaps.app/alert/:id

     <description text>
     ```
   - **Reliability & Retry Semantics**: If message delivery fails for any recipient ID, the error is logged and dispatch continues to other recipients. An alert is marked as successfully reported (`reported_at = unixepoch()`) **only if** delivery succeeds for all configured recipients. If any recipient fails, the alert remains unreported and will be automatically retried on subsequent background polling cycles.

---

## Configuration & Deployment

The chatbot and its background dispatcher daemon are configured using file-based secrets and environment variables:

- **`CHATBOT_TOKEN`**: API token for the Telegram bot (obtained via BotFather).
- **`CHATBOT_DATABASE`**: Path to the shared SQLite database file.
- **`REPORT_RECIPIENTS`**: Comma-separated list of Telegram chat IDs (moderators/curators) to receive private report notifications. _(Note: Recipients must have previously initiated a chat with the bot by messaging `/start`)._
- **`WEBSITE_URL`**: Base URL for alert links in notifications (defaults to `http://localhost:5173` in development; configured as `https://yerevan.treemaps.app` in production).
- **`FILES_BUCKET`, `FILES_REGION`, `FILES_ENDPOINT`, `FILES_KEY`, `FILES_SECRET`**: Object storage credentials for uploaded alert media.
- **`RUST_LOG`**: Logging level configuration (e.g., `info,chatbot=debug`).

### Production Supervision

In production deployments (`compose.prod.yaml`), two separate processes run inside the chatbot container via Supervisor:

1. **`chatbot`**: Runs the interactive Telegram bot message REPL (`serve`).
2. **`dispatch-alerts`**: Runs the background alert dispatcher daemon (`dispatch-alerts`).
