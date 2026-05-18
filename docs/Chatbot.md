# Telegram Chatbot

The application includes a Telegram chatbot that allows users to send tree damage alerts directly from their mobile devices.

- Tree damage alerting: users can send alerts including photos and location data.

- Map integration: alerts are visualized as red circles on the main map.

- Data retention: all alerts are stored permanently in the database for historical tracking.

- Display logic: to maintain map clarity and relevance, only alerts from the last 7 days are shown on the active map.

## Configuration

The chatbot is configured using the following environment variables:

- `CHATBOT_TOKEN`: the API token for the Telegram bot. This token needs to be found with the botfather.
- `CHATBOT_DATABASE`: path to the SQLite database file.
- `FILES_BUCKET`: name of the storage bucket for uploaded media.
- `FILES_REGION`: region for the file storage service.
- `FILES_ENDPOINT`: API endpoint for the file storage service.
- `FILES_KEY`: access key for file storage.
- `FILES_SECRET`: secret key for file storage.
- `RUST_LOG`: logging level.

## Setup

To get started with the chatbot:

1. Create a new bot by messaging [@BotFather](https://t.me/botfather) on Telegram.
2. Follow the prompts to name your bot and choose a username.
3. BotFather will provide a `CHATBOT_TOKEN`. Add this token to your environment configuration.
