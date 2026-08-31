# Telegram Chatbot

The application includes a Telegram chatbot that allows users to send citizen feedback and remarkable sightings directly from their mobile devices.

- citizen feedback: users can send reports including photos, location data, and descriptions of remarkable sightings, new plantings, or maintenance needs.
- map integration: feedback reports are visualized on the main map.
- data retention: all reports are stored permanently in the database for historical tracking.
- display logic: to maintain map clarity and relevance, only reports from the last 7 days are shown on the active map.

## Configuration

The chatbot is configured using the following environment variables:

- `CHATBOT_TOKEN`: the API token for the Telegram bot (obtained via BotFather).
- `CHATBOT_DATABASE`: path to the SQLite database file.
- `FILES_BASE_URL`: base URL for media files.
- `FILES_BUCKET`: name of the storage bucket for uploaded media.
- `FILES_REGION`: region for the file storage service.
- `FILES_ENDPOINT`: API endpoint for the file storage service.
- `FILES_KEY`: access key for file storage.
- `FILES_SECRET`: secret key for file storage.
- `RUST_LOG`: logging level configuration.
- `REPORT_RECIPIENTS`: comma-separated list of Telegram chat IDs to receive notifications when a completed citizen feedback report is ready.
- `WEBSITE_URL`: base URL for report links in notifications.

## Feedback Dispatcher and Draft Pinger

The application runs decoupled background workers (managed via supervisord):

1. `dispatch-alerts`: periodically scans for pending citizen feedback reports with status `new` and dispatches them to all chat IDs in `REPORT_RECIPIENTS`.
2. `dispatch-outbox`: delivers pending messages from the outbox queue with exponential backoff retries.
3. `dispatch-pings`: scans for incomplete draft reports and enqueues reminder messages into the outbox when `ping_at` is reached. Draft reports remain in `draft` status indefinitely until completed.

To get started with the chatbot:

1. Create a new bot by messaging [@BotFather](https://t.me/botfather) on Telegram.
2. Follow the prompts to name your bot and choose a username.
3. BotFather will provide a `CHATBOT_TOKEN`. Add this token to your environment configuration.
