# Container setup

This folder contains files to create a self-contained Docker image which runs all parts of the application.
This is enough for running an instance of the application for a small local community.
It also serves as an example of how things can be set up.

This container runs the following components:

- A single web process, which handles both the API and HTML pages, enriched with the metadata.
- A single queue consumer process, which handles the background tasks.  (The queue is based on SQLite.)
- A cron daemon, which runs periodic tasks, like OSM integration and logrotate.

For a busy system, you will likely want to split this into individual containers.
