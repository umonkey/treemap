# Telegram Chatbot

The application includes a Telegram chatbot that allows users to report tree damage directly from their mobile devices.

## Overview

The chatbot serves as an intake mechanism for community-driven tree health monitoring. Reports submitted via the bot are automatically integrated into the main mapping application.

## Features

- Tree damage reporting: users can send reports including photos and location data.
- Map integration: reports are visualized as red circles on the main map.
- Data retention: all reports are stored permanently in the database for historical tracking.
- Display logic: to maintain map clarity and relevance, only reports from the last 7 days are shown on the active map.

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
